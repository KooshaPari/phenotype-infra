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

## Exact-head refresh: 2026-08-01 09:02-09:10 UTC

This refresh supersedes the older snapshot rows above for triage, while
preserving their history. The values below were read from the live pull
requests and their completed check rollups; a failure count is not a release
approval.

| Repository / PR | Exact head | State | Completed rollup | Blocking evidence |
| --- | --- | --- | --- | --- |
| BytePort #318 | `e6e684babe257e311678572db746487ed42527fc` | draft, blocked, review required | 56 success / 17 failure / 3 skipped | SBOM x2, Rust/Go coverage, lint/trunk, a11y/snapshots, npm audit, semver, Sonar; CodeQL default setup was disabled because advanced CodeQL is the authoritative workflow. |
| PhenoCompose #113 | `960a839c869bed7c9b248eb868871675ed7bb3e3` | draft, unstable | 26 success / 9 failure / 4 skipped | Root audit passes but the rust-ffi audit target has no lockfile; three unused serializers fail `-D warnings`; Android lane lacks `aarch64-linux-android-clang`; lint remains red. |
| NanoVMS #128 | `808beac1ee6cdb47aa62f32697c1fbe9e9114af4` | ready, blocked, review required | 16 success / 3 failure / 3 skipped | Repository gates are otherwise green; SonarCloud and Mergify/Summary remain external/policy blockers. Canceled jobs are not treated as green. |
| phenotype-infra #125 | `c4c6592bf1489837888b8b1af664187eb563439e` | draft, blocked, review required | 28 success / 5 failure / 8 skipped | Trunk/actionlint configuration, broad markdown baseline, and real IaC workspace coverage (10.41% vs 60%) remain red. |

### Substrate probe evidence

| Probe | Result | Interpretation |
| --- | --- | --- |
| NanoVMS `go test ./pkg/runtime ./pkg/orchestrate` | pass | Deterministic backend matrix, selection, digest handoff, and orchestration unit surface is locally healthy. |
| NanoVMS `go run ./cmd/nanovms` on the Windows host | `Platform: windows \| VM Tier: wsl \| Sandbox: none` | Host auto-selection is observable; this is not a workload readiness receipt. |
| Podman shim | present at `C:\Python313\podman.bat`, forwards to `podman-machine-default` | The command did not return within the bounded probe window; no Podman lifecycle claim is made. |
| WSL service | `WslService` and `vmcompute` running | `wsl --list --verbose` timed out; distro/readiness remains unverified. |
| Apple Containers | not reachable from this Windows host | Record as capability-gated/unproven until a macOS lane supplies a positive probe. |

These observations leave C1/C2/D1/E1 open. They are an evidence refresh, not a
merge, deployment, or pilot-complete claim.

## Follow-up head refresh: 2026-08-01 09:13 UTC

Two component PRs advanced after the preceding table. The authoritative heads
are now:

| Repository / PR | New exact head | Change since prior refresh | Current release implication |
| --- | --- | --- | --- |
| PhenoCompose #113 | `84510c24aa5f4b48c6ef7d0a8ec70656485a8dec` | Audit targets now use the committed root lockfile, Android setup exports the NDK clang toolchain, and serde-only serializer helpers are feature-gated. | CI is still settling; stable rustfmt diffs remain a separate gate. |
| NanoVMS #128 | `b2845c4442b853f4a94b5a73a858e4becb498ca9` | Removed duplicated sandbox test fixture lines and replaced unavailable `blacksmith-2vcpu-ubuntu-2204` labels with hosted runners. | SonarCloud quality gate is now green; hosted CI, Mergify, and review are still open. |

The earlier rows remain intentionally unchanged as an audit trail; no exact
head is promoted until its own current checks and required approval are green.

## Current-head refresh: 2026-08-01 23:48 UTC

This additive refresh records the heads after the focused remediation passes.
Hosted checks were still queued at capture time; local evidence is explicitly
separated from release evidence.

