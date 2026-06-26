# A25 — Orphaned Scripts Audit (`iac/`)

**Date:** 2026-06-25
**Branch:** `dag-A25-2026-06-25`
**Auditor:** DAG unit A25 (infra-pool, phenotype-infra)
**Scope:** All `.sh`, `.ps1`, `.bat`, `.py` standalone scripts under `iac/`, plus standalone `.rs` cargo-scripts.

---

## Methodology

1. **Discover**: Enumerate every `.sh`, `.ps1`, `.bat`, `.py` file under `iac/` recursively.
2. **Enrich**: Also find standalone `.rs` cargo-scripts (files with `#!/usr/bin/env -S cargo` shebang) that live outside Rust crate directories.
3. **Cross-reference**: For each script, grep for its filename (without extension and with) across:
   - `Cargo.toml`, `Makefile`, `lefthook.yml` (build files)
   - `iac/README.md`, `iac/scripts/README.md` (directory index)
   - All `.github/workflows/*.yml` (CI/CD)
   - All `docs/*.md` (runbooks, governance, specs)
   - All sibling scripts and data files
   - `git log --all` for historical references
4. **Classify**:
   - **In-use**: referenced by ≥1 non-trivial source (README, CI, runbook, code)
   - **Orphaned**: zero references outside the file itself and its own directory README
   - **Unclear**: referenced only in self-referential or incidental mentions

---

## Discovered Scripts

### `iac/scripts/` — Operator-run bootstrap & runner helpers

| # | File | Type | Lines | Purpose |
|---|------|------|-------|---------|
| 1 | `bootstrap-oci.sh` | Bash | 7 | Chains `terraform apply` + `ansible-playbook` for OCI bring-up |
| 2 | `health-check.sh` | Bash | 7 | Probes tailnet node liveness via `tailscale ping` |
| 3 | `register-home-runner.sh` | Bash | 29 | Writes launchd plist for woodpecker-agent on macOS |
| 4 | `install-windows-runner.ps1` | PowerShell | 370 | Installs GitHub Actions self-hosted runner on Windows 11 |
| 5 | `restore-rulesets.rs` | Rust (cargo-script) | 243 | Re-adds dropped ruleset rules via `gh api` |

### `iac/oci-post-acquire/` — Post-acquisition hook chain

| # | File | Type | Lines | Purpose |
|---|------|------|-------|---------|
| 6 | `oci-post-acquire.sh` | Bash | 7 | ≤5-line glue wrapper that execs the Rust binary |
| 7 | `10-update-portfolio-card.sh` | Bash | 12 | Example hook: flips `mesh: ✅` in portfolio data |

### `iac/tailscale/` — Tailscale mesh management

| # | File | Type | Lines | Purpose |
|---|------|------|-------|---------|
| 8 | `apply-acl.sh` | Bash | 9 | PUTs `acl.json` to Tailscale API |

---

## Cross-Reference Results

### Build files

| Script | `Cargo.toml` | `Makefile` | CI workflows |
|--------|-------------|-----------|-------------|
| `bootstrap-oci.sh` | — | — | `.github/workflows/terraform-plan.yml:47` (comment) |
| `health-check.sh` | — | — | — |
| `register-home-runner.sh` | — | — | — |
| `install-windows-runner.ps1` | — | — | — |
| `restore-rulesets.rs` | — | — | — |
| `oci-post-acquire.sh` | `oci-post-acquire/Cargo.toml:11` (doc comment) | — | — |
| `10-update-portfolio-card.sh` | — | — | — |
| `apply-acl.sh` | — | — | — |

### Documentation & runbooks

| Script | References in `docs/` |
|--------|----------------------|
| `bootstrap-oci.sh` | — |
| `health-check.sh` | `docs/specs/rollback-kill-switch-spec.md:34`, `docs/runbooks/node-rebuild-recovery.md:25`, `docs/runbooks/day1-oci-first-light.md:99` |
| `register-home-runner.sh` | `docs/runbooks/day1-home-runner-setup.md:52,56` |
| `install-windows-runner.ps1` | `docs/runbooks/windows-desktop-runner.md:6,27,60,125` |
| `restore-rulesets.rs` | — |
| `oci-post-acquire.sh` | `docs/governance/oci-acquire-hook-chain.md:5,12,85,100`, `docs/runbooks/day1-oci-first-light.md:24` |
| `10-update-portfolio-card.sh` | `docs/governance/oci-acquire-hook-chain.md:88` |
| `apply-acl.sh` | `docs/governance/tailscale-policy.md:115` |

