# Foundation forward DAG and exact-head scorecard - 2026-07-31 07:24 UTC

Status: current execution plan and evidence snapshot. This is not a merge,
publish, deployment, or release approval. The check counts below are a
point-in-time GitHub observation tied to the exact heads; refresh after every
push, merge, workflow rerun, or policy change.

## Scope and ownership

| Layer | Repository | Boundary |
| --- | --- | --- |
| Spine and governance | `phenotype-infra` | ADRs, ownership, IaC conventions, receipts, release policy |
| Provider-neutral control plane | `BytePort` | desired state, workload identity, reconciliation, provider/IaC adapters |
| Composition and translation | `PhenoCompose` | deterministic compose/proc/kube rendering, digest, substrate selection |
| Execution and lifecycle | `NanoVMS` | sandbox/container lifecycle, readiness, inspect, status, audit correlation |

`sharecli`, `phenodag`, and substrate implementations stay separately owned.

## 08:02 UTC exact-head refresh

The bounded production-auth change is now on BytePort #318 as
`d396513196018012cc556935c86339af407f253e`. It keeps the legacy synthetic
token path behind the test constructor, adds fail-closed WorkOS startup, and
adds startup/rejection tests. GitHub's new run proves Go modules, build, vet,
Rust coverage, and SBOM generation; the advisory Go test still exits 1, while
Go framework coverage, service/E2E setup, a11y, frontend lint, npm audit,
semver, Sonar, Mergify/Summary, and the CodeQL configuration surface remain
unresolved or active. The exact head is therefore still blocked and must not
be called release-ready.

