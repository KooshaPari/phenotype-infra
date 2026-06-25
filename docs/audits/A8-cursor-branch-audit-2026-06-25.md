# A8 — Cursor Branch Audit Report

**Date:** 2026-06-25
**Auditor:** Forge (DAG unit A8)
**Repo:** phenotype-infra-ci-fix
**Commit (main):** `3c1ab56` Phase-5-Resume: External grade run reports

---

## Summary

| Total cursor/* branches | Unique commits (vs main) | Merged into main | Status |
|------------------------:|-------------------------:|:-----------------:|:-------|
| 6                       | 16                       | 0                 | All unmerged |

---

## Per-Branch Disposition

### 1. `cursor/governance-files-for-redirects-de22`
- **Author:** Cursor Agent (top commit), Forge (base)
- **Date:** 2026-04-25
- **Unique commits (2):**
  - `23279b8` fix: only emit governance files for full scaffolds, not redirect-only repos
  - `18e3bd0` feat(landing-bootstrap): emit governance files + repo topics by default
- **Diff:** 5 files, +199/−2
- **Disposition:** **MERGE** — Landing-bootstrap governance template work. Valid changes, not superseded. Open a PR.

### 2. `cursor/iac-configuration-bugs-1cab`
- **Author:** Cursor Agent (top commit), Forge (base commits)
- **Date:** 2026-04-25
- **Unique commits (3):**
  - `8ec0b81` Fix IaC configuration bugs
  - `58f7478` feat(iac): add ruleset restoration script + tracking data
  - `3a85ed9` feat(iac): Tailscale ACL + ephemeral keygen for OCI hook chain
- **Diff:** 7 files, +710/−0
- **Disposition:** **KEEP/MERGE** — Substantial new IaC tooling (restore-rulesets.rs, tailscale-keygen). Needs review and PR to main.

### 3. `cursor/infrastructure-bug-fixes-bda2`
- **Author:** Cursor Agent (top commit), Forge (base commits)
- **Date:** 2026-04-25
- **Unique commits (4):**
  - `3120c23` fix: resolve infrastructure bugs in ACL, rulesets, and landing-bootstrap
  - `4251443` feat(governance,iac): Tier-3 path microfrontends spec + bootstrap default
  - `58f7478` feat(iac): add ruleset restoration script + tracking data
  - `3a85ed9` feat(iac): Tailscale ACL + ephemeral keygen for OCI hook chain
- **Diff:** 11 files, +1399/−0
- **Note:** This branch is a **superset** of branch 2 plus Tier-3 microfrontend work.
- **Disposition:** **MERGE (after dedup)** — Contains the most comprehensive set of IaC changes. Merge via PR. Close branch 2 in favor of this one after merge.

### 4. `cursor/oci-hooks-schema-path-4511`
- **Author:** Cursor Agent (top commit), Forge (base)
- **Date:** 2026-04-25
- **Unique commits (2):**
  - `8411e80` fix(oci-hooks): align JSON schema and remove redundant PATH lookup
  - `956b201` feat(iac): integrate oci-lottery → oci-post-acquire
- **Diff:** 13 files, +279/−108
- **Disposition:** **MERGE** — Bug fix for OCI hooks schema PATH. The changes touch iac/oci-lottery and iac/oci-post-acquire. Targeted and low-risk.

### 5. `cursor/ruleset-id-null-deserialization-7e11`
- **Author:** Cursor Agent (top commit), Forge (base commits)
- **Date:** 2026-04-25
- **Unique commits (3):**
  - `7d86adb` Fix null ruleset_id deserialization in restore-rulesets.rs
  - `265f308` chore(billing-blocked-rules): record phenotype-dep-guard PR #3 merge block
  - `fd564c4` feat(landing-bootstrap): scaffold Tier-3 paths by default
- **Diff:** 3 files, +30/−8
- **Disposition:** **MERGE** — Small, targeted fix for null deserialization + supporting changes. Cherry-pick into main via PR.

### 6. `cursor/tailscale-acl-test-fixes-eaf7`
- **Author:** Cursor Agent (top commit), Forge (base)
- **Date:** 2026-04-25
- **Unique commits (2):**
  - `6361263` fix: correct Tailscale ACL test assertions
  - `3a85ed9` feat(iac): Tailscale ACL + ephemeral keygen for OCI hook chain
- **Diff:** 5 files, +379/−0
- **Note:** Base commit `3a85ed9` shared with branches 2 and 3.
- **Disposition:** **KEEP/MERGE** — Tailscale ACL test corrections are valid. Merge alongside branch 2/3 work. Could be redundant with branch 3 once merged.

---

## Overall Action Plan

| Step | Action | Details |
|------|--------|---------|
| 1 | Open PR for branch 3 (`infrastructure-bug-fixes-bda2`) | Contains superset of branches 2 & 6 work + Tier-3 microfrontends |
| 2 | Open PR for branch 4 (`oci-hooks-schema-path-4511`) | Independent OCI hook schema PATH fix |
| 3 | Open PR for branch 5 (`ruleset-id-null-deserialization-7e11`) | Independent null deserialization fix |
| 4 | Open PR for branch 1 (`governance-files-for-redirects-de22`) | Governance template redirect fix |
| 5 | After all PRs merged → delete all 6 cursor/* branches | Clean up stale remote tracking refs |

---

## Recommendations

1. **Priority:** Branch 3 and branch 4 touch the most code and should be reviewed first.
2. **Conflict check:** Branches 2, 3, and 6 share commits `3a85ed9` and `58f7478`. Merge branch 3 first, then verify branches 2 and 6 are redundant.
3. **Risk:** These branches are 2 months stale (April 2026). Upstream `main` has advanced significantly. Expect merge conflicts.
4. **If abandoned work:** If these Cursor Agent commits represent exploratory work that was never reviewed, consider archiving or deleting after confirming no value is lost.

---

*Generated by DAG unit A8 — branch-clean/cursor-branch-audit*
