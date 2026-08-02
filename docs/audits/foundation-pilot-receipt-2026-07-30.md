# Phenotype foundation pilot receipt — 2026-07-31

Status: evidence receipt, not a release approval. Runtime evidence below is
explicitly identified as a host/runtime probe; the review-head table is the
authoritative snapshot observed 2026-07-31. No secrets, tokens, or private
addresses are recorded.

## Reviewed heads

| Component | Review surface | Exact head | Evidence |
| --- | --- | --- | --- |
| phenotype-infra | PR #125 | `64d33ad` (receipt parent; this update is evidence-only) | 28 success / 5 failure / 0 pending / 8 skipped; markdown-lint, Lint & Format, and IAC service coverage (10.41% vs 60%) fail; Mergify/Summary are external. |
| BytePort | PR #318 | `097b8ae` | Fresh push checks observed 4 success / 2 failure / 56 pending / 1 skipped; the CI fan-out is still running. Stable mesh workload `id`, same-process replay, and changed-digest conflict handling are now present; external/security gates remain open. |
| PhenoCompose | PR #113 | `fa7ad4e` | 24 success / 10 failure / 2 pending / 4 skipped; Cargo audit/check/deny/clippy, Trunk, and external gates remain open. |
| NanoVMS | PR #128 | `3723b47` | 13 success / 5 failure / 3 pending / 3 skipped; Trunk/configuration, Kilo, and external gates remain open. Lifecycle labels, fail-closed readiness, and status/request-ID audit evidence are now present. |

The exact heads above supersede all older commit identifiers in the runtime
notes below. Those notes are retained as reproducible pilot evidence, but do
not imply that the same runtime run was performed against every current PR
head.

## Current scorecard (observed progress, not release approval)

The check counts above are descriptive because GitHub exposes multiple checks
per workflow and some are external automation. The weighted progress score is
therefore separate from the hard release gate:

| Area | Weight | Current | Evidence basis |
| --- | ---: | ---: | --- |
| Exact branch ancestry and review surfaces | 10 | 10 | All four PR heads are mergeable and 0 commits behind their protected main base. |
| CI and security convergence | 35 | 25 | Current completed-check ratios are approximately 84.8%, 62.0%, 70.6%, and 72.2%; required failures remain. |
| Runtime probes and substrate adapters | 15 | 9 | Podman smoke plus Apple/WSL host probes exist; current-head adapter wiring is incomplete. |
| Provider-neutral reconciliation | 15 | 8 | BytePort validates desired intent, exposes a stable persisted workload ID, and now tests same-process replay/conflict behavior; cross-process uniqueness, generation, and execution reconciliation remain unimplemented. |
| Cross-component pilot | 15 | 3 | Separate component smokes exist; no authenticated three-hop transaction or complete receipt exists. |
| Governance, ownership, and evidence | 10 | 9 | Ownership matrix, correlation contract, and this exact-head receipt are present; historical ADR/provider inventory debt remains. |
| **Progress score** | **100** | **64** | Capability progress only; not a merge or release decision. BytePort's new-head checks are still pending. |

**Strict release gate: 0/4 exact heads green.** The score must not be read as
publication readiness; all required checks, review approvals, and the live
pilot remain mandatory.

## Cross-component runtime evidence

### BytePort mesh control plane

- PostgreSQL 16 was launched in the Podman substrate and BytePort migrations
  completed.
- BytePort `/api/v1/health` returned HTTP 200.
- Authenticated `POST /api/v1/mesh/workloads` returned HTTP 202.
- Authenticated `GET /api/v1/mesh/workloads` returned the persisted workload
  and preserved `region`, `zone`, `node_pool`, labels, and constraints after
  the placement round-trip fix (`383d9695`).

### NanoVMS UDS and Podman execution

- Exact NanoVMS lifecycle binary `e023138` was cross-built as a static Linux
  x86-64 ELF: 10,389,054 bytes,
  SHA-256 `81AF2BF54C08B12C21335D9E4F412790793ECFBCCB206C60518A3692E41961CE`.
- In FedoraLinux-44 WSL, the daemon logged UDS startup and
  `curl --unix-socket ... /readyz` returned HTTP 200 with `{"status":"ready"}`.
