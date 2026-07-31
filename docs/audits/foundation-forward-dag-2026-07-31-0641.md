# Foundation forward DAG and exact-head scorecard - 2026-07-31 06:44 UTC

Status: working execution plan and evidence snapshot, refreshed after the
06:41 UTC publication. This is not a merge,
publish, or release approval. GitHub check counts are a point-in-time
observation and must be refreshed whenever a head or workflow run changes.

## Scope and ownership

The four repositories form one foundation without collapsing their ownership:

| Layer | Owner | Contract |
| --- | --- | --- |
| Spine and governance | `phenotype-infra` | ADRs, ownership, evidence receipts, policy, IaC conventions |
| Provider-neutral control plane | `BytePort` | desired state, workload identity, reconciliation, provider/IaC adapters |
| Composition and translation | `PhenoCompose` | deterministic compose/proc/kube render, digest, and substrate adapter selection |
| Execution and lifecycle | `NanoVMS` | sandbox/container lifecycle, readiness, status, inspect, audit correlation |

`sharecli`, `phenodag`, and substrate implementations remain separately owned.
Docker is not a foundation dependency. Podman is the first positive local
substrate; first-party WSL Containers (`wslc`/`container.exe`) and Apple
Containers are capability-gated adapters.

## Exact remote review surfaces

The heads and ancestry below were fetched from GitHub, not inferred from a
possibly stale local checkout. All four compare as `ahead` with zero commits
behind `main`; all are mergeable by GitHub but remain blocked or unstable by
checks and/or review policy.

| Repo | PR | Branch | Exact head | Ahead/behind | Review state | Check snapshot |
| --- | ---: | --- | --- | --- | --- | --- |
| BytePort | #318 | `codex/byteport-workflow-syntax-pilot` | `aec8e356ab6b622f1d9aa6de806b05a706951a2d` | 75 / 0 | draft; mergeable, **BLOCKED**; review required | 49 pass, 21 fail, 1 neutral, 3 skipped, 2 pending |
| NanoVMS | #128 | `codex/nanovms-podman-provider` | `27f20dfb908607720d9df1ceef726838e7d34b16` | 44 / 0 | ready; mergeable, **BLOCKED**; review required | 14 pass, 4 fail, 3 skipped, 3 pending |
| PhenoCompose | #113 | `codex/phenocompose-pr112-workflow-fixes` | `7d321deeff222ac1b91fd51f4660cf084ccc3a54` | 41 / 0 | draft; mergeable, **UNSTABLE** | 19 pass, 7 fail, 4 skipped, 10 pending |
| phenotype-infra | #125 | `codex/phenotype-infra-pr115-docs-gates` | `d068787df87fc811b50c65b88c827f813260efc7` | 36 / 0 | draft; mergeable, **BLOCKED**; review required | 25 pass, 4 fail, 8 skipped, 3 pending |

No repository has a green exact-head release surface. `gh pr checks
--required` exposes no configured required-check subset for these PRs, so a
green subset cannot be treated as a merge decision.

## True current progress score

The score measures capability progress, not release probability. For each PR,
CI credit is `success / (success + failure + neutral)`; skipped and pending
checks earn no credit. The weighted score intentionally stays separate from
the hard release gates.

| Pillar | Weight | Score | Evidence and missing proof |
| --- | ---: | ---: | --- |
| Exact ancestry and review surfaces | 10 | 10.0 | Four exact heads, open PRs, zero commits behind; review approval and green checks are still absent. |
| CI and security convergence | 35 | 26.8 | Completed success ratios are BytePort 69.0%, NanoVMS 77.8%, PhenoCompose 73.1%, infra 86.2%; failures remain on all four. |
| Runtime probes and substrate adapters | 15 | 9.0 | Podman smoke and probe code exist; current-head lifecycle proof is not yet one reproducible receipt. Apple host is unreachable now. |
| Provider-neutral reconciliation | 15 | 8.0 | Stable BytePort ID, same-process replay, and changed-digest conflict are present; cross-process uniqueness, generation, and execution reconciliation are open. |
| Cross-component pilot | 15 | 3.0 | Component-level smokes exist; no authenticated current-head PhenoCompose -> BytePort -> NanoVMS transaction exists. |
| Governance, ownership, and evidence | 10 | 9.0 | Ownership, correlation contract, forward DAG, and substrate probe are recorded; required-check policy and stale worktree maintenance remain. |
| **Capability progress** | **100** | **65.8** | Heuristic only. **Hard release gate: 0/4 exact heads green; pilot: NOT RUN.** |

