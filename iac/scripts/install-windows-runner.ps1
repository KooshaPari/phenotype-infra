<#
.SYNOPSIS
  Install Forgejo act_runner on Windows 11 (AMD64) as a dedicated service.

.DESCRIPTION
  Provisions a `forgejo-runner` local user, downloads the latest act_runner.exe
  release from gitea.com, writes a starter config.yaml wired for Tailscale
  MagicDNS (http://forgejo.ts.internal:3000), and installs a Windows service
  in daemon mode. Registration with the Forgejo instance is left as a manual
  follow-up step because the OCI-hosted Forgejo is not yet live.

  A companion "gaming-mode" scheduled task pauses the service whenever a
  sentinel file exists (%USERPROFILE%\.phenotype\runner-paused) or when
  Parsec (parsecd.exe) is running. This keeps the runner from stealing
  cycles during remote-desktop / game-streaming sessions.

.PARAMETER InstallDir
  Install root. Default: C:\forgejo-runner

.PARAMETER ForgejoUrl
  Forgejo instance URL to bake into config.yaml.
  Default: http://forgejo.ts.internal:3000

.PARAMETER Labels
  Runner labels. Default: self-hosted,windows,amd64,desktop

.NOTES
  Run in an ELEVATED PowerShell session on kooshapari-desk.tail2b570.ts.net.
  Requires: Tailscale installed + connected, Windows 11 AMD64, admin rights.

.EXAMPLE
  PS> Set-ExecutionPolicy -Scope Process Bypass -Force
  PS> .\install-windows-runner.ps1
#>

[CmdletBinding()]
param(
  [string]$InstallDir  = 'C:\forgejo-runner',
  [string]$ForgejoUrl  = 'http://forgejo.ts.internal:3000',
  [string]$Labels      = 'self-hosted,windows,amd64,desktop',
  [string]$RunnerUser  = 'forgejo-runner',
  [string]$ServiceName = 'ForgejoRunner'
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

function Write-Step { param($msg) Write-Host "==> $msg" -ForegroundColor Cyan }
function Write-Ok   { param($msg) Write-Host "  ok  $msg" -ForegroundColor Green }
function Write-Warn2{ param($msg) Write-Host "  !!  $msg" -ForegroundColor Yellow }
function Fail       { param($msg) Write-Host "  xx  $msg" -ForegroundColor Red; throw $msg }

# ---------------------------------------------------------------------------
# 1. Prerequisites
# ---------------------------------------------------------------------------
Write-Step 'Checking prerequisites'

# Admin check
$principal = New-Object Security.Principal.WindowsPrincipal(
  [Security.Principal.WindowsIdentity]::GetCurrent())
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
  Fail 'Must run in an elevated (Administrator) PowerShell session.'
}
Write-Ok 'Running as Administrator'

# OS + arch
$os = Get-CimInstance Win32_OperatingSystem
if ($os.Caption -notmatch 'Windows 11') {
  Write-Warn2 "OS caption: $($os.Caption) — expected Windows 11; continuing anyway"
} else { Write-Ok "OS: $($os.Caption)" }

if ($env:PROCESSOR_ARCHITECTURE -ne 'AMD64') {
  Fail "Unsupported arch: $($env:PROCESSOR_ARCHITECTURE). This script targets AMD64."
}
Write-Ok 'Architecture: AMD64'

# Tailscale installed + up
$tailscaleExe = 'C:\Program Files\Tailscale\tailscale.exe'
if (-not (Test-Path $tailscaleExe)) {
  Fail "Tailscale not found at $tailscaleExe. Install from https://tailscale.com/download/windows first."
}
try {
  $tsStatus = & $tailscaleExe status --json 2>$null | ConvertFrom-Json
  if ($tsStatus.BackendState -ne 'Running') {
    Fail "Tailscale BackendState=$($tsStatus.BackendState); run 'tailscale up' first."
  }
  Write-Ok "Tailscale up (Self=$($tsStatus.Self.HostName))"
} catch {
  Fail "Tailscale status check failed: $_"
}

# ---------------------------------------------------------------------------
# 2. Dedicated local user
# ---------------------------------------------------------------------------
Write-Step "Provisioning local user '$RunnerUser'"

$existing = Get-LocalUser -Name $RunnerUser -ErrorAction SilentlyContinue
if ($null -ne $existing) {
  Write-Warn2 "User '$RunnerUser' already exists — skipping creation"
  $generatedPassword = $null
} else {
  # Generate a 32-char password from a broad character pool.
  Add-Type -AssemblyName System.Web
  $generatedPassword = [System.Web.Security.Membership]::GeneratePassword(32, 6)
  $secure = ConvertTo-SecureString $generatedPassword -AsPlainText -Force

  New-LocalUser -Name $RunnerUser `
                -Password $secure `
                -FullName 'Forgejo Runner Service Account' `
                -Description 'Dedicated svc account for act_runner (Phenotype infra)' `
                -PasswordNeverExpires `
                -UserMayNotChangePassword | Out-Null
  Add-LocalGroupMember -Group 'Users' -Member $RunnerUser -ErrorAction SilentlyContinue
  Write-Ok "Created user '$RunnerUser'"
}

# ---------------------------------------------------------------------------
# 3. Download latest act_runner.exe (Windows AMD64)
# ---------------------------------------------------------------------------
Write-Step 'Resolving latest act_runner release from gitea.com'

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$apiUrl = 'https://gitea.com/api/v1/repos/gitea/act_runner/releases?limit=1'
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
$release = Invoke-RestMethod -Uri $apiUrl -Headers @{ 'User-Agent' = 'phenotype-installer' }
if (-not $release -or $release.Count -eq 0) { Fail 'Could not query act_runner releases.' }

$rel = $release[0]
$asset = $rel.assets | Where-Object { $_.name -match 'windows.*amd64\.exe$' } | Select-Object -First 1
if (-not $asset) {
  # Fallback: construct predictable URL
  $version = $rel.tag_name.TrimStart('v')
  $dlUrl   = "https://gitea.com/gitea/act_runner/releases/download/$($rel.tag_name)/act_runner-$version-windows-amd64.exe"
  Write-Warn2 "No asset match in API; falling back to $dlUrl"
} else {
  $dlUrl = $asset.browser_download_url
}

$exePath = Join-Path $InstallDir 'act_runner.exe'
Write-Step "Downloading $dlUrl"
Invoke-WebRequest -Uri $dlUrl -OutFile $exePath -UseBasicParsing
if (-not (Test-Path $exePath)) { Fail 'Download failed.' }
Unblock-File -Path $exePath
Write-Ok "act_runner.exe placed at $exePath"

# ---------------------------------------------------------------------------
# 4. Starter config.yaml
# ---------------------------------------------------------------------------
Write-Step 'Generating starter config.yaml'

$configPath = Join-Path $InstallDir 'config.yaml'
$cacheDir   = Join-Path $InstallDir 'cache'
$logDir     = Join-Path $InstallDir 'logs'
New-Item -ItemType Directory -Force -Path $cacheDir, $logDir | Out-Null

$configYaml = @"
# Phenotype Forgejo runner — generated by install-windows-runner.ps1
# Instance: $ForgejoUrl (Tailscale MagicDNS, OCI-hosted once bootstrapped)
log:
  level: info

runner:
  file: .runner
  capacity: 2
  envs:
    TZ: UTC
  labels:
    - $($Labels -replace ',', "`n    - ")
  fetch_timeout: 5s
  fetch_interval: 2s

cache:
  enabled: true
  dir: $($cacheDir -replace '\\','/')
  host: ''
  port: 0

container:
  # Windows runners run host-native; keep containers disabled unless WSL2 is wired.
  network: ''
  privileged: false
  options: ''
  workdir_parent: ''
  valid_volumes: []
  docker_host: -
  force_pull: false

host:
  workdir_parent: $($InstallDir -replace '\\','/')/work
"@
Set-Content -Path $configPath -Value $configYaml -Encoding UTF8
Write-Ok "Wrote $configPath"

# ---------------------------------------------------------------------------
# 5. ACL: grant runner user access to install dir
# ---------------------------------------------------------------------------
Write-Step 'Setting ACLs on install dir'
$acl = Get-Acl $InstallDir
$rule = New-Object System.Security.AccessControl.FileSystemAccessRule(
  $RunnerUser, 'Modify', 'ContainerInherit,ObjectInherit', 'None', 'Allow')
$acl.SetAccessRule($rule)
Set-Acl -Path $InstallDir -AclObject $acl
Write-Ok "Granted Modify to $RunnerUser on $InstallDir"

# ---------------------------------------------------------------------------
# 6. Install Windows service (daemon mode)
# ---------------------------------------------------------------------------
Write-Step "Installing service '$ServiceName'"

if (Get-Service -Name $ServiceName -ErrorAction SilentlyContinue) {
  Write-Warn2 "Service '$ServiceName' already exists — stopping + removing to reinstall"
  Stop-Service -Name $ServiceName -Force -ErrorAction SilentlyContinue
  sc.exe delete $ServiceName | Out-Null
  Start-Sleep -Seconds 2
}

$binPath = "`"$exePath`" daemon --config `"$configPath`""
# Note: the service account needs the password; for an existing user the caller
# must re-run with a known password (or reset via Set-LocalUser + --passwd to
# sc.exe config). For the fresh-install case we use the generated password.
if ($generatedPassword) {
  sc.exe create $ServiceName binPath= $binPath `
    start= auto `
    obj= ".\$RunnerUser" `
    password= $generatedPassword `
    DisplayName= 'Forgejo Runner (act_runner)' | Out-Null
} else {
  Write-Warn2 "User pre-existed; installing service under LocalSystem (rotate later via sc.exe config $ServiceName obj= .\$RunnerUser password= <pw>)"
  sc.exe create $ServiceName binPath= $binPath `
    start= auto `
    DisplayName= 'Forgejo Runner (act_runner)' | Out-Null
}
sc.exe description $ServiceName 'Forgejo act_runner daemon (Phenotype desktop runner).' | Out-Null
sc.exe failure $ServiceName reset= 86400 actions= restart/5000/restart/15000/restart/60000 | Out-Null
Write-Ok "Service '$ServiceName' installed (NOT started — registration pending)"

# ---------------------------------------------------------------------------
# 7. Gaming-mode pause hook (scheduled task)
# ---------------------------------------------------------------------------
Write-Step 'Installing gaming-mode pause scheduled task'

$pauseScript = Join-Path $InstallDir 'gaming-mode-pause.ps1'
$sentinelDir = Join-Path $env:USERPROFILE '.phenotype'
$sentinel    = Join-Path $sentinelDir 'runner-paused'
New-Item -ItemType Directory -Force -Path $sentinelDir | Out-Null

$pauseBody = @"
# Auto-generated pause hook — pauses '$ServiceName' when Parsec is running
# or when sentinel '$sentinel' exists. Resumes otherwise.
`$svc = Get-Service -Name '$ServiceName' -ErrorAction SilentlyContinue
if (-not `$svc) { exit 0 }

`$parsecRunning = [bool](Get-Process -Name parsecd -ErrorAction SilentlyContinue)
`$sentinelHit   = Test-Path '$sentinel'
`$shouldPause   = `$parsecRunning -or `$sentinelHit

if (`$shouldPause -and `$svc.Status -eq 'Running') {
  Stop-Service -Name '$ServiceName' -Force
  Add-Content -Path '$logDir\gaming-mode.log' -Value "`$(Get-Date -Format o) paused (parsec=`$parsecRunning sentinel=`$sentinelHit)"
} elseif (-not `$shouldPause -and `$svc.Status -ne 'Running') {
  try {
    Start-Service -Name '$ServiceName'
    Add-Content -Path '$logDir\gaming-mode.log' -Value "`$(Get-Date -Format o) resumed"
  } catch {
    Add-Content -Path '$logDir\gaming-mode.log' -Value "`$(Get-Date -Format o) resume-failed: `$_"
  }
}
"@
Set-Content -Path $pauseScript -Value $pauseBody -Encoding UTF8

