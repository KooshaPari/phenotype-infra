# Tool-sphere ownership

This document records the boundary for repositories that coordinate work
across the foundation. These tools may discover resources, build plans, or
invoke an owning API; they must not become a second cloud-state or runtime
control plane.

## Ownership matrix

| Tool sphere | Owns | Must not own | Handoff to | Evidence required |
|-------------|------|--------------|------------|-------------------|
| `substrate` | host and local substrate discovery, capability facts, and transport adapters | provider credentials, cloud resource state, composition semantics, or runtime lifecycle state | BytePort for cloud/IaC operations; NanoVMS for execution | capability snapshot, adapter version, probe timestamp, and source host |
| `sharecli` | operator-facing cross-repository commands, authenticated request routing, and receipt formatting | provider CRUD, deployment truth, secret values, or renderer-specific policy | the owning API (BytePort, PhenoCompose, or NanoVMS) | command, target API, request/receipt identifier, exit status, and UTC timestamp |
| `phenodag` | dependency graphs, task ordering, retries, and orchestration receipts | cloud state, credentials, image/build semantics, or process/VM lifecycle | BytePort for apply/status; PhenoCompose for render; NanoVMS for run/status | graph digest, node outcomes, retry history, and linked owner receipt |

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

| Substrate | NanoVMS responsibility | PhenoCompose responsibility | BytePort boundary | Required evidence |
|-----------|------------------------|-----------------------------|-------------------|-------------------|
| Podman | detect the host engine, select capabilities, and own container lifecycle | render the unified composition into a Podman-compatible plan | must not persist Podman containers, images, or runtime state | Podman version, host capability probe, image digest, lifecycle receipt |
| Apple Containers extension | provide the engine adapter and health/failure contract when available | render the composition target and preserve deterministic dependencies | must not become an Apple Containers state or credential owner | extension version, host/architecture, image digest, health receipt |
| First-party WSL containers extension | provide the WSL host adapter, isolation tier, and lifecycle semantics | render the composition target for the WSL execution context | must not persist WSL distro/container state or own host credentials | WSL/extension version, distro identity, image digest, lifecycle receipt |

The substrate rows describe an adapter contract, not a claim that every adapter
is already production-ready. A new implementation must provide the evidence
fields above before it is enabled by default. BytePort may carry an opaque
execution request or receipt reference, but BytePort does not own substrate
state.

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
