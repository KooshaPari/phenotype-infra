# Foundation forward DAG and current scorecard — 2026-07-31

Status: execution plan and evidence snapshot, not a release approval. Check
counts are volatile GitHub observations taken at 2026-07-31 06:01 UTC and
must be refreshed when a head or workflow run changes. Two heads changed
during this refresh; stale-head detection is itself a release gate.

## Frozen review surfaces

| Component | PR | Exact head | Base ancestry | Review state | Checks at snapshot |
| --- | ---: | --- | --- | --- | --- |
| BytePort | #318 | `ec599fa5531d899e22b963d0648ac72f04ab6577` | 73 ahead / 0 behind `main` | open draft, mergeable / blocked | 50 success, 21 failure, 1 neutral, 3 skipped, 2 active |
| NanoVMS | #128 | `9556e77dbedf2e5743df2c55569a46b5af6e22e9` | 43 ahead / 0 behind `main` | open, mergeable / blocked | 11 success, 4 failure, 3 skipped, 6 active |
| PhenoCompose | #113 | `fa7ad4ea8752d13fcab4a162140b3db21629558b` | 40 ahead / 0 behind `main` | open draft, mergeable / unstable | 24 success, 10 failure, 4 skipped, 2 active |
| phenotype-infra | #125 | `002243f57576300a77f5bc62eb4f0894055e2611` | 33 ahead / 0 behind `main` | open draft, mergeable / blocked | 30 success, 5 failure, 7 skipped |

All four review surfaces report no configured required checks through
`gh pr checks --required`. Therefore a green-looking subset is not a merge
decision: workflow failures, review approval, and Mergify policy remain
independent gates.

## Capability scorecard

This is a progress score, not a probability of release. The CI pillar is the
mean of `success / (success + failure + neutral)` for each repository,
multiplied by its weight; skipped and active checks do not earn credit.

| Pillar | Weight | Current | Basis and missing evidence |
| --- | ---: | ---: | --- |
| Exact ancestry and review surfaces | 10 | 10 | Every head is merge-base clean (0 behind) and has an open PR. |
| CI and security convergence | 35 | 26 | Mean completed non-skipped success ratio is about 74.8%; failures and active runs remain on every component. |
| Runtime probes and substrate adapters | 15 | 9 | Podman smoke and Apple/WSL capability probes exist; current-head lifecycle adapters are not yet proven. |
| Provider-neutral reconciliation | 15 | 8 | Stable workload ID, same-process replay, and changed-digest conflict exist; generation, cross-process uniqueness, execution reconciliation, and attested artifact identity do not. |
| Cross-component pilot | 15 | 3 | Component smokes exist; no authenticated current-head PhenoCompose -> BytePort -> NanoVMS receipt exists. |
| Governance, ownership, and evidence | 10 | 9 | Ownership boundaries and receipts exist; required-check policy, stale worktree metadata, and historical ADR debt remain. |
| **Capability progress** | **100** | **65** | Heuristic progress only. **Strict release gate: 0/4 heads green.** |

## Forward DAG

```text
A0 Evidence freeze (DONE)
 |  exact heads, ancestry, clean target worktrees, no destructive cleanup
 +----------------------+----------------------+
 |                      |
A1 CI/security repair   A2 Contract lock
 |                      |  digest, identity, generation,
 |                      |  artifact, audit and ownership ADRs
 +----------+-----------+
            |
      B1 Exact-head component gates
      |  focused tests, full CI, security, review approval
      +---------------------+
                            |
                    C1 Podman substrate proof
                    |  health, readiness, inspect/readback
                    +----------------------+
                                           |
                                  C2 Runtime bridge
                                  |  PhenoCompose -> BytePort submit/readback
                                  |  BytePort -> NanoVMS deploy/status
                                  |  labels and image digest preserved
                                  +----------------------+
                                                                 |
                                                        D1 Current-head pilot
                                                        |  authenticated 3-hop run
                                                        |  receipt and rollback
                                                        +----------------------+
                                                                 |
                                                        D2 Provider expansion
                                                        |  Apple Containers and WSL
                                                        |  probe-first, then lifecycle
                                                        +----------------------+
                                                                 |
                                                        E1 Publish/deploy gate
                                                        |  attestations, signed artifacts,
                                                        |  rollback drill, required checks
```

