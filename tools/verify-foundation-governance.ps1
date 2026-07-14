<#+
.SYNOPSIS
  Read-only check for the foundation ownership and evidence contracts.

.DESCRIPTION
  Verifies that phenotype-infra still carries the cross-repository boundary
  documents and their required evidence fields. This script only reads files;
  it does not contact providers, inspect credentials, or mutate cloud state.
#>
[CmdletBinding()]
param(
    [string] $RepoRoot = (Join-Path $PSScriptRoot "..")
)

$ErrorActionPreference = "Stop"
$failures = [System.Collections.Generic.List[string]]::new()

function Require-Pattern {
    param(
        [string] $Path,
        [string] $Pattern,
        [string] $Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        $failures.Add("Missing ${Description}: $Path")
        return
    }

    $content = Get-Content -LiteralPath $Path -Raw
    if ($content -notmatch $Pattern) {
        $failures.Add("Missing $Description pattern '$Pattern' in $Path")
    }
}

$computeMesh = Join-Path $RepoRoot "docs/governance/compute-mesh-state.md"
$toolSpheres = Join-Path $RepoRoot "docs/governance/tool-sphere-ownership.md"

Require-Pattern $computeMesh "inventory and evidence index" "compute-mesh inventory boundary"
Require-Pattern $computeMesh "Owner.*Source.*Verified.*Evidence" "compute-mesh evidence fields"
Require-Pattern $computeMesh "Provider credentials and mutable state never belong" "compute-mesh state safety rule"

Require-Pattern $toolSpheres "\| Tool sphere \| Owns \| Must not own \| Handoff to \| Evidence required \|" "tool-sphere ownership table"
Require-Pattern $toolSpheres "substrate" "substrate ownership entry"
Require-Pattern $toolSpheres "sharecli" "sharecli ownership entry"
Require-Pattern $toolSpheres "phenodag" "phenodag ownership entry"
Require-Pattern $toolSpheres "verified_utc" "tool-sphere evidence field"
Require-Pattern $toolSpheres "must\s+not become a second cloud-state or runtime\s+control plane" "orchestration-only boundary"

if ($failures.Count -gt 0) {
    $failures | ForEach-Object { Write-Error $_ }
    exit 1
}

Write-Output "Foundation governance checks passed (read-only)."
exit 0
