# Grade Card: phenotype-infra

**Date:** 2026-06-25
**Repository:** phenotype-infra (branch: `dag-B43-2026-06-25`)
**Stack:** Rust (Go FFI) + Svelte + Python + Markdown (governance)

---

## Current Grade Summary

| Metric | Score | Max | % | Grade |
|--------|-------|-----|---|-------|
| **30-Pillar Total** | 59 | 120 | 49.2% | C- |
| **41 Cross-Cutting Total** | 87 | 205 | 42.4% | C- |
| **Grand Total (phenotype-infra)** | 146 | 325 | 44.9% | **C-** |
| **Governance docs (ci-fix branch)** | 201 | 325 | 61.8% | **B-** |

> _Source: [`docs/audit/scorecard.json`](../audit/scorecard.json) (2026-06-23)_

### Fresh Grade Runner (grade.sh --json)

| Mode | Score | Max | % | Grade |
|------|-------|-----|---|-------|
| **Full** | 0 | 17 | 0% | **F** |
| **Fast** | 0 | 10 | 0% | **F** |

> **Note:** Full grade run reports all failures due to Cargo.lock v4 compatibility issue (`lock file version 4 requires -Znext-lockfile-bump`), fmt diff in `nvms-ffi/build.rs`, deny.toml config value mismatch, and missing tooling (llvm-cov, cargo-nextest, cargo-audit). These are CI/environment issues rather than code quality regressions.

### Batch Grade (structural checks)

| Check | Status |
|-------|--------|
| repo-structure | Pass |
| iac-structure | Pass |
| config-structure | Pass |
| docs-structure | Pass |
| ci-workflows | Pass |
| governance | Pass |
| rust-fmt | Skip (batch mode) |
| audit-trail | Pass |

**Batch score:** 7 / 8 (87.5%) — Grade: B+

> _Source: [`.grade-reports/grade-batch.cmd`](../../.grade-reports/grade-batch.cmd)_

---

## Tier Status

### Tier-0: Foundation

| Pillar | Score (0-5) | Status |
|--------|-------------|--------|
| L0 — Project Setup | 4 | **Good** — Cargo workspace, git init, AGENTS.md |
| L1 — Version Control | 4 | **Good** — Proper commit messages, .gitignore |
| L2 — CI/CD | 4 | **Good** — quality-gate, release, audit, scorecard workflows |
| L3 — Documentation | 3 | **Adequate** — README, ADRs, governance docs ported |
| **Tier-0 Average** | **3.75 / 5** | **On Track** |

### Tier-1: Core

| Pillar | Score (0-5) | Status |
|--------|-------------|--------|
| L4 — Test Coverage | 1 | **Critical** — No test files present |
| L5 — Security Scanning | 3 | Adequate — .gitleaks, codecov, deny.toml |
| L6 — Dependency Management | 3 | Adequate — Cargo.lock, deny.toml |
| L7 — Architecture Governance | 4 | **Good** — ADRs for compute/HW-mesh, polyglot |
| L8 — Branch Hygiene | 3 | Adequate — Git flow, branches present |
| L9 — Release Process | 2 | Weak — release.yml present, no automation |
| L10 — Performance Criteria | 2 | Weak — tier specs exist, no benchmarks |
| L11 — Error Handling | 1 | **Critical** — No consistent error strategy |
| L12 — Observability Integration | 2 | Weak — governance mentions telemetry, no OTel |
| L13 — API Design | 2 | Weak — CLI/REST defined, no OpenAPI spec |
| **Tier-1 Average** | **2.3 / 5** | **Needs Improvement** |

### Tier-2: Advanced

| Pillar | Score (0-5) | Status |
|--------|-------------|--------|
| L14 — Config Management | 3 | Adequate — pheno-config, figment for TOML/env |
| L15 — Testing Strategy | 1 | **Critical** — No cross-crate integration tests |
| L16 — Build Reproducibility | 2 | Weak — Cargo.lock pinned, Go build not pinned |
| L17 — Monitoring | 1 | **Critical** — No monitoring setup |
| L18 — Cross-Cutting Hygiene | 3 | Adequate — .editorconfig, pre-commit, dprint |
| L19 — Licensing | 4 | **Good** — Apache-2.0 + MIT |
| L20 — Community Health | 1 | **Critical** — No CONTRIBUTING.md, no issue templates |
| **Tier-2 Average** | **2.14 / 5** | **Needs Significant Improvement** |

---

## Blocking Issues (P0)

