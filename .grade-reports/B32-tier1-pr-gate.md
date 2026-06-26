# B32 -- Tier-1 PR Enforcement Gate

**Date:** 2026-06-25
**Target:** `KooshaPari/phenotype-infra` (local at `C:\Users\koosh\phenotype-infra`)
**Branch:** `dag-B32-2026-06-25`
**Standard:** Tier-1 = security scan (cargo audit / trufflehog), SBOM, LICENSE check, CHANGELOG update
**Status:** FULLY ENFORCED (post-remediation)

---

## 1. Tier-1 Requirements Matrix

| Requirement                | Pre-B32 Status | Post-B32 Status | Workflow File(s)                      |
|----------------------------|----------------|-----------------|---------------------------------------|
| Cargo deny (advisories)    | Present     | Present      | cargo-deny.yml, ci.yml (audit job)|
| Cargo audit (vulns)        | Present     | Present      | ci.yml (audit job)                  |
| Trufflehog (secrets)       | Present     | Present      | trufflehog.yml                      |
| CodeQL (SAST)              | Present     | Present      | codeql.yml                          |
| Trivy (filesystem vulns)   | Present     | Present      | audit.yml (Security Guard)          |
| SBOM generation        | Missing     | Added    | sbom.yml (new)                      |
| LICENSE file check     | Missing     | Added    | license-check.yml (new)             |
| CHANGELOG update check | Missing     | Added    | changelog-check.yml (new)           |

---

## 2. Existing Gates (Unchanged)

### 2.1 cargo-deny.yml
- Trigger: pull_request on main + push on main + workflow_dispatch
- Tool: EmbarkStudios/cargo-deny-action@v2
- Path: iac/Cargo.toml
- Verdict: Adequate for Tier-1

### 2.2 ci.yml - audit job
- Trigger: pull_request on main
- Tools: cargo-deny + rustsec/audit-check@v2
- Verdict: Adequate for Tier-1

### 2.3 trufflehog.yml
- Trigger: pull_request (any) + push on main
- Tool: trufflesecurity/trufflehog@v3.95.6
- Verdict: Adequate for Tier-1

### 2.4 codeql.yml
- Trigger: pull_request on main + push on main
- Language: Rust analysis
- Verdict: Adequate for Tier-1

### 2.5 audit.yml (Trivy)
- Trigger: pull_request (any) + push on main/master + daily
- Tool: aquasecurity/trivy-action
- Verdict: Adequate for Tier-1

---

## 3. New Gates Added (This DAG Unit)

### 3.1 sbom.yml - SBOM Generation
- File: .github/workflows/sbom.yml
- Tool: anchore/sbom-action@v0 (Syft, CycloneDX JSON)
- Why: Supply-chain transparency (EO 14028)

### 3.2 license-check.yml - LICENSE Compliance
- File: .github/workflows/license-check.yml
- Checks: LICENSE files exist, Cargo.toml license declaration
- Why: Legal compliance

### 3.3 changelog-check.yml - CHANGELOG Enforcement
- File: .github/workflows/changelog-check.yml
- Checks: CHANGELOG.md exists and was modified in PR
- Why: Documentation hygiene

---

## 4. Summary

| Dimension | Coverage | Details |
|-----------|----------|---------|
| Security scanning | Complete | cargo deny, audit, trufflehog, CodeQL, Trivy |
| Supply-chain | Complete | SBOM (Syft CycloneDX) on every PR |
| License | Complete | File presence + workspace declaration |
| Docs hygiene | Complete | CHANGELOG update detection |

**Final Verdict: TIER-1 FULLY ENFORCED**
