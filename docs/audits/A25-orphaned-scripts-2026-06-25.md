# A25 — Sweep orphaned scripts in `iac/`

**Date:** 2026-06-25
**Executor:** Forge (A25 DAG unit)
**Branch:** `sweep/A25-orphaned-scripts`
**Repo:** `phenotype-infra` (at `C:\Users\koosh\phenotype-infra-ci-fix`)
**Status:** COMPLETED

## Scope

Scan `iac/` for scripts (\*.sh, \*.ps1, \*.py, \*.bat) and check if they
are referenced anywhere in the codebase (Cargo.toml, CI configs, Makefile,
Taskfile.yml, Rust source files, documentation).

## Inventory

| # | Script | Lines | Type | Verdict |
|---|--------|-------|------|---------|
| 1 | `iac/oci-post-acquire/oci-post-acquire.sh` | 7 | bash | **KEPT** — active, called from `oci-lottery/src/hooks.rs` (line 58-62) |
| 2 | `iac/oci-post-acquire/hooks.d.example/10-update-portfolio-card.sh` | 12 | bash | **KEPT** — intentional example scaffold, referenced in docs |
| 3 | ~~`iac/scripts/bootstrap-oci.sh`~~ | → `iac/archive/scripts/bootstrap-oci.sh` | bash | **ARCHIVED** |
| 4 | ~~`iac/scripts/health-check.sh`~~ | → `iac/archive/scripts/health-check.sh` | bash | **ARCHIVED** |
| 5 | ~~`iac/scripts/register-home-runner.sh`~~ | → `iac/archive/scripts/register-home-runner.sh` | bash | **ARCHIVED** |
| 6 | ~~`iac/scripts/install-windows-runner.ps1`~~ | → `iac/archive/scripts/install-windows-runner.ps1` | powershell | **ARCHIVED** |
| 7 | ~~`iac/tailscale/apply-acl.sh`~~ | → `iac/archive/tailscale/apply-acl.sh` | bash | **ARCHIVED** |

## Disposition

### Orphaned scripts → moved to `iac/archive/`

| Script | Archived to | Status |
|--------|-------------|--------|
| `bootstrap-oci.sh` | `iac/archive/scripts/bootstrap-oci.sh` | ✅ moved via `git mv` |
| `health-check.sh` | `iac/archive/scripts/health-check.sh` | ✅ moved via `git mv` |
| `register-home-runner.sh` | `iac/archive/scripts/register-home-runner.sh` | ✅ moved via `git mv` |
| `install-windows-runner.ps1` | `iac/archive/scripts/install-windows-runner.ps1` | ✅ moved via `git mv` |
| `apply-acl.sh` | `iac/archive/tailscale/apply-acl.sh` | ✅ moved via `git mv` |

### Scripts intentionally kept

| Script | Reason |
|--------|--------|
| `oci-post-acquire.sh` | Active fallback — `oci-lottery/src/hooks.rs` line 58-62 resolves it at runtime |
| `10-update-portfolio-card.sh` | Intentionally shipped example hook; referenced in docs |

## Documentation updates

All documentation references were updated to reflect new paths:

| Doc updated | Script ref changed |
|-------------|--------------------|
| `iac/scripts/README.md` | Rewritten as archiving notice with cross-ref table |
| `iac/scripts/README-windows-runner.md` | Added archiving notice header |
| `iac/README.md` | Updated `scripts/` directory description + added `archive/` |
| `docs/runbooks/day1-oci-first-light.md` | `health-check.sh` path |
| `docs/runbooks/day1-home-runner-setup.md` | `register-home-runner.sh` path |
| `docs/runbooks/windows-desktop-runner.md` | `install-windows-runner.ps1` paths (x3) |
| `docs/runbooks/node-rebuild-recovery.md` | `health-check.sh` path |
| `docs/governance/tailscale-policy.md` | `apply-acl.sh` path |
| `docs/specs/rollback-kill-switch-spec.md` | `health-check.sh` path |
| `.github/workflows/terraform-plan.yml` | `bootstrap-oci.sh` comment path |

Archive READMEs created:
- `iac/archive/README.md` — overview of archive contents
- `iac/archive/scripts/README.md` — per-script documentation for preserved scripts

## Refs

- **Epic:** epic_A — Hygiene garden & branch slim
- **Unit:** A25 — Sweep orphaned scripts in iac/
- **Branch:** `sweep/A25-orphaned-scripts`