The other frozen heads remain `808beac1` (NanoVMS #128), `18365987`
(PhenoCompose #113), and `ea630658` (phenotype-infra #125). NanoVMS,
PhenoCompose, and phenotype-infra have no newly green release gate in the
08:02 refresh. The Podman runbook correction is already present in infra; no
Docker command is part of the intended substrate path.

This refresh does not change the 66.1/100 capability-progress baseline above:
that number is still heuristic and provisional while BytePort's rerun is
active. The hard release state remains 0/4 exact heads green, with C1/C2/D1
unproven. The next DAG edge is to finish source/config CI convergence, then
run the authenticated current-head bridge; no provider expansion or publish
step may bypass those gates.
Docker is not a foundation dependency. Podman is the first positive local
substrate; first-party WSL Containers (`wslc`/`container.exe`) and Apple
Containers are capability-gated adapters.

## Exact remote state

GitHub compare calls show all four selected PRs are linear descendants of
their current `main` (zero commits behind). Local primary checkouts and many
worktrees are stale or divergent; integration must use detached worktrees
created from these exact SHAs and must preserve every sibling ref.

| Repo / PR | Branch | Exact head | Base main | Ahead/behind | Review / merge state | Rollup |
| --- | --- | --- | --- | --- | --- | --- |
| BytePort #318 | `codex/byteport-workflow-syntax-pilot` | `b17b124541affe7b39d7f76dab80cc813c39ea13` | `da128172664c5c8b10f5b52cc5edfddf4e1689fb` | 78 / 0 | draft; blocked | 51 pass, 20 fail, 1 neutral, 3 skipped, 2 queued, 1 unknown |
| NanoVMS #128 | `codex/nanovms-podman-provider` | `808beac1ee6cdb47aa62f32697c1fbe9e9114af4` | `4b7188d0` (tombstone main) | 45 / 0 | ready; blocked | 15 pass, 3 fail, 3 skipped, 3 queued |
| PhenoCompose #113 | `codex/phenocompose-pr112-workflow-fixes` | `18365987896f08f20af45b6fe8d905e41b44ef90` | `aae65521` | 42 / 0 | draft; unstable | 24 pass, 10 fail, 4 skipped, 2 queued |
| phenotype-infra #125 | `codex/phenotype-infra-pr115-docs-gates` | `0c369f4416eafc5be9db09cd16c4d3508dadc84e` | `5f68ebce` | 37 / 0 | draft; blocked | 28 pass, 5 fail, 8 skipped, 1 unknown |

NanoVMS #128 contains an intentional merge from `main` and the archived
tombstone ancestry; do not squash or rebase it. BytePort #317/#319/#313/#316
overlap the selected PR on 65-86 paths and diverge from a common base. Pheno
PRs 107, 110, 111, and 112 overlap the contract files in PR 113. Infra #125 covers the
same governance paths as #115 on current main. These are preserved alternatives,
not serial merge inputs.

## Failure ledger (exact-head evidence)

| Repo | Actionable source/config work | External, transient, or policy work |
| --- | --- | --- |
| BytePort | Frontend Prettier reports 46 files; Go framework coverage is 62.5% vs 70%; missing visual baselines; serious/critical axe violations; cargo-semver has no baseline for `app@0.1.0`; SBOM validation searches the wrong output path. | CodeQL advanced/default setup conflict; Sonar automatic/CI conflict; setup-only Service/E2E and npm-audit failures with no logs; CI Security/Trunk setup noise; invalid base Mergify config. |
| NanoVMS | Podman lifecycle gates are green; remaining functional hardening is digest-label lookup before create, cross-process idempotency, and not-found convergence for stop/delete. | SonarCloud duplication; current base Mergify Summary/Queue invalid. The branch's syntax-valid Mergify config is only effective after merge. |
| PhenoCompose | Workspace `cargo fmt` debt; `map_is_empty`, `vec_is_empty`, `is_false` unused under `-D warnings`; Android linker PATH lacks `aarch64-linux-android-clang`; action refs are mutable. | cargo-audit/cargo-deny advisory parser rejects CVSS 4.0 (RUSTSEC-2026-0109); Trunk bootstrap/tool annotation failures; invalid base Mergify config. The foundation-pilot fixtures are placeholders, not a receipt. |
| phenotype-infra | IaC coverage is 10.41% (76/730) vs 60%; broad Markdown baseline debt; Trunk bootstrap/toolchain failures; OCI runbook still contains Docker commands and must be Podman-only. | Effective required-check policy is ambiguous: branch-protection probes expose stale `ci / lint` and `ci / test` contexts while active rulesets are weak; invalid base Mergify config. |

No failure may be dismissed as “green enough” without a rerun at the exact
head or a recorded provider-owned exception. Setup-only failures must be
rerun before being counted as code failures.

## Current capability score

CI credit is `success / (success + failure + neutral)`, with skipped, queued,
and unknown checks earning no credit. It is a capability compass, not a
release probability.

| Pillar | Weight | Score | Evidence / missing proof |
| --- | ---: | ---: | --- |
| Exact ancestry and provenance | 10 | 10.0 | Exact heads, ancestry, sibling overlap, and preserved worktree inventory recorded. |
| CI and security convergence | 35 | 27.1 | BytePort 70.8%, NanoVMS 83.3%, PhenoCompose 70.6%, infra 84.8% completed-check credit; source and policy failures remain. |
| Runtime/substrate capability | 15 | 9.0 | Podman code and probes exist; no current-head inspect/readback receipt; Apple host unavailable; WSLC is probe-only. |
| Provider-neutral reconciliation | 15 | 8.0 | Stable owner/name/digest replay exists in-process; DB uniqueness, generation, and cross-process reconciliation remain open. |
| Cross-component pilot | 15 | 3.0 | DTO/fixture tests exist; no authenticated PhenoCompose -> BytePort -> NanoVMS transaction. |
| Governance and evidence | 10 | 9.0 | Ownership/correlation docs exist; effective rulesets, required contexts, action pinning, and release verifier remain open. |
| **Capability progress** | **100** | **66.1** | Heuristic only. Hard release state: 0/4 exact heads green; C1/C2/D1 not run. |

## Forward DAG

```text
A0 Freeze exact evidence and preserve refs/worktrees
 | heads, bases, ancestry, checks, dirty/divergent inventory
 +---------------------------+---------------------------+
 |                           |                           |
A1 CI/security ledger    A2 Contract lock            A3 History boundary
 | source vs external    | digest, artifact,          | choose canonical PRs;
 | and exact reruns       | identity, generation,      | preserve sibling refs
 |                        | correlation, secret refs  |
 +-------------+----------+---------------------------+
               |
B1 Component gates and policy convergence
 | focused tests, full CI, security, required checks,
 | approvals, immutable workflow action refs
 +---------------------------+
               |
C1 Substrate readiness (NanoVMS first)
 | Podman create/start/inspect/readiness/cleanup;
 | WSLC probe; Apple capability-gated
 +---------------------------+
               |
C2 Authenticated bridge
 | PhenoCompose render -> BytePort submit/readback ->
 | NanoVMS deploy/status, one correlation id
 +---------------------------+
               |
D1 Current-head pilot and failure semantics
 | replay, changed-digest conflict, failed start,
 | timeout, cleanup, rollback; append-only receipt
 +---------------------------+
               |
D2 Provider and substrate expansion
 | AWS/GCP/Azure/Hetzner/Vercel/Supabase/Neon/Upstash
 | plus positive WSLC/Apple proofs where reachable
 +---------------------------+
               |
E1 Publish/release
 | signed artifacts, SBOM + verified attestation,
 | deployment/rollback rehearsal, green exact heads
```

### Node contracts and exit evidence

| Node | Depends on | Owner(s) | Exit evidence | Current |
| --- | --- | --- | --- | ---: |
| A0 | none | infra | SHA/base/check/worktree ledger; no destructive cleanup | 100% |
| A1 | A0 | all repo owners | Every red check fixed, rerun, or explicitly external with owner | 55% |
| A2 | A0 | infra + three product repos | One lowercase `sha256:<64 hex>`, stable workload ID + generation, immutable artifact ref, secret-ref-only payload | 70% |
| A3 | A0 | integration owner | #318/#128/#113/#125 authoritative; sibling refs mapped to cherry-pick reviews | 80% |
| B1 | A1-A3 | all repo owners | Focused and full gates green; effective required contexts and one approval verified | 35% |
| C1 | B1 | NanoVMS + substrate owners | Disposable Podman readiness/inspect/readback/cleanup receipt; WSLC/Apple explicit capability state | 50% |
| C2 | A2 + C1 | PhenoCompose + BytePort + NanoVMS | Authenticated non-test-double request/response with digest, workload, backend, sandbox, status, UTC timestamps | 20% |
| D1 | B1 + C2 | integration owner | Append-only receipt proves replay, conflict, failed-start, timeout, cleanup, rollback | 0% |
| D2 | D1 | provider/substrate owners | Per-provider desired-state adapter contract; positive WSLC/Apple proof or signed exception | 10% |
| E1 | D1 + D2 | infra release owner | Signed/attested artifacts, SBOM verification, deployment + rollback drill, green exact heads | 0% |

## Five-day execution ticks

| Tick | Parallel work | Advance condition |
| --- | --- | --- |
| 0-4h | Freeze refs; classify failures; choose #318/#128/#113/#125; inspect effective rulesets. | A0/A3 ledger reviewable; no hidden required-check mismatch. |
| 4-12h | Nano Podman idempotency/readback tests; BytePort rerun setup-only jobs and repair SBOM/CodeQL policy; Pheno parser/toolchain lane; infra OCI runbook correction. | Source-vs-external ledger updated; no unverified green claim. |
| 12-24h | BytePort coverage/a11y/snapshot fixes; Pheno fmt/clippy/Android; infra focused IaC tests and deterministic Trunk bootstrap; pin mutable action refs. | B1 focused gates pass on exact heads. |
| Day 2 | Obtain one approval per protected repo; reconcile required contexts/rulesets; rerun all four PRs. | B1 full gates and policy are green/effective. |
| Day 3 | Run C1 Podman disposable lifecycle, C2 authenticated bridge, persist correlation receipt. | C1/C2 receipts contain digest, workload, sandbox, backend, status. |
| Day 4 | D1 replay/conflict/failure/rollback; clean disposable rerun; keep Apple offline as signed capability exception if needed. | D1 receipt is append-only and independently reviewable. |
| Day 5 | D2 provider matrix and reachable WSLC/Apple proofs; signed SBOM/attestation verification; deployment/rollback rehearsal. | E1 only if every release rule is satisfied; otherwise publish residual blockers with owners. |

## Safe integration order

1. Preserve every branch, tag, dirty worktree, and tombstone. Never reset,
   squash, or delete sibling history as an integration shortcut.
2. Make NanoVMS #128 the lifecycle substrate baseline; land only after its
   current-base policy and Sonar gates are resolved or formally excepted.
3. Treat BytePort #318 as the sole control-plane stack. Extract unique fixes
   from #317/#319/#313/#316 only by reviewed patch, never by serial merge.
4. Canonicalize PhenoCompose #113 contract/adapters, then reconcile renderer
   PRs #107/#110/#111/#112 against that canonical port-types surface.
5. Use phenotype-infra #125 as the current-main successor to #115; preserve
   #115 and review only non-overlapping additions.
6. Refresh exact heads, checks, ancestry, required contexts, and receipts after
   each merge. A green old SHA is not evidence for a new SHA.

## Non-negotiable preconditions discovered after the snapshot

- **Real authentication before C2:** BytePort's production route currently
  constructs `AuthMiddlewareWithFallback(nil)`. Its fallback accepts only
  synthetic `test-*`/`mock-*` tokens, so an authenticated pilot cannot pass
  until a configured WorkOS auth service is injected, fail-closed behavior is
  tested, and a scoped non-test token is exercised.
- **Podman is MVP, not yet a production sandbox:** NanoVMS must validate image
  digests, rootless/user-namespace policy, mount allowlists, environment
  redaction, network/resource limits, deterministic digest labels, and
  cross-process replay before E1.
- **Release verification is currently non-cryptographic:** BytePort's
  attestation workflows pass literal shell syntax instead of a computed
  digest, and its verifier does not run `gh attestation verify`. PhenoCompose
  and NanoVMS likewise lack a fail-closed artifact/SBOM/attestation verifier.
  E1 must require non-empty expected binaries, checksums, SBOM linkage,
  verified attestations, and rollback evidence.
- **Ownership policy is not independent:** CODEOWNERS is effectively
  `@KooshaPari` across the foundation and active rulesets expose weak or
  mismatched required contexts. Add subsystem owners or an explicit
  second-person release/security/IaC approval before publish.

## Release rule

No merge, publish, deployment, or “pilot complete” claim is valid until the
claimed exact head has green repository/security gates, effective required
checks and approval, a current-head C1/C2 receipt, and a D1 receipt containing
composition digest, artifact reference, BytePort intent ID, Nano correlation
ID, provider/backend, status, and UTC timestamps. Provider expansion is an
adapter matrix behind the same desired-state contract, not a reason to bypass
the foundation gates.