- NanoVMS PR #128 (`30d5e45`) adds an explicit `serve --provider podman`
  provider. FedoraLinux-44 WSL Podman 5.8.4 (rootless, crun) pulled
  `alpine:latest`; an authenticated deploy returned HTTP 201 with sandbox
  short ID `beed2053e6d6` (the full runtime ID is intentionally omitted), status
  `running`, and type `container`. `GET /v1/sandboxes` returned the same
  running sandbox and `podman ps` confirmed it; the container was removed
  after the smoke.

The Podman provider is intentionally separate from NanoVMS tier 1–3 defaults.
It uses Podman-created IDs and inspect-backed state; an immediate container
exit is surfaced as a failed start rather than normalized to running.

## Capability probes

| Runtime | Probe result | Boundary |
| --- | --- | --- |
| Podman | FedoraLinux-44 WSL: Podman 5.8.4, rootless, crun; disposable Alpine run and NanoVMS provider smoke passed. | NanoVMS explicit Podman provider; no Docker dependency. |
| Apple Containers | Tailscale/SSH probe on `kooshas-laptop`: `/usr/local/bin/container`, version `1.0.0`, Darwin ARM64. | PhenoCompose Apple adapter; macOS-only execution surface. |
| WSL Containers | Windows first-party `C:\Program Files\WSL\container.exe`, `wslc 2.9.3.0`; PhenoCompose adapter commit `d514ac9` prefers `container.exe` and falls back to legacy `wslc.exe`. | PhenoCompose WSL adapter; compatibility fallback only. |

## Current-head end-to-end pilot gate

**Status: NOT RUN.** The separate runtime probes above do not prove a
PhenoCompose -> BytePort -> NanoVMS transaction. A current-head audit found no
safe way to run that transaction without introducing an integration bridge or
silently substituting a test double.

| Required prerequisite | Current-head evidence | Blocking action before pilot |
| --- | --- | --- |
| PhenoCompose submission adapter | PR #113 head `f7313c8` has deterministic render and pure handoff types, but no BytePort HTTP client or `/mesh/workloads` transport. | Add and review a provider-neutral BytePort adapter that emits a deterministic composition/render digest. |
| BytePort control plane | PR #318 head `8c65c49` exposes authenticated `GET/POST /api/v1/mesh/workloads` and now returns a stable workload `id`; the live backend, Postgres, and a disposable auth token must be started for the run. | Start the exact head with isolated Postgres and capture health plus authenticated submit/read-back responses. |
| NanoVMS execution bridge | PR #128 head `3723b47` exposes `nvms serve --provider podman`, authenticated deploy, fail-closed readiness, and correlation labels/audit status; it still requires an explicit transport bridge. | Start the exact head, verify readiness, and submit the BytePort-selected execution request through an explicit bridge. |
| Correlatable receipt | BytePort workload identity and NanoVMS container correlation labels now exist, but no current manifest maps render digest to both workload and sandbox IDs. | Define and capture `{render_digest, workload_id, sandbox_id, provider, status, timestamps}` with no secrets or runtime-private addresses. |

Until all four rows have head-specific evidence, the foundation remains
pilot-ready only in separate component probes and is not end-to-end release
approved.

## Ownership and safety boundaries

- `phenotype-infra` remains the organization infrastructure spine and ADR /
  governance source.
- BytePort owns provider-neutral deployment/IaC and compute-mesh control-plane
  state.
- PhenoCompose owns compose/orchestration translation and runtime adapters.
- NanoVMS owns sandbox execution and lifecycle APIs.
- Substrate, `sharecli`, and `phenodag` ownership was not moved or absorbed by
  this receipt. No destructive cleanup, history rewrite, or fork deletion was
  performed.

## External gates still open

- BytePort SonarCloud Automatic Analysis is enabled for the project, so CI
  analysis cannot be used as the project gate. The current PR reports 3.6%
  new-code duplication against a 3% threshold; disabling Automatic Analysis
  at the Sonar project level and rerunning the exact head is required.
- Mergify and Summary checks report automation failures on the review PRs;
  these are not repository test evidence.
- Kilo review is external automation and may remain pending; it is not used as
  a substitute for repository tests or security gates.