| Repository / PR | Exact head | Local evidence at refresh | Hosted release state |
| --- | --- | --- | --- |
| BytePort #318 | `1159b8175be9542226237d4b471f46c9373b7bd8` | 48/48 Playwright snapshots and 12/12 axe cases pass against the production preview; npm audit is 0 high/moderate. | Fresh gate/golangci/links/Semgrep checks queued; Sonar CPD rerun queued after targeted fixture exclusion; draft/review required. |
| PhenoCompose #113 | `04d226226f49e83e5908f429705188f3735f1dae` | Nightly fmt check, workspace clippy `-D warnings`, and locked workspace check pass locally. | Fresh hosted matrix queued; prior macOS/Linux checks are superseded and not counted. |
| NanoVMS #128 | `b2845c4442b853f4a94b5a73a858e4becb498ca9` | Existing runtime/orchestration unit evidence remains passing. | Repository checks green except external Mergify/Summary policy; review remains required. |
| phenotype-infra #125 | `c5fefbca66f5eecc112812639234b1c4a4491478` | Exact-head DAG and substrate probe record published. | Trunk/markdown/IaC coverage and policy gates remain open; draft/review required. |

The new BytePort and PhenoCompose heads are not promoted by this table: a
current green local run cannot substitute for exact-head hosted gates, policy,
approval, or the still-unproven C1/C2/D1 pilot receipts.

## Current-head refresh: 2026-08-02 00:12 UTC

This additive refresh records the latest published remediation heads. Local
tests and hosted checks remain separate evidence; no row is a release claim.

| Repository / PR | Exact head | Latest evidence | Hosted release state |
| --- | --- | --- | --- |
| BytePort #318 | `a3a5f5b4f3367c16cbfcdd886a5a0d553934ce26` | Backend focused and full Go tests plus `golangci-lint run ./...` pass locally; SonarCloud is green after fixture deduplication. | Gate, golangci, links, and review/policy checks are still queued. |
| PhenoCompose #113 | `04d226226f49e83e5908f429705188f3735f1dae` | Nightly fmt, workspace clippy `-D warnings`, and locked workspace check pass locally. | Hosted matrix remains queued; no promotion. |
| NanoVMS #128 | `b2845c4442b853f4a94b5a73a858e4becb498ca9` | Existing runtime/orchestration and cross-target checks remain green. | Repository gates are green except external Mergify/Summary policy. |
| phenotype-infra #125 | `64193b316f1fb2846ecda78746b4e135e2662eca` | Sonar blockers fixed: trunk action pinned to a full SHA and probe timeout catch now reports failure context. | Fresh Sonar/links/Semgrep checks are settling; review and coverage policy remain open. |

The foundation still lacks an authenticated, non-test-double PhenoCompose to
BytePort to NanoVMS C1/C2 transaction and real Podman/WSL/Apple substrate
receipts; these remain the next release gates.

### Local substrate probe (2026-08-02 00:18 UTC)

On the Windows host, the native Podman client is installed (`5.8.3`) and the
`podman-machine-default` WSL machine is present but stopped. A bounded
`podman machine start podman-machine-default` attempt failed after the WSL
bootstrap timeout (`Wsl/Service/CreateInstance/0x800705b4`); `podman info`
therefore cannot reach the Linux socket. The host also has an unrelated,
long-running Fedora WSL inference process, which was preserved. This is an
unverified substrate blocker, not a release or pilot receipt.

### Current-head and hosted-run refresh (2026-08-02 01:09 UTC)

Manual CI was dispatched against each current component head to obtain fresh
hosted evidence. Queued is intentionally recorded as pending, not green.