The older June scorecard (3.3/9 aggregate and D+/D- component grades) is
historical baseline data, not the current release score. The current score is
the exact-head capability score above and should be superseded only by a new
evidence snapshot.

## Commit-history synthesis

The remote histories show a coherent but incomplete dependency chain:

1. **BytePort** moved from provider boundary and mesh ownership docs through
   owner-scoped desired state, stable workload identity, idempotent replay,
   dependency remediation, and verified middleware identity aliases. The
   remaining work is repository-wide CI convergence plus cross-process
   reconciliation and an execution handoff that can be observed in a receipt.
2. **NanoVMS** added substrate probes, Podman lifecycle, fail-closed readiness,
   correlation preservation, authenticated daemon requests, and Trunk/tool
   bootstrap. The current hard blockers are Sonar new-code duplication and
   invalid Mergify configuration; a current-head inspect/readback receipt is
   still needed.
3. **PhenoCompose** added deterministic rendering, immutable render digests,
   BytePort/NanoVMS handoff types, Podman/Apple/WSL backend modeling, and WSLC
   resolution. Commit `7d321de` now adds a serializable deterministic bridge
   DTO and fixture (44 serde tests plus doctests pass); authenticated transport
   and live readback remain open.
4. **phenotype-infra** added the governance spine, correlation contract,
   forward-DAG/receipt documents, and a read-only capability probe. Its
   remaining repository gate is service coverage plus Markdown/Trunk and
   external Mergify policy.

## Forward DAG

```text
A0 exact evidence freeze
 | heads, ancestry, check snapshot, worktree inventory
 +----------------------+----------------------+
 |                      |
A1 CI/security         A2 contract lock
 |                      | digest, identity, generation,
 |                      | artifact, correlation, ownership
 +----------+-----------+
            |
      B1 exact-head component gates
      | focused tests + full CI + security + review
      +---------------------+
                            |
                    C1 substrate capability/readiness
                    | Podman positive; WSLC probe; Apple gated
                    +----------------------+
                                           |
                                  C2 authenticated bridge
                                  | PhenoCompose -> BytePort submit/readback
                                  | BytePort -> NanoVMS deploy/status
                                  +----------------------+
                                                                 |
                                                        D1 current-head pilot
                                                        | replay/conflict/failure/
                                                        | rollback receipts
                                                        +----------------------+
                                                                 |
                                                        D2 provider expansion
                                                        | Apple + WSLC lifecycle proofs
                                                        +----------------------+
                                                                 |
                                                        E1 publish/release
                                                        | attestations, SBOM, signed
                                                        | artifacts, required checks,
                                                        | deploy + rollback rehearsal
```

### Node contracts and exit evidence

| Node | Parallel owner(s) | Depends on | Exit gate | Current completion |
| --- | --- | --- | --- | ---: |
| A0 | infra governance | none | Exact SHAs, ancestry, check counts, and preserved worktree inventory recorded. | 100% |
| A1 | four repository owners | A0 | Every source/config failure fixed or explicitly external; exact-head CI/security rerun. | 55% |
| A2 | infra + BytePort + PhenoCompose + NanoVMS | A0 | One lowercase `sha256:<64 hex>` digest, stable workload ID + generation, immutable artifact reference, and correlation labels tested at each boundary. | 70% |
| B1 | four repository owners | A1, A2 | Focused tests exit 0; full checks green; approvals and required-check policy present. | 35% |
| C1 | NanoVMS + substrate lane | B1 | Podman-only disposable run proves readiness, deploy, inspect/readback, status, labels/digest, and cleanup. WSLC/Apple remain explicit capability states. | 50% |
| C2 | PhenoCompose + BytePort + NanoVMS | A2, C1 | Authenticated, non-test-double transport returns render digest, workload ID, provider/backend, sandbox ID, and status. | 20% |
| D1 | integration owner | C2, B1 | Append-only current-head receipt plus replay, changed-digest conflict, failed-start, and rollback evidence. | 0% |
| D2 | PhenoCompose + substrate owners | D1 | Apple Containers and WSLC each have a positive lifecycle proof or a probe-only signed exception. | 10% |
| E1 | infra governance/release | D1, D2 | Signed/attested artifacts, SBOM, deployment/publish rehearsal, rollback drill, and all four exact heads green. | 0% |

