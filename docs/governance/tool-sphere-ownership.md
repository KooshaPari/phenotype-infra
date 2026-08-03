# Tool-sphere ownership

This document records the boundary for repositories that coordinate work
across the foundation. These tools may discover resources, build plans, or
invoke an owning API; they must not become a second cloud-state or runtime
control plane.

## Ownership matrix

| Tool sphere | Field | Contract |
| --- | --- | --- |
| `substrate` | Owns | host discovery, capabilities, transport adapters |
| `substrate` | Must not own | credentials/state, composition, lifecycle |
| `substrate` | Handoff | BytePort for IaC; NanoVMS for execution |
| `substrate` | Evidence | capability snapshot, adapter version, probe |
| `sharecli` | Owns | cross-repository commands, auth routing, and receipts |
| `sharecli` | Must not own | CRUD, deploy truth, secrets, renderer policy |
| `sharecli` | Handoff | the owning BytePort, PhenoCompose, or NanoVMS API |
| `sharecli` | Evidence | command, target, receipt ID, exit status, UTC time |
| `phenodag` | Owns | graphs, ordering, retries, orchestration receipts |
| `phenodag` | Must not own | cloud state, credentials, images, or lifecycle |
| `phenodag` | Handoff | BytePort status; PhenoCompose render; NanoVMS run |
| `phenodag` | Evidence | graph digest, outcomes, retries, and owner receipt |

The owning component remains authoritative after a handoff:

- **BytePort** owns AWS-like infrastructure plans, provider state, and apply
  authorization.
- **PhenoCompose** owns composition manifests and deterministic target renders.
- **NanoVMS** owns execution-engine selection, lifecycle, health, and failures.
- **phenotype-infra** owns this cross-repository policy, inventory, and
  runbooks; it does not copy mutable state from those components.

## Execution substrate extensions

These substrates are runtime targets behind the NanoVMS/PhenoCompose boundary.
They are not additional cloud providers and do not change BytePort's scope.

| Substrate | Component | Contract |
| --- | --- | --- |
| Podman | NanoVMS | detect engine/capabilities and own container lifecycle |
| Podman | PhenoCompose | render the deterministic Podman composition plan |
| Podman | BytePort | never persist Podman state or credentials |
| Podman | Evidence | version, probe, image digest, lifecycle receipt |
| Apple Containers | NanoVMS | provide adapter and health/failure contract |
| Apple Containers | PhenoCompose | render deterministic dependencies |
| Apple Containers | BytePort | never own Apple state or credentials |
| Apple Containers | Evidence | extension version, host, digest, health |
| WSL Containers | NanoVMS | provide host adapter, isolation, and lifecycle |
| WSL Containers | PhenoCompose | render the WSL execution-context plan |
| WSL Containers | BytePort | never own distro/container state or credentials |
| WSL Containers | Evidence | version, distro, digest, lifecycle receipt |

The substrate rows describe an adapter contract, not a claim that every adapter
is already production-ready. A new implementation must provide the evidence
fields above before it is enabled by default. BytePort may carry an opaque
execution request or receipt reference, but BytePort does not own substrate
state.

The direct Apple Containers and WSLc lifecycle gate is normative; see
[ADR 0011][direct-runtime-gate].

## Evidence record

Every tool-sphere change or operational run should leave a small, linkable
record containing:

```yaml
tool: substrate | sharecli | phenodag
version: <commit-or-release>
operation: <discover | route | plan | render | apply | run | status>
owner_component: byteport | phenocompose | nanovms | phenotype-infra
source: <repository path or URL>
input_digest: <manifest/graph/request digest, if applicable>
receipt: <owner receipt or CI/runbook link>
verified_utc: <YYYY-MM-DDThh:mm:ssZ>
```

`TBD` is acceptable for a not-yet-captured evidence field only when the
operator also links a follow-up issue. Do not store credentials, provider
state snapshots, or mutable runtime records in this governance document.

## Boundary review

Review this map whenever a tool adds a provider adapter, a new renderer, or a
new lifecycle operation. The review must identify the owning API and add a
receipt/evidence field before the feature is enabled in production workflows.

[direct-runtime-gate]: ../adr/0011-direct-runtime-lifecycle-gate.md
