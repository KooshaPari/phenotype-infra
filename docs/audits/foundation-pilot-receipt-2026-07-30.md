# Phenotype foundation pilot receipt — 2026-07-30

Status: evidence receipt, not a release approval. All evidence below is
head-specific or explicitly identified as a host/runtime probe. No secrets,
tokens, or private addresses are recorded.

## Reviewed heads

| Component | Review surface | Exact head | Evidence |
| --- | --- | --- | --- |
| phenotype-infra | PR #125 | `f5ce6e26b0ba4030f826ea9ee995437028bdbb51` | Exact-head CI run `30509150577` passed Rust fmt/Clippy/build/tests, security, Trivy/SARIF, and aggregate; SonarCloud and Semgrep passed. |
| BytePort | PR #318 | `383d9695` | Focused Go package gates, CodeQL, Semgrep, Snyk, links, and security checks passed. Sonar remains an external project-configuration gate; see blockers. |
| PhenoCompose | PR #113 | `d514ac9` | YAML/action checks, lockfile enforcement, focused Cargo tests, tier1/tests, Sonar, CodeQL, Semgrep, GitGuardian, and Socket checks passed. |
| NanoVMS | PR #123 | `737f8b5` | Lifecycle normalization (`Start`/`running`/`failed`) and focused API tests passed; a subsequent Podman provider is tracked separately in PR #128. |

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
