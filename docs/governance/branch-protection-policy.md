# Branch Protection Policy — phenotype-infra

- **Status:** Adopted (pending GitHub-side enforcement)
- **Date:** 2026-06-24
- **Owner:** Compute/Infra monorepo (`KooshaPari/phenotype-infra`)
- **Worklog ref:** T-PI.D-033/w
- **Supersedes:** ad-hoc protection (none — repo currently has 0/9 repos in the org
  requiring status checks per the parent DAG §1.1)

## Purpose

Codify the branch-protection settings on `phenotype-infra` so that:

1. Every PR is gated by the same set of CI checks before merge.
2. Direct pushes to `master` are forbidden; only PRs and admins may land.
3. The settings are reproducible from this document (no implicit tribal knowledge).
4. The policy is auditable: a reviewer can verify protection state against this
   file without needing to inspect the GitHub UI.

This closes the gap surfaced by the parent DAG: **0/9 repos in the org require
status checks**. Without enforced checks, a green main branch is a function of
reviewer discipline rather than tooling.

## Scope

Applies to:

- **Default branch:** `master` (the current default for `phenotype-infra`).
- **Long-lived release branches** once they exist (e.g. `release/v0.x`) — same
  rules, with `master` → branch name substitution.
- **Hotfix branches** — see "Exceptions" below.

Does **not** apply to:

- Feature branches (`feat/*`, `fix/*`, `chore/*`, `test/*`, `docs/*`, etc.) —
  these are short-lived and exist only to feed PRs.
- Archived branches (kept for history; no active commits expected).

## Required Status Checks

The following checks must pass on every PR before merge. They map 1:1 to jobs
in `.github/workflows/`:

| Check name (job) | Workflow | What it enforces |
|------------------|----------|------------------|
| `cargo (workspace)` | `ci.yml` | `cargo fmt --check`, `cargo check`, `cargo clippy`, `cargo test` across the whole workspace |
| `lint` | `docs-lint.yml` | Vale + actionlint docs hygiene |
| `lint` | `quality-gate.yml` | Repo-level quality gate (deny, audit, scorecard) |
| `coverage` | `coverage.yml` | Tarpaulin coverage report (informational; not a gate) |
| `scorecard` | `scorecard.yml` | OpenSSF Scorecard posture |
| `trufflehog` | `trufflehog.yml` | Secret-scan on every PR |
| `codeql` | `codeql.yml` | GitHub CodeQL security analysis |
| `audit` | `audit.yml` | Repo audit gate |
| `iac-rust` | `iac-rust.yml` | IaC + Rust cross-check |

> **Note:** GitHub matches these by **job name** (not workflow file name). The
> job names above must remain stable; renaming requires updating this doc.

## Settings

The canonical protection rule applied to `master`:

| Field | Value |
|-------|-------|
| `required_status_checks.strict` | **true** (re-runs after push trigger a fresh status) |
| `required_status_checks.contexts` | See table above |
| `enforce_admins` | **true** (admins also subject to the rule) |
| `required_pull_request_reviews.required_approving_review_count` | **1** |
| `required_pull_request_reviews.dismiss_stale_reviews` | **true** |
| `required_pull_request_reviews.require_code_owner_reviews` | **false** (CODEOWNERS file is `@KooshaPari` for everything; would block self-merge) |
| `required_pull_request_reviews.require_last_push_approval` | **true** |
| `required_linear_history` | **true** (no merge commits on `master`) |
| `allow_force_pushes` | **false** |
| `allow_deletions` | **false** |
| `block_creations` | **false** (branches may be created; merge is what is gated) |
| `required_conversation_resolution` | **true** |
| `lock_branch` | **false** |
| `allow_fork_syncing` | **false** |

> **Note on CODEOWNERS:** The current `CODEOWNERS` (root-level) is a single-owner
> catch-all (`* @KooshaPari`). Until CODEOWNERS is split by directory (a future
> PR), `require_code_owner_reviews` stays off to avoid blocking the owner's own
> merges. Once CODEOWNERS is granular, flip this to `true` and document the
> affected paths.

## How to Apply (GitHub API)

The protection rule is applied via the GitHub REST API. The repo must have
admin access in the GitHub token used. Run from any shell with `gh` or `curl`
authenticated:

```bash
# Requires: GitHub CLI authenticated as a repo admin.
# Owner: KooshaPari  /  Repo: phenotype-infra  /  Branch: master
gh api \
  --method PUT \
  -H "Accept: application/vnd.github+json" \
  /repos/KooshaPari/phenotype-infra/branches/master/protection \
  --input .github/protection/master.json
```

Where `.github/protection/master.json` (companion file to this doc) holds the
exact JSON payload matching the table above. The JSON file is the **source of
truth** for the API call; this Markdown doc is the human-readable narrative.

## How to Verify

Re-read the protection state and diff against this doc / `master.json`:

```bash
gh api /repos/KooshaPari/phenotype-infra/branches/master/protection | jq .
```

Spot-check:

- `required_status_checks.contexts` includes every row of the "Required
  Status Checks" table.
- `required_pull_request_reviews.required_approving_review_count == 1`.
- `enforce_admins.enabled == true`.
- `allow_force_pushes.enabled == false`.

Run this verification as part of:

1. Every release-tag cut.
2. Any change to `.github/workflows/*` (since renames break protection).
3. Quarterly, on the calendar, as a passive audit.

## Exceptions

**Hotfix branches** (e.g. `hotfix/cve-2026-xyz`) may temporarily relax
`required_linear_history` to allow emergency squash-merge from a fork when CI
is unreachable. After the hotfix lands on `master`, immediately re-apply the
rule via the API call above. Document each relaxation in
`docs/sessions/<date>-hotfix-relaxation.md` with the reason and re-application
timestamp.

**Repo archival.** When the repo transitions to `archived: true`, the
protection API call becomes a no-op; document the archival in
`docs/operations/repo-archive-log.md`.

## Adoption Checklist

1. [x] Author this document (`docs/governance/branch-protection-policy.md`).
2. [x] Author the JSON payload (`.github/protection/master.json`).
3. [ ] Apply via `gh api ...` once an admin token is wired.
4. [ ] Verify via the read-back call.
5. [ ] Note in `CHANGELOG.md` under `[Unreleased] → ### Changed`.
6. [ ] Cross-link from `docs/ABSORPTION_INDEX.md` "Adding entries" checklist
       so future absorption waves adopt the same policy.

## Related

- `CODEOWNERS` — repo-level owners file.
- `.github/workflows/ci.yml` — defines `cargo (workspace)` job.
- `plans/2026-06-23-phenotype-infra-validation.md` — original "0/9 repos
  require status checks" finding.
- `plans/2026-06-24-2026-06-24-t-pi-d-37-pr-plan-v1.0.md` §2.9 — PR spec for
  T-PI.D-033/w.