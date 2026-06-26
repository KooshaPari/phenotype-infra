# G4: SLSA L3 on IaC Modules — Assessment Report

**Date:** 2026-06-25
**Unit:** G4 (Infra Pool)
**Type:** slsa
**Target:** IaC modules under `iac/` (Terraform, Ansible, Rust operational crates)
**Repo:** KooshaPari/phenotype-infra

---

## Executive Summary

This report evaluates the current SLSA (Supply-chain Levels for Software Artifacts)
posture of the IaC modules in `phenotype-infra/iac/`. The assessment finds that
the project satisfies **SLSA L1** and **most of L2**, but requires targeted
work to reach **SLSA L3**.

| Level | Status | Notes |
|-------|--------|-------|
| **L1** (Build scripts) | ✅ Met | CI workflows are version-controlled and reproducible |
| **L2** (Isolated, version-controlled) | ✅ Met (partial) | Ephemeral runners; needs hash-pinned actions |
| **L3** (Hardened, provenance) | ❌ Not met | No signed provenance attestations; no hermetic build enforcement |

---

## 1. Current State Assessment

### 1.1 IaC Module Inventory

The `iac/` directory contains multiple classes of infrastructure artifacts:

| Category | Contents | Build/Release Artifacts |
|----------|----------|------------------------|
| **Terraform modules** | `terraform/{oci,gcp,aws,cloudflare}/` | `.tf` plan output (no binaries) |
| **Ansible playbooks** | `ansible/playbooks/` | No build artifacts |
| **Rust operational crates** | `oci-lottery`, `oci-post-acquire`, `tailscale-keygen`, `landing-bootstrap`, `observability`, `phenotype-logging-stub` | Binary releases via `cargo build` |
| **Shell scripts** | `scripts/` | No build artifacts; invoked directly |
| **Supporting crates** | `oci-helpers` | Library, linked into binaries |

### 1.2 Existing SLSA-Supporting Infrastructure

**Provenance & Build Integrity:**
- ✅ `Cargo.lock` committed for the `iac/` workspace — pins dependency versions
- ✅ All Rust builds use `--locked` flag in CI (`iac-rust.yml`, `ci.yml`)
- ✅ `cargo-deny` and `cargo audit` run in CI (`audit.yml`)
- ✅ `codeql.yml` runs static analysis on Rust code
- ✅ `trufflehog.yml` scans for secrets
- ✅ `scorecard.yml` runs OpenSSF Scorecard analysis weekly

**Build Isolation:**
- ✅ All CI runs on ephemeral GitHub Actions runners (satisfies L3 isolation)
- ✅ `cancel-in-progress: true` on all workflows prevents concurrent contamination

**Missing (blocking L3):**
- ❌ No **SLSA provenance attestations** (intoto statement + signed DSSE envelope)
- ❌ No **cosign/GPG signing** of build artifacts
- ❌ No **hermetic build** enforcement (network access not restricted in CI)
- ❌ No **verified provenance** consumption step
- ❌ GitHub Actions not pinned by hash in most workflows (use semver tags)
- ❌ No `release-attest.yml` workflow for provenance generation

### 1.3 License Review

| File | Content | SLSA Relevance |
|------|---------|----------------|
| `LICENSE` | Apache 2.0 (2024 Phenotype Org) | Standard OSS license |
| `LICENSE-APACHE` | Apache 2.0 | Duplicate of `LICENSE` |
| `LICENSE-MIT` | MIT license | Dual-licensed workspace |
| `iac/landing-bootstrap/templates/governance/LICENSE.MIT` | MIT template | For generated scaffolding |

No SLSA-specific license concerns. Dual-licensing (MIT/Apache-2.0) is compatible
with SLSA L3 verification tooling.

### 1.4 SLSA L1 Requirements

**Build scripts are defined:** ✅
- `iac-rust.yml` defines Rust crate CI/CD
- `terraform-plan.yml` defines Terraform validation
- `ci.yml` defines general Rust checks
- `quality-gate.yml` defines Go + Rust change-filtered checks
- `release.yml` defines tagged release pipeline

### 1.5 SLSA L2 Requirements

**Version-controlled build:** ✅
- All workflow definitions are committed in `.github/workflows/`
- All IaC source files are in git

**Isolated build:** ✅
- All jobs run on fresh `ubuntu-24.04` or `ubuntu-latest` runners
- No persistent build agents

**Requires: hash-pinned actions:** ❌
- Most `uses:` references use semver tags (e.g., `actions/checkout@v4`)
  instead of commit SHA pins. SLSA L3 requires hash-pinning for supply-chain
  integrity.

### 1.6 SLSA L3 Requirements (Gap Analysis)

| Requirement | Current State | Gap | Effort |
|-------------|--------------|-----|--------|
| **Provenance attestation** | No attestation generated | Must add `slsa-github-generator` to release workflow | Medium |
| **Provenance verification** | No verification step | Must add `slsa-verifier` to consume attestations | Low |
| **Signed releases** | No artifact signing | Must add cosign or GPG signing | Medium |
| **Hermetic builds** | Network unrestricted | Must move to `slsa-github-generator` hermetic mode or add network restrictions | High |
| **Reproducible builds** | `--locked` flag used, `Cargo.lock` committed | Already satisfied for Rust | ✅ Done |
| **Hash-pinned deps** | Semver tags on GitHub Actions | Must pin to commit SHAs | Medium |
| **Script dependencies** | Unpinned shell scripts | Must hash-pin or version-pin external references | Low |

---

## 2. SLSA L3 Roadmap

### Phase 1: Foundation (Estimated: 1-2 days)