### Node contracts and owners

| Node | Owner(s) | Depends on | Exit evidence |
| --- | --- | --- | --- |
| A0 | phenotype-infra | none | This snapshot, exact SHAs, merge-base counts, and clean target worktrees. |
| A1 | each repository owner | A0 | Every source/security failure is fixed or explicitly classified with a reproducible external blocker; exact-head CI is green. |
| A2 | phenotype-infra + BytePort + PhenoCompose + NanoVMS | A0 | One canonical digest (`sha256:<64 lowercase hex>` at control-plane boundaries), stable workload `id` plus `generation`, immutable artifact reference, and correlation labels are documented and tested. |
| B1 | all four owners | A1, A2 | Focused tests pass with terminal exit 0, full repository gates pass, review approvals exist, and no hidden required-check gap remains. |
| C1 | NanoVMS + substrate | B1 (component gates) | Podman-only disposable run: `/readyz`, deploy, inspect/readback, status, label/digest verification, and cleanup receipt. |
| C2 | PhenoCompose + BytePort + NanoVMS | A2, C1 | Authenticated transport path returns workload ID, selected provider/backend, sandbox ID, status, and digest with no test double. |
| D1 | integration owner | C2 and all exact-head gates | One current-head receipt `{render_digest, workload_id, sandbox_id, provider, status, timestamps}` plus failed-start and rollback evidence. |
| D2 | PhenoCompose + substrate | D1 | Apple Containers and first-party WSL probes are capability-gated; each provider has a positive lifecycle proof or remains explicitly probe-only. |
| E1 | phenotype-infra governance | D1, D2 | Signed/attested artifacts, deploy/publish rehearsal, rollback drill, configured required checks, and a refreshed receipt with all four release gates green. |

## Parallel execution lanes and five-day horizon

| Window | Parallel work | Gate to advance |
| --- | --- | --- |
| Day 0 (now) | Freeze heads; classify current failures; preserve the missing `BytePort-pilot` worktree metadata and unrelated untracked binary until separately authorized. | A0 recorded; no unreviewed cleanup. |
| Day 1 | BytePort CI/security triage; Nano Trunk/tool bootstrap; PhenoCompose Rust/platform gates; phenotype-infra coverage/markdown gates. In parallel, draft A2 contract tests. | A1 failure list is reduced to actionable source/config items; A2 reviewable. |
| Day 2 | Land contract tests and adapters; add BytePort generation/cross-process uniqueness; add Nano inspect/readback and image digest; add PhenoCompose authenticated client. | B1 focused tests and exact-head CI. |
| Day 3 | Podman current-head integration; run bridge against isolated Postgres/Podman; capture receipt and rollback. | C1 and C2 terminal evidence. |
| Day 4 | Execute D1 pilot; replay, conflict, failed-start, and rollback cases; review receipt. | D1 complete with no substituted test doubles. |
| Day 5 | Apple/WSL probe and lifecycle matrix; attest/publish rehearsal; configure required checks and refresh scorecard. | D2/E1 or an explicitly documented residual blocker. |

## Immediate next actions

1. Do not merge or publish any of the four PRs while their exact-head gates
   are non-green.
2. Triage source/config failures before waiting on neutral, active, Mergify,
   or Summary automation. NanoVMS's Trunk bootstrap is now green; its exact
   head still has three workflow-format findings and one tool-helper typecheck
   finding until the latest rerun settles. BytePort has the largest active
   failure surface.
3. Refresh `foundation-pilot-receipt-2026-07-30.md` only after the frozen SHAs
   and check snapshot are intentionally changed; do not overwrite historical
   runtime evidence.
4. Keep the live pilot behind C2. A fixture digest or a direct NanoVMS smoke
   is not a three-hop receipt.
5. Before E1, add GitHub required-check policy and resolve the stale worktree
   metadata through a separately reviewed, reversible maintenance action.

## Ownership boundary

`phenotype-infra` remains the spine and ADR/governance source; BytePort owns
provider-neutral state and IaC; PhenoCompose owns composition translation and
runtime adapters; NanoVMS owns sandbox execution and lifecycle. Substrate,
`sharecli`, and `phenodag` remain separate owners and are not absorbed by this
plan.