| Repository / PR | Exact head | Manual hosted run | State at capture |
| --- | --- | --- | --- |
| PhenoCompose #113 / CI | `cea81b78f30ec679d711cdd00cb4c3d24aed03c3` | [30726541057](https://github.com/KooshaPari/PhenoCompose/actions/runs/30726541057) | queued |
| PhenoCompose #113 / Rust CI | `cea81b78f30ec679d711cdd00cb4c3d24aed03c3` | [30726541818](https://github.com/KooshaPari/PhenoCompose/actions/runs/30726541818) | queued |
| BytePort #318 / CI | `a3a5f5b4f3367c16cbfcdd886a5a0d553934ce26` | [30726542678](https://github.com/KooshaPari/BytePort/actions/runs/30726542678) | queued |
| phenotype-infra #125 / CI | `37655c6712f24325ba2a4dc4dc2b912b2122141a` | [30726543547](https://github.com/KooshaPari/phenotype-infra/actions/runs/30726543547) | queued |
| NanoVMS #128 | `b2845c4442b853f4a94b5a73a858e4becb498ca9` | no manual dispatch required | repository gates green; external Mergify/Summary and review remain open |

These runs are evidence collection only. C1/C2/D1/E1 remain open until the
exact heads have completed required checks and the authenticated pilot and
substrate receipts exist.

### Read-only substrate capability refresh (2026-08-02 01:28 UTC)

The exact-head capability probe was run on `fafc993` with credentials and
runtime state untouched. It distinguishes command presence from lifecycle
readiness.

| Substrate | Status | Evidence |
| --- | --- | --- |
| Podman | installed_unavailable | `C:\Python313\podman.bat` is present; the read-only probe skipped shim invocation. A separate bounded machine-start probe still hits WSL `0x800705b4`. |
| First-party WSL Containers | available | `C:\Program Files\WSL\container.exe`, `wslc 2.9.3.0`. |
| WSL host | available | `C:\Program Files\WSL\wsl.exe`; FedoraLinux-44 default WSL 2 distribution. |
| Apple Containers | not probed on this Windows host | Requires the currently unreachable macOS lane. |

This is positive adapter evidence for WSL Containers, not a Podman workload
readiness or C1/C2 pilot receipt.

### Exact-head refresh (2026-08-02 01:43 UTC)

PhenoCompose advanced after its standalone CLI lockfile was refreshed. The
strict crate gate passed locally (`cargo test --manifest-path
crates/phenocompose-cli/Cargo.toml --locked`, 12 tests). Fresh hosted runs are
pending on the new SHA.

| Repository / PR | Exact head | Current evidence |
| --- | --- | --- |
| PhenoCompose #113 | `9e70c7252db40e5869e040f8b0ca3ede53d29748` | CLI locked test passed; CI [30727554873](https://github.com/KooshaPari/PhenoCompose/actions/runs/30727554873) and Rust CI [30727555722](https://github.com/KooshaPari/PhenoCompose/actions/runs/30727555722) pending. |
| BytePort #318 | `a3a5f5b4f3367c16cbfcdd886a5a0d553934ce26` | Local Go tests/lint and Sonar remain green; hosted gates pending. |
| NanoVMS #128 | `b2845c4442b853f4a94b5a73a858e4becb498ca9` | Full local Go package suite clean; repository security gates previously green. |
| phenotype-infra #125 | `5c983eb8163dde8b81721af62975685d33ff2c95` | Governance and substrate probes pass; CI [30727152075](https://github.com/KooshaPari/phenotype-infra/actions/runs/30727152075) is superseded by this refresh if the SHA changes again. |

The shared Mergify base-policy repairs are published as PhenoCompose #114,
BytePort #320, phenotype-infra #129, and NanoVMS #129; their Mergify and
Summary checks are still in progress and require review before merge.

### Native WSL Podman runtime receipt (2026-08-02 02:38 UTC)

The Windows Podman Desktop machine remains unavailable, but the already-running
`FedoraLinux-44` WSL 2 distribution has a healthy native Podman lane. The
unrelated Fedora workload was preserved.

| Receipt field | Value |
| --- | --- |
| Host | `FedoraLinux-44` WSL 2 |
| Podman | `5.8.4`; `podman info` returned Linux host data successfully |
| Image | `quay.io/podman/hello:latest` |
| Image digest | `sha256:43de9874507eaa8ffd88eac885b672b1dfc57cc583d9ad920850f97f19809f8f` |
| Container ID | `674a015eec4a8374afb158d49caa12d9c8fb936e4f1434ebef09578cd1b7d49a` |
| Result | `exited`, exit code `0` |
| Correlation labels | `phenocompose.name=phenotype-lab`, `phenocompose.sha256=69b4f35ff771775f0a8f4c32d2bcfa68b778e79da4be1aa636caed1c3a2c899e`, `nvms.backend=podman` |
| Cleanup | Container removed after inspection |

