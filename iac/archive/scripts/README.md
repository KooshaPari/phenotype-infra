# Archived scripts — original documentation preserved

The scripts in this directory were originally at `iac/scripts/` and
`iac/tailscale/`. They are preserved here with their original content
for historical reference as part of the A25 orphaned-scripts sweep.

## Files

| Script | Original purpose |
|--------|------------------|
| `bootstrap-oci.sh` | Oracle Cloud ARM VM bootstrap — chains `terraform apply` + `ansible-playbook` |
| `register-home-runner.sh` | macOS — writes launchd plist for woodpecker-agent |
| `health-check.sh` | Fleet connectivity probe — tailscale ping to mesh nodes |
| `install-windows-runner.ps1` | Windows 11 AMD64 — installs GH Actions runner service |

## Usage note

These scripts are NOT actively maintained. If you need to use one:

1. Review the script content first — it may reference paths that no longer exist.
2. Consider porting the functionality to a Rust binary (per scripting policy).
3. The original documentation below is preserved inline.
