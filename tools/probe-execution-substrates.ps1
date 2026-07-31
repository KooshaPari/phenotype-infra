<#+
.SYNOPSIS
  Read-only capability probe for the local execution substrates.

.DESCRIPTION
  Detects the locally installed Podman, Apple Containers, and first-party WSL
  Containers command surfaces and records whether each can answer a version
  probe. The probe never starts a machine, creates a container, reads a secret,
  contacts a provider, or mutates runtime state. An unavailable engine is an
  evidence-bearing result, not a script failure: the same contract can run on
  Windows, Linux, and macOS hosts.
#>
[CmdletBinding()]
param(
    [switch] $Json,
    [ValidateRange(1, 30)]
    [int] $TimeoutSeconds = 5
)

$ErrorActionPreference = "Stop"

function Get-FirstCommand {
    param([Parameter(Mandatory)][string] $Name)

    Get-Command $Name -ErrorAction SilentlyContinue |
        Where-Object { $_.CommandType -in @("Application", "ExternalScript") } |
        Select-Object -First 1
}

function Clip-Output {
    param([AllowNull()][string] $Text)

    if ([string]::IsNullOrWhiteSpace($Text)) {
        return $null
    }

    # wsl.exe emits UTF-16 when called through redirected pipes on Windows;
    # remove NULs so the JSON contract stays portable and human-readable.
    $singleLine = ($Text -replace "`0", "" -replace "\s+", " ").Trim()
    if ($singleLine.Length -gt 400) {
        return $singleLine.Substring(0, 400) + "..."
    }

    return $singleLine
}

function Invoke-VersionProbe {
    param(
        [Parameter(Mandatory)] $Command,
        [Parameter(Mandatory)][string] $ArgumentLine,
        [Parameter(Mandatory)][int] $TimeoutMs
    )

    $path = if ($Command.Source) { $Command.Source } else { $Command.Path }
    $fileName = $path
    $arguments = $ArgumentLine

    # A .bat shim can start a WSL VM or another long-lived child process. Do
    # not invoke it from a bounded, read-only probe; its presence is still
    # useful evidence and the owning adapter can perform an explicit health
    # check under its own lifecycle policy.
    if ($path -match "\.(bat|cmd)$") {
        return [pscustomobject]@{
            status = "installed_unavailable"
            exit_code = $null
            version = $null
            detail = "command shim detected; invocation skipped by read-only probe"
        }
    }

    $startInfo = [System.Diagnostics.ProcessStartInfo]::new()
    $startInfo.FileName = $fileName
    $startInfo.Arguments = $arguments
    $startInfo.UseShellExecute = $false
    $startInfo.CreateNoWindow = $true
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true

    $process = [System.Diagnostics.Process]::new()
    $process.StartInfo = $startInfo

    try {
        if (-not $process.Start()) {
            return [pscustomobject]@{
                status = "error"
                exit_code = $null
                version = $null
                detail = "process did not start"
            }
        }

        $stdoutTask = $process.StandardOutput.ReadToEndAsync()
        $stderrTask = $process.StandardError.ReadToEndAsync()

        if (-not $process.WaitForExit($TimeoutMs)) {
            try { $process.Kill($true) } catch { try { $process.Kill() } catch {} }
            return [pscustomobject]@{
                status = "timeout"
                exit_code = $null
                version = $null
                detail = "version probe exceeded timeout"
            }
        }

        $process.WaitForExit()
        $stdout = $stdoutTask.Result
        $stderr = $stderrTask.Result
        $combined = Clip-Output (($stdout + " " + $stderr).Trim())
        $version = Clip-Output $stdout
        $status = if ($process.ExitCode -eq 0) { "available" } else { "installed_unavailable" }

        return [pscustomobject]@{
            status = $status
            exit_code = $process.ExitCode
            version = $version
            detail = $combined
        }
    }
    catch {
        return [pscustomobject]@{
            status = "error"
            exit_code = $null
            version = $null
            detail = Clip-Output $_.Exception.Message
        }
    }
    finally {
        $process.Dispose()
    }
}

function Add-SubstrateResult {
    param(
        [Parameter(Mandatory)][AllowEmptyCollection()][System.Collections.Generic.List[object]] $Results,
        [Parameter(Mandatory)][string] $Name,
        [Parameter(Mandatory)][string] $CommandName,
        [Parameter(Mandatory)][string] $ArgumentLine,
        [Parameter(Mandatory)][int] $TimeoutMs
    )

    $command = Get-FirstCommand $CommandName
    if (-not $command) {
        $Results.Add([pscustomobject][ordered]@{
            substrate = $Name
            status = "missing"
            command = $CommandName
            path = $null
            exit_code = $null
            version = $null
            detail = "command not found"
        })
        return
    }

    $path = if ($command.Source) { $command.Source } else { $command.Path }
    $probe = Invoke-VersionProbe -Command $command -ArgumentLine $ArgumentLine -TimeoutMs $TimeoutMs
    $Results.Add([pscustomobject][ordered]@{
        substrate = $Name
        status = $probe.status
        command = $CommandName
        path = $path
        exit_code = $probe.exit_code
        version = $probe.version
        detail = $probe.detail
    })
}

$observedUtc = [DateTime]::UtcNow.ToString("o")
$hostName = if ($env:COMPUTERNAME) { $env:COMPUTERNAME } elseif ($env:HOSTNAME) { $env:HOSTNAME } else { "unknown" }
$results = [System.Collections.Generic.List[object]]::new()
$timeoutMs = $TimeoutSeconds * 1000

Add-SubstrateResult -Results $results -Name "podman" -CommandName "podman" -ArgumentLine "--version" -TimeoutMs $timeoutMs

$onWindows = $env:OS -eq "Windows_NT"
$containerCommand = Get-FirstCommand "container"
$containerPath = if ($containerCommand) {
    if ($containerCommand.Source) { $containerCommand.Source } else { $containerCommand.Path }
}
if ($containerCommand -and $onWindows -and ($containerPath -match "[\\/]WSL[\\/]container\.exe$")) {
    Add-SubstrateResult -Results $results -Name "wsl-containers" -CommandName "container" -ArgumentLine "--version" -TimeoutMs $timeoutMs
}
else {
    Add-SubstrateResult -Results $results -Name "apple-containers" -CommandName "container" -ArgumentLine "--version" -TimeoutMs $timeoutMs
}

if ($onWindows) {
    Add-SubstrateResult -Results $results -Name "wsl-host" -CommandName "wsl" -ArgumentLine "--status" -TimeoutMs $timeoutMs
}
else {
    $results.Add([pscustomobject][ordered]@{
        substrate = "wsl-host"
        status = "not_applicable"
        command = "wsl"
        path = $null
        exit_code = $null
        version = $null
        detail = "non-Windows host"
    })
}

$report = [pscustomobject][ordered]@{
    schema = "phenotype.infra/execution-substrate-capability/v1"
    observed_utc = $observedUtc
    host = [pscustomobject][ordered]@{
        name = $hostName
        os = [System.Runtime.InteropServices.RuntimeInformation]::OSDescription
        architecture = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
    }
    read_only = $true
    credentials_inspected = $false
    substrates = @($results)
}

if ($Json) {
    $report | ConvertTo-Json -Depth 8
    exit 0
}

Write-Output "Execution substrate capability probe (read-only)"
Write-Output ("Observed UTC: " + $report.observed_utc)
$report.substrates |
    Select-Object substrate, status, command, version, detail |
    Format-Table -AutoSize
exit 0