This is a real substrate receipt and a valid artifact/correlation handoff
candidate. It is not yet an authenticated BytePort acceptance or NanoVMS
deployment receipt; C1/C2 remains open until those two service-side receipts
are collected against the same composition and artifact digests.

### Exact-head full-suite receipts (2026-08-02 02:43 UTC)

The published feature heads were checked in disposable worktrees from the
exact SHAs below. These are local reproducibility receipts; hosted checks,
review, and deployment evidence remain separate gates.

| Repository / PR | Exact head | Command | Result |
| --- | --- | --- | --- |
| BytePort #318 | `a3a5f5b4f3367c16cbfcdd886a5a0d553934ce26` | `go test ./...` from `backend/` | exit `0`; every Go package passed, including `mesh`, `meshworkload`, `container`, HTTP handlers, persistence, auth, secrets, and cloud adapters |
| NanoVMS #128 | `b2845c4442b853f4a94b5a73a858e4becb498ca9` | `go test ./...` | exit `0`; all adapters, API/config/domain, orchestration, runtime, resilience, and tier packages passed |

The BytePort receipt verifies the authenticated desired-state route and its
container wiring compile and pass together. The NanoVMS receipt verifies the
execution-side composition handoff and backend matrix against the same
published feature head. Neither receipt claims a live network transaction;
the authenticated BytePort-to-NanoVMS C1/C2 pilot remains open.

### Conflict reconciliation receipt (2026-08-02 02:47 UTC)

BytePort and NanoVMS each had one content conflict against their current
`main`, limited to `.github/workflows/ci.yml`. The conflicts were resolved in
isolated worktrees and pushed as additive merge commits; no application files
were discarded.

| Repository / PR | Reconciliation head | Resolution | Mergeability after push |
| --- | --- | --- | --- |
| BytePort #318 | `5c0f63511fdb25845ed5d0aef517b471858cdea1` | Retained current `main`'s stable `ci / lint` and `ci / test` workflow gates; mesh implementation remains unchanged. | `MERGEABLE`; hosted checks queued, review required |
| NanoVMS #128 | `d5920941e2bd04178f4111dc0f9cce723fcff460` | Retained the feature branch's hosted-runner change; conflict was runner-only plus newline. | `MERGEABLE`; hosted checks queued, review required |

These commits remove the GitHub conflict gate but do not waive required
review, Mergify/Summary policy, or hosted CI completion.

### Hosted-gate classification and workflow syntax receipt (2026-08-02 02:52 UTC)

The latest published heads are mergeable but intentionally not release-ready:

| Repository / PR | Exact head | Mergeability | Completed failures | Pending gate |
| --- | --- | --- | --- | --- |
| PhenoCompose #113 | `582929bb553e8f0e865f92e1a85700a3c8d8f82f` | mergeable / blocked | Mergify, Summary, SonarCloud | review plus 23 queued checks |
| BytePort #318 | `0dc859366a2c98503976045e07a741bb8ea447f2` | mergeable / blocked | Mergify, Summary | review plus 50 queued and 2 in-progress checks |
| NanoVMS #128 | `d5920941e2bd04178f4111dc0f9cce723fcff460` | mergeable / blocked | Mergify, Summary, SonarCloud | review plus 4 queued checks |
| phenotype-infra #125 | `a853ffe218e6c1eea5179d50d314051146127eb1` | mergeable / blocked | Mergify, Summary, SonarCloud | review plus 18 queued checks |

BytePort's follow-up CI correction (`0dc8593`) is actionlint-clean: the
detector no longer reads its own step outputs during execution, all
`GITHUB_OUTPUT` writes are safely quoted, and the aggregate gate references
the actual `dep-review` job. NanoVMS' reconciled workflow is also actionlint-
clean. These are syntax and configuration receipts, not hosted pass claims.