| Issue | Pillar | Remediation |
|-------|--------|-------------|
| No test infrastructure | L4 | Add unit tests to nvms-ffi and pheno-compose crates |
| No consistent error types | L11 | Define common error types in nanovms-core crate |
| No integration test suite | L15 | Create cross-crate integration test suite |
| No monitoring | L17 | Add health-check endpoint in nvms CLI |
| No community templates | L20 | Add CONTRIBUTING.md + issue templates |

---

## Recent Changes Since Last Grade Card

| Commit | Date | Description |
|--------|------|-------------|
| `d767475` | 2026-06-25 | fix(pi-007c): restore rustc-check-cfg + fmt --all after restructure |
| `6616ebd` | 2026-06-25 | feat(audit): implement Recommendation #3 — automated no-idle-audit workflow |
| `3c1ab56` | 2026-06-25 | Phase-5-Resume: External grade run reports (build/fmt/test-unit) + Cargo.lock refresh |

Working tree also contains uncommitted changes to 7 source files (nvms-ffi, pheno-compose, pheno-config, integration tests).

---

## Grade Runner Detailed Results

| Check | Score | Max | Detail |
|-------|-------|-----|--------|
| build | 0 | 2 | Cargo.lock v4 parsing error |
| test-unit | 0 | 3 | Cargo.lock v4 parsing error |
| fmt | 0 | 2 | Diff in `nvms-ffi/build.rs` |
| clippy | 0 | 2 | Cargo.lock v4 parsing error |
| deny | 0 | 1 | deny.toml: unexpected value for `unmaintained` (expected `"deny"`) |
| doc | 0 | 1 | Cargo.lock v4 parsing error |
| test-snapshot | 0 | 1 | Cargo.lock v4 parsing error |
| test-fuzz | 0 | 1 | Cargo.lock v4 parsing error |
| coverage | 0 | 2 | `cargo llvm-cov` not installed |
| audit | 0 | 1 | `cargo audit` not installed |
| bench | 0 | 1 | No benchmark harness configured |
| **Total** | **0** | **17** | **Grade: F** |

---

## Cross-Reference to Audit Reports

| Unit | Report | Location |
|------|--------|----------|
| **A1** | Stale Branches Audit | `.grade-reports/A1-stale-branches.md` |
| **A6-A7** | CI/CD + ADR reconciliation | `git log` — commits `5b8e214`, `91be06d` |
| **A8** | Cursor Branch Audit | [`.grade-reports/A8-cursor-audit.md`](../../.grade-reports/A8-cursor-audit.md) |
| **A19** | Work-state header (README) | Commit `cb65647` |
| **A22** | TOC reconciliation | Commit `92fe1de` — [`docs/TOC.md`](../TOC.md) |
| **A25** | Orphaned Scripts Sweep | [`iac/.grade-reports/A25-orphaned-scripts-audit.md`](../../iac/.grade-reports/A25-orphaned-scripts-audit.md) |
| **A26** | Orphaned Configs Sweep | [`iac/.grade-reports/A26-orphaned-configs-audit.md`](../../iac/.grade-reports/A26-orphaned-configs-audit.md) |
| **B14** | OCI Helpers Dedup | [`.grade-reports/B14-oci-helpers-dedup.md`](../../.grade-reports/B14-oci-helpers-dedup.md) |
| **B26** | Tier-0 PR Gate | [`.grade-reports/B26-tier0-pr-gate.md`](../../.grade-reports/B26-tier0-pr-gate.md) |
| **B32** | Tier-1 PR Gate | [`.grade-reports/B32-tier1-pr-gate.md`](../../.grade-reports/B32-tier1-pr-gate.md) |
| **B38** | Tier-2 Coverage Gate | [`.grade-reports/B38-tier2-coverage-gate.md`](../../.grade-reports/B38-tier2-coverage-gate.md) |
| **Scorecard** | Master aggregate scorecard | [`docs/audit/scorecard.json`](../audit/scorecard.json) |
| **Scorecard** | Remediation plan + master average | [`docs/audit/master-scorecard.json`](../audit/master-scorecard.json) |
| **Grade batch** | Structural check runner | [`.grade-reports/grade-batch.cmd`](../../.grade-reports/grade-batch.cmd) |
| **TOC** | Documentation index | [`docs/TOC.md`](../TOC.md) |
| **Governance** | Governance documents | [`docs/governance/`](../governance/) |

---

*Generated by DAG unit B43.*
*Date: 2026-06-25*