### README index references

| Script | `iac/README.md` | `iac/scripts/README.md` |
|--------|----------------|------------------------|
| `bootstrap-oci.sh` | — | `line 10` |
| `health-check.sh` | — | `line 12` |
| `register-home-runner.sh` | — | `line 11` |
| `install-windows-runner.ps1` | — | `lines 13,17,42,48` |
| `restore-rulesets.rs` | — | — |
| `oci-post-acquire.sh` | `line 13` | n/a |
| `10-update-portfolio-card.sh` | — | n/a |
| `apply-acl.sh` | — | n/a |

### Code references

| Script | Referenced from |
|--------|----------------|
| `oci-post-acquire.sh` | `iac/oci-lottery/src/hooks.rs:36,51,58-61` (legacy fallback resolution in Rust hook chain) |
| `apply-acl.sh` | `iac/tailscale/acl.json:3` (JSON comment citing the apply script) |
| `restore-rulesets.rs` | `iac/data/billing-blocked-rules.json:3` (data description) |

### `git log` evidence

| Script | Last commit touching it | Date |
|--------|------------------------|------|
| `bootstrap-oci.sh` | `3a85ed9` (feat: Tailscale ACL) | 2026-04-25 |
| `health-check.sh` | `3a85ed9` | 2026-04-25 |
| `register-home-runner.sh` | `3a85ed9` | 2026-04-25 |
| `install-windows-runner.ps1` | `9c19fcb` (fix: runner) | 2026-04-24 |
| `restore-rulesets.rs` | `3a85ed9` | 2026-04-25 |
| `oci-post-acquire.sh` | `b7116a5` | 2026-04-25 |
| `10-update-portfolio-card.sh` | `b7116a5` | 2026-04-25 |
| `apply-acl.sh` | `3f91947` | 2026-04-25 |

---

## Classification

| # | Script | Status | Rationale |
|---|--------|--------|-----------|
| 1 | `bootstrap-oci.sh` | **In-use** | Listed in `iac/scripts/README.md`, mentioned in CI workflow |
| 2 | `health-check.sh` | **In-use** | Listed in README, cited by 3 runbooks/specs |
| 3 | `register-home-runner.sh` | **In-use** | Listed in README, cited by day1 runbook |
| 4 | `install-windows-runner.ps1` | **In-use** | Listed in README, has companion README, cited by runbook, actively maintained |
| 5 | `restore-rulesets.rs` | **In-use (dormant)** | Consumer cited in data file; on standby until billing restored |
| 6 | `oci-post-acquire.sh` | **In-use** | Runtime fallback in Rust hook chain, documented in README + governance |
| 7 | `10-update-portfolio-card.sh` | **In-use (example)** | Documented example hook, part of hooks.d.example |
| 8 | `apply-acl.sh` | **In-use** | Canonical apply method for ACL, cited in `acl.json` and governance doc |

---

## Summary

| Metric | Count |
|--------|-------|
| Total scripts discovered | 8 |
| In-use | 6 |
| In-use (dormant / example) | 2 |
| Orphaned | **0** |
| Unclear | 0 |

**No orphaned scripts found.** Every script under `iac/` has at least one substantive reference in the codebase — README index, runbook, governance doc, CI workflow, data file, or runtime code.

### Recommendations

1. **`restore-rulesets.rs`**: Consider adding a reference to this script in `docs/runbooks/` so operators know it exists when billing is restored. Currently only referenced from the data file it consumes.
2. **No cleanup actions needed**: No `git rm` or archival moves are warranted at this time.
3. **Future-proofing**: If new scripts are added to `iac/scripts/`, ensure they are listed in `iac/scripts/README.md` and cited in at least one runbook.
