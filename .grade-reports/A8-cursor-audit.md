# A8 — Cursor Branch Audit Report

**Date:** 2026-06-25
**Auditor:** DAG unit A8 (branch-clean)
**Repo:** KooshaPari/phenotype-infra
**DAG Branch:** `dag-A8-2026-06-25`

---

## Summary

| Total `cursor/*` branches | Last updated | Merged into `main` | Stale |
|--------------------------:|:------------:|:------------------:|:-----:|
| 6                         | 2026-04-25   | 0 (all unmerged)   | ~2 mo |

---

## Per-Branch Analysis

### 1. `cursor/governance-files-for-redirects-de22`
| Field       | Value                                                              |
|-------------|--------------------------------------------------------------------|
| Last commit | `23279b8` — 2026-04-25 21:35 UTC                                  |
| Message     | fix: only emit governance files for full scaffolds, not redirect-only repos |
| Diff        | 5 files, +199/−2 (landing-bootstrap templates/README)              |
| On `main`?  | No — 2 unique commits                                              |
| **Status**  | Has unique work. Needs review & PR.                                |

**Recommendation:** KEEP — Open a PR or manually merge the landing-bootstrap governance work.

---

### 2. `cursor/iac-configuration-bugs-1cab`
| Field       | Value                                                    |
|-------------|----------------------------------------------------------|
| Last commit | `8ec0b81` — 2026-04-25 17:01 UTC                        |
| Message     | Fix IaC configuration bugs                               |
| Diff        | 7 files, +710/−0 (restore-rulesets.rs, tailscale-keygen, docs) |
| On `main`?  | No — 3 unique commits                                    |
| **Status**  | Substantial new IaC tooling.                             |

**Recommendation:** KEEP/MERGE via PR. Needs review.

---

### 3. `cursor/infrastructure-bug-fixes-bda2`
| Field       | Value                                                        |
|-------------|--------------------------------------------------------------|
| Last commit | `3120c23` — 2026-04-25 16:47 UTC                            |
| Message     | fix: resolve infrastructure bugs in ACL, rulesets, and landing-bootstrap |
| Diff        | 11 files, +1399/−0 (large — includes microfrontend spec)     |
| On `main`?  | No — 4 unique commits                                        |
| **Status**  | Superset of branch 2 plus Tier-3 microfrontend work.         |

**Recommendation:** PRIORITY MERGE — Contains the most comprehensive IaC changes. Close branch 2 after this merges.

---

### 4. `cursor/oci-hooks-schema-path-4511`
| Field       | Value                                                              |
|-------------|--------------------------------------------------------------------|
| Last commit | `8411e80` — 2026-04-25 16:08 UTC                                  |
| Message     | fix(oci-hooks): align JSON schema and remove redundant PATH lookup |
| Diff        | 13 files, +279/−108 (oci-lottery, oci-post-acquire)                |
| On `main`?  | No — 2 unique commits                                              |
| **Status**  | Independent OCI hooks bug fix.                                     |

**Recommendation:** MERGE — Targeted, low-risk fix for OCI hooks.

---

### 5. `cursor/ruleset-id-null-deserialization-7e11`
| Field       | Value                                                              |
|-------------|--------------------------------------------------------------------|
| Last commit | `7d86adb` — 2026-04-25 17:27 UTC                                  |
| Message     | Fix null ruleset_id deserialization in restore-rulesets.rs         |
| Diff        | 3 files, +30/−8 (small targeted fix)                               |
| On `main`?  | No — 3 unique commits                                              |
| **Status**  | Small, targeted null deserialization fix.                          |

**Recommendation:** MERGE — Low risk, small fix.

---

### 6. `cursor/tailscale-acl-test-fixes-eaf7`
| Field       | Value                                                            |
|-------------|------------------------------------------------------------------|
| Last commit | `6361263` — 2026-04-25 16:08 UTC                                |
| Message     | fix: correct Tailscale ACL test assertions                       |
| Diff        | 5 files, +379/−0 (acl.json, apply-acl.sh, tailscale-keygen)      |
| On `main`?  | No — 2 unique commits                                            |
| **Status**  | Tailscale ACL test corrections; shares base with branches 2 & 3. |

**Recommendation:** KEEP/MERGE — Likely redundant with branch 3 once that merges.

---

## Overall Action Plan

| Step | Action | Details |
|------|--------|---------|
| 1 | **Open PR** for `cursor/infrastructure-bug-fixes-bda2` | Superset of branches 2 & 6 + Tier-3 microfrontends |
| 2 | **Open PR** for `cursor/oci-hooks-schema-path-4511` | Independent OCI fix |
| 3 | **Open PR** for `cursor/ruleset-id-null-deserialization-7e11` | Small null fix |
| 4 | **Open PR** for `cursor/governance-files-for-redirects-de22` | Governance templates |
| 5 | **After all merged** → delete all 6 cursor/* branches | Remote cleanup |

## Risk Note

All 6 branches are ~2 months stale (last updated April 2026). Upstream `main` has advanced significantly. Expect merge conflicts on PRs, especially for the larger branches (2, 3, 6) which share overlapping commits.

---

*Generated by DAG unit A8 — branch-clean/cursor-branch-audit on phenotype-infra*