### Parallel execution lanes

| Window | Lane A: repository gates | Lane B: contract/pilot | Lane C: substrate/ops | Advance condition |
| --- | --- | --- | --- | --- |
| 0-4 h | Refresh exact checks; classify failures; preserve forks/worktrees. | Review PhenoCompose bridge DTO against BytePort/NanoVMS request schemas. | Keep Docker out; record Podman/WSLC/Apple capability states. | A1 failure ledger and A2 contract reviewable. |
| 4-24 h | Fix actionable lint/coverage/security; repair external policy configs through reviewed changes. | Add generation/cross-process idempotency and digest/identity contract tests. | Run bounded Podman readiness/inspect smoke on exact NanoVMS head. | B1 focused gates pass and no hidden required-check gap. |
| Day 2 | Rerun all four PRs at stable heads; obtain human approvals. | Implement authenticated bridge, capture request/response correlation. | Capture cleanup and failure semantics; keep Apple probe-only if offline. | C1 and C2 terminal receipts exist. |
| Day 3 | Recheck ancestry after every push. | Run current-head pilot, replay, conflict, failed-start, rollback. | Repeat on a clean disposable namespace; no Docker fallback. | D1 receipt is complete and reviewable. |
| Day 4-5 | Configure required checks and publish/rollback policy. | Provider matrix for AWS/GCP/Azure/Hetzner/Vercel/Supabase/Neon/Upstash/etc. behind the same desired-state contract. | Positive Apple/WSLC lifecycle proofs where hosts are reachable. | D2/E1, or explicit residual blockers with owners and next evidence. |

## Immediate next actions (ordered)

1. **Verify the PhenoCompose bridge contract** at `7d321de` against the live
   BytePort and NanoVMS schemas, then add the authenticated transport/readback;
   do not call the fixture a transport proof.
2. **Triage BytePort's 21 failures by class**: CodeQL/coverage/SBOM/frontend
   checks first, then Trunk/Go fmt and external Sonar/Mergify. Re-run at the
   current `aec8e356` head after each bounded fix.
3. **Close NanoVMS's two external blockers** (Sonar duplication and Mergify
   configuration) or record them as provider-owned exceptions; then capture
   inspect/readback evidence at `27f20df`.
4. **Raise phenotype-infra service coverage from 10.41% to >=60%**, fix
   Markdown/Trunk findings, and keep the read-only substrate probe as the
   adapter-discovery boundary.
5. **Run C1/C2 only after B1**: Podman disposable runtime, authenticated
   BytePort submit/readback, authenticated NanoVMS deploy/status, and one
   append-only correlation receipt.
6. **Treat Apple Containers as unverified on this host** until the Mac is
   reachable over the approved path; do not infer readiness from an old probe.
7. **Before E1**, configure GitHub required checks, preserve all forks/history,
   remove only explicitly approved stale worktree metadata, and publish signed
   artifacts with rollback evidence.

## Substrate research constraints

The adapter matrix follows current upstream behavior:

- [Podman container inspect](https://docs.podman.io/en/stable/markdown/podman-container-inspect.1.html)
  is the readback source for state, health, image identity, and exit status.
- [Apple's `container` project](https://github.com/apple/container) is an
  Apple-silicon/macOS lightweight-VM runtime consuming OCI images; its service
  lifecycle is macOS-only and must be capability-gated.
- [WSL's native container tooling](https://github.com/microsoft/WSL/releases)
  exposes the `wslc` command/SDK on Windows; `container.exe` is the local
  first-party command surface and must not be conflated with Apple `container`.

## Release rule

No merge, publish, or deployment claim is valid until the exact head being
claimed has: (1) green repository/security gates, (2) review approval and
configured required checks, (3) current-head C1/C2 evidence, and (4) a
correlatable D1 receipt containing render digest, workload ID, sandbox ID,
provider/backend, status, and UTC timestamps. The score above is a compass,
not permission to release.
