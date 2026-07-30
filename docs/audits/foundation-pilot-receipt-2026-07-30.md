# Phenotype foundation pilot receipt — 2026-07-30

Status: evidence receipt, not a release approval. Runtime evidence below is
explicitly identified as a host/runtime probe; the review-head table is the
authoritative snapshot observed 2026-07-30. No secrets, tokens, or private
addresses are recorded.

## Reviewed heads

| Component | Review surface | Exact head | Evidence |
| --- | --- | --- | --- |
| phenotype-infra | PR #125 | `35e4868b868215348ff5ef3e4d90f1a2d10af394` | Latest rollup: Rust/security/links/observability/IAC check pass; markdown-lint, Trunk Lint & Format, and IAC service coverage (10.41% vs 60%) fail; Mergify/Summary are external. |
| BytePort | PR #318 | `d005fdc893db97694e05b03dddae2898e5dcbcfd` | Latest rollup has mixed repository gates: Go/Rust lint/coverage, cargo-deny, CodeQL/Sonar, SBOM, a11y, and audit checks remain failing or queued; external Mergify/Summary also fail. |
| PhenoCompose | PR #113 | `cf847478a79396c6913d81847c487e15f05da244` | Latest rollup: security, CodeQL, Trivy, and supply-chain checks pass; Cargo audit/check/deny/clippy/test and Lint & Format remain failing or queued. |
| NanoVMS | PR #128 | `71f52a21d297bb8559d157dfdec7a1250e632a85` | Latest rollup: cross-compilation and security checks pass; Lint & Format, Dependency Review, Trivy, and configuration checks remain failing or queued. |

The exact heads above supersede the older commit identifiers in the runtime
notes below. Those notes are retained as reproducible pilot evidence, but do
not imply that the same runtime run was performed against every current PR
head.

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
| PhenoCompose submission adapter | PR #113 head `cf847478` exposes `Orchestrator` implementations for Noop, Helm, and ArgoCD; no BytePort HTTP client or `/mesh/workloads` transport is present. | Add and review a provider-neutral BytePort adapter that emits a deterministic composition/render digest. |
| BytePort control plane | PR #318 head `d005fdc` exposes authenticated `GET/POST /api/v1/mesh/workloads` and `GET /api/v1/health`; the live backend, Postgres, and a disposable auth token must be started for the run. | Start the exact head with isolated Postgres and capture health plus authenticated submit/read-back responses. |
| NanoVMS execution bridge | PR #128 head `71f52a21` exposes authenticated `POST /v1/deploy` and unauthenticated `/readyz` over configured UDS/TCP; it requires `serve --provider podman`, Podman, and a disposable token. | Start the exact head, verify readiness, and submit the BytePort-selected execution request through an explicit transport bridge. |
| Correlatable receipt | No current manifest or receipt maps a PhenoCompose render digest to a BytePort workload ID and NanoVMS sandbox ID. | Define and capture `{render_digest, workload_id, sandbox_id, provider, status, timestamps}` with no secrets or runtime-private addresses. |

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