| Step | Detail | Depends On |
|------|--------|------------|
| 1.1 | Pin all GitHub Actions to commit SHAs across all workflows | None |
| 1.2 | Add `slsa-github-generator` attestation job to `release.yml` | Step 1.1 |
| 1.3 | Create `release-attest.yml` for provenance-only attestation workflow | Step 1.1 |
| 1.4 | Verify attestation output with `slsa-verifier` | Step 1.2 |

### Phase 2: Hardening (Estimated: 2-3 days)

| Step | Detail | Depends On |
|------|--------|------------|
| 2.1 | Add cosign keyless signing to release workflow | Phase 1 |
| 2.2 | Enforce hermetic builds: `CARGO_NET_OFFLINE=true` or `--frozen` | Phase 1 |
| 2.3 | Add `actions/attest-build-provenance` for each binary artifact | Phase 1 |
| 2.4 | Hash-pin script dependency URLs in `iac/scripts/` | None |

### Phase 3: Verification & Badging (Estimated: 1 day)

| Step | Detail | Depends On |
|------|--------|------------|
| 3.1 | Add provenance verification as PR gate | Phase 2 |
| 3.2 | Add SLSA badge to README.md | Phase 2 |
| 3.3 | Document SLSA build process in `docs/governance/slsa-provenance.md` | Phase 2 |

### Effort Summary

| Phase | Effort (engineering-days) | Risk |
|-------|--------------------------|------|
| Phase 1 | 1-2 | Low — well-documented GitHub Actions |
| Phase 2 | 2-3 | Medium — hermetic builds may break existing workflows |
| Phase 3 | 1 | Low — documentation and badging |
| **Total** | **4-6 engineering-days** | |

---

## 3. Infrastructure That Already Supports SLSA

### GitHub Actions Ecosystem

The following existing infrastructure is directly leveraged for SLSA:

1. **Ephemeral runners** — All jobs run on GitHub-hosted `ubuntu-24.04` runners,
   which are ephemeral and discarded after each job. This satisfies the
   build-isolation requirement for L3.

2. **`Cargo.lock` — pinned dependencies** — The `iac/` workspace commits its
   `Cargo.lock`, and all CI commands use `--locked`. This ensures reproducible
   Rust builds, a prerequisite for SLSA L3.

3. **`scorecard.yml` (OpenSSF Scorecard)** — Already runs weekly. Scorecard
   includes SLSA-specific checks and publishes results. This provides a baseline
   for tracking SLSA posture improvements.

4. **OpenID Connect (OIDC)** — The repository has `id-token: write` permissions
   configured in `scorecard.yml`, which means GHA OIDC tokens are available.
   These are required by `slsa-github-generator` for keyless provenance signing.

5. **`release.yml` workflow** — Already defines the release pipeline with
   `actions/upload-artifact@v4`. Adding a provenance attestation step is a
   minimal delta.

6. **CodeQL + trufflehog** — Existing security scanning provides complementary
   supply-chain security beyond SLSA's strict scope.

### Terraform-Specific Considerations

The Terraform modules (`iac/terraform/`) produce `.tfplan` files, not
compiled binaries. For SLSA L3 purposes, the relevant build artifact is the
**Terraform plan output**. This can be attested with:

```yaml
- name: Generate Terraform plan provenance
  uses: actions/attest-build-provenance@v2
  with:
    subject-path: "${{ runner.temp }}/terraform.plan"
```

However, since `terraform plan` requires credentials that are not available in
CI (see `terraform-plan.yml:47`), provenance for Terraform outputs is deferred
until the human-applied plan step.

---

## 4. Recommendations (Priority Order)

### P0 — Do this sprint

1. **Create `release-attest.yml`** — Adds a reusable workflow that generates
   SLSA provenance attestations for the Rust crate binaries. This uses
   `slsa-framework/slsa-github-generator` and generates signed intoto
   statements. See the companion file committed alongside this assessment.

2. **Pin GitHub Actions to commit SHAs** — Replace all `@v4`, `@v3`, `@stable`
   tags with the corresponding commit SHAs across all 17 workflow files.
   This is a prerequisite for SLSA L3's "no unverified dependencies" requirement.

### P1 — Next sprint

3. **Add attestation to `release.yml`** — After step 1, integrate
   `actions/attest-build-provenance@v2` directly into the existing release
   workflow so every tag push produces provenance.

4. **Enforce `--hermetic` on Rust builds** — Add
   `CARGO_NET_OFFLINE=true` or `--frozen` to ensure builds do not fetch
   dependencies at build time (they should be cached from `Cargo.lock`).

### P2 — Within 2 sprints

5. **Cosign keyless signing** — Add `sigstore/cosign-installer` and
   `cosign sign-blob` to sign all release artifacts with keyless signing
   (using OIDC).

6. **Script dependency audit** — Review `iac/scripts/` for any shell scripts
   that fetch external content without hash verification. Pin all external
   URLs with SHA256 checksums.

7. **SLSA badge** — Add `[![SLSA L3](https://slsa.dev/images/gh-badge-level3.svg)](https://slsa.dev)`
   to `README.md` once attestation is verified.

---

## 5. Conclusion

**Current assessment:** SLSA L1 fulfilled, SLSA L2 partially fulfilled,
SLSA L3 not yet met. The primary blockers are:
1. No provenance attestation generation
2. No signed releases
3. GitHub Actions not hash-pinned

**The most impactful single action** is creating a `release-attest.yml` workflow
using `slsa-framework/slsa-github-generator`. This immediately enables SLSA
provenance for every release, verifiable by `slsa-verifier`. The companion
workflow file is committed alongside this report.

With an estimated **4-6 engineering-days** of effort, the IaC modules can
reach SLSA L3, providing verifiable supply-chain integrity for the phenotype
compute/infra consolidation stack.

---

*Generated by Forge G4 infra-pool DAG unit. 2026-06-25.*
