# iac/archive/ — Orphaned scripts preserved for reference

This directory contains scripts that were identified as orphaned during
the A25 sweep and moved here to keep the active `iac/` tree clean.

## Archive contents

| Subdirectory | Contents | Original location |
|-------------|----------|-------------------|
| `scripts/` | Bash/PowerShell runner helpers | `iac/scripts/` |
| `tailscale/` | Tailscale ACL apply script | `iac/tailscale/` |

## Why archived

These scripts are not called from any Cargo.toml, CI config, Makefile,
Taskfile.yml, or Rust source file in the repository. They are preserved
here for historical reference and manual use.

## Scripting-policy compliance

Per the Phenotype scripting policy, persistent daemons and operator-facing
tooling should be ported to Rust. These scripts remain as bash/PowerShell
because they are either:

- Single-use operator-run glue (not worth a Rust binary)
- Platform-specific (Windows PS1) with no cross-platform equivalent
- Legacy fallbacks superseded by Rust binaries

If any of these scripts need to be revived, consider porting them to a Rust
binary under `iac/` first.
