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