$taskName = 'ForgejoRunner-GamingModePause'
if (Get-ScheduledTask -TaskName $taskName -ErrorAction SilentlyContinue) {
  Unregister-ScheduledTask -TaskName $taskName -Confirm:$false
}
$action = New-ScheduledTaskAction -Execute 'powershell.exe' `
  -Argument "-NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File `"$pauseScript`""
$trigger1 = New-ScheduledTaskTrigger -AtStartup
$trigger2 = New-ScheduledTaskTrigger -Once -At (Get-Date).AddMinutes(1) `
  -RepetitionInterval (New-TimeSpan -Minutes 1)
$principal2 = New-ScheduledTaskPrincipal -UserId 'SYSTEM' -LogonType ServiceAccount -RunLevel Highest
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries `
  -StartWhenAvailable -MultipleInstances IgnoreNew
Register-ScheduledTask -TaskName $taskName -Action $action `
  -Trigger @($trigger1, $trigger2) -Principal $principal2 -Settings $settings | Out-Null
Write-Ok "Scheduled task '$taskName' installed (runs every 60s)"

# ---------------------------------------------------------------------------
# 8. Final report + registration TODO
# ---------------------------------------------------------------------------
Write-Host ''
Write-Host '============================================================' -ForegroundColor Magenta
Write-Host ' Forgejo runner install complete — REGISTRATION PENDING' -ForegroundColor Magenta
Write-Host '============================================================' -ForegroundColor Magenta

if ($generatedPassword) {
  Write-Host ''
  Write-Host 'Service account password (shown ONCE — stash in Vaultwarden post-OCI):' -ForegroundColor Yellow
  Write-Host "  user: $RunnerUser" -ForegroundColor Yellow
  Write-Host "  pass: $generatedPassword" -ForegroundColor Yellow
  Write-Host ''
}

Write-Host 'Next steps (after Forgejo is up on OCI):' -ForegroundColor Cyan
Write-Host "  1. In Forgejo admin UI, create a runner registration token."
Write-Host "  2. From an elevated PS here, register:"
Write-Host "       cd '$InstallDir'"
Write-Host "       .\act_runner.exe register \`"
Write-Host "         --instance '$ForgejoUrl' \`"
Write-Host "         --token '<RUNNER_TOKEN>' \`"
Write-Host "         --labels '$Labels' \`"
Write-Host "         --no-interactive"
Write-Host "  3. Start-Service -Name '$ServiceName'"
Write-Host "  4. Get-Service '$ServiceName'  # expect Running"
Write-Host ''
Write-Host "Logs: $logDir" -ForegroundColor DarkGray
Write-Host "Config: $configPath" -ForegroundColor DarkGray
Write-Host "Pause sentinel: $sentinel (touch to force-pause)" -ForegroundColor DarkGray
