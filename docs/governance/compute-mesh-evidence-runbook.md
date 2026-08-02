# Compute Mesh Evidence Index and Runbook

**Status:** template (no live deployment is asserted)

This runbook defines the evidence record needed before a compute-mesh row can
move from `⏳` to `✅`. It is an operator-facing index for the cross-repository
view; it does not apply infrastructure, render compositions, or manage runtime
state. Keep credentials, provider handles, and mutable state out of this file.

## Ownership boundaries

The owning repository remains authoritative after each handoff:

- **BytePort** owns provider-neutral desired state, provider adapters, IaC
  plans, and apply authorization. It does not own composition semantics or
  local runtime lifecycle. Its receipt is an immutable plan/status record with
  the provider scope.
- **PhenoCompose** owns composition manifests and deterministic target renders.
  It does not own cloud provider state or runtime health. Its receipt carries
  the manifest digest, target, and render result.
- **NanoVMS** owns runtime selection, capability probes, lifecycle, health, and
  failures. It does not own provider desired state or composition policy. Its
  receipt carries the probe/lifecycle result, backend, host, and instance ID.
- **phenotype-infra** owns the cross-repository inventory, policy, and
  runbooks. It does not own any of the state stores above. Its receipt links the
  evidence record and UTC verification time.

Podman, Apple Containers, and the first-party WSL Containers extension are
runtime targets behind the PhenoCompose/NanoVMS boundary. They are not extra
cloud-provider owners. `substrate`, `sharecli`, and `phenodag` may discover,
route, or order work, but they must hand state back to the owning API.

## Evidence index

Create one row per pilot or reconciliation attempt. Leave unknown fields as
`TBD` and keep the row `⏳`; do not infer health from a successful plan or a
stale receipt. The record templates below carry the source URL and digest.

| Component | Operation | Ref | Receipt | Verified | Result |
| --------- | --------- | --- | ------- | -------- | ------ |
| BytePort | plan/status | TBD | TBD | TBD | ⏳ |
| PhenoCompose | validate/render | TBD | TBD | TBD | ⏳ |
| NanoVMS | probe/run/status | TBD | TBD | TBD | ⏳ |
| phenotype-infra | inventory/reconcile | TBD | TBD | TBD | ⏳ |

Canonical sources: [BytePort][byteport], [PhenoCompose][phenocompose], and
[NanoVMS][nanovms].

[byteport]: https://github.com/KooshaPari/BytePort
[phenocompose]: https://github.com/KooshaPari/PhenoCompose
[nanovms]: https://github.com/KooshaPari/nanovms

## Record templates

Copy the relevant block into the operator's evidence record or issue. A
complete record uses immutable commits and a link to terminal/CI output; it
never includes a secret value.

### BytePort: desired state and provider plan

```yaml
component: byteport
repository: https://github.com/KooshaPari/BytePort
commit: TBD
operation: plan | status | reconcile
provider: TBD
resource_scope: TBD
desired_state_digest: TBD
plan_or_status_receipt: TBD
verified_utc: TBD
result: TBD
```

The plan is read-only until an owner-authorized apply is recorded. A provider
plan alone does not prove that a node or service is reachable.

### PhenoCompose: composition and target render

```yaml
component: phenocompose
repository: https://github.com/KooshaPari/PhenoCompose
commit: TBD
operation: validate | render
manifest: TBD
target: podman | apple-containers | wsl-containers
manifest_digest: TBD
render_digest: TBD
render_receipt: TBD
verified_utc: TBD
result: TBD
```

The render is an input to NanoVMS; it does not transfer cloud or runtime
ownership to PhenoCompose.

### NanoVMS: capability and lifecycle

```yaml
component: nanovms
repository: https://github.com/KooshaPari/nanovms
commit: TBD
operation: capability-probe | run | health | stop | rollback
backend: podman | apple-containers | wsl-containers | TBD
host_or_distro: TBD
image_digest: TBD
instance_id: TBD
probe_or_lifecycle_receipt: TBD
verified_utc: TBD
result: TBD
```

`ready` requires both an executable capability probe and a representative
health request. An unavailable or executable-only probe remains `⏳`.

## Reconciliation sequence

1. Pin the exact commit/ref for BytePort, PhenoCompose, and NanoVMS. Record
   dirty-state findings before running any command.
2. Ask BytePort for a read-only provider plan/status and record its desired
   state digest and receipt. Do not apply from this runbook.
3. Validate and render the immutable composition with PhenoCompose. Record the
   manifest and render digests.
4. Run the NanoVMS capability probe for the selected backend. Record the host,
   backend version, probe result, and timeout/error if unavailable.
5. If the probe is `ready`, run one representative lifecycle: create, health,
   restart, and stop. Capture instance identity and receipts for every step.
6. Reconcile the three digests and receipts. Update
   [compute-mesh-state.md](compute-mesh-state.md) only with the UTC time and
   links to the evidence; never copy mutable provider/runtime state into it.
7. If any step is missing, times out, or fails, leave the status at `⏳` (or
   `❌` for a directly observed failure), file the follow-up, and preserve the
   exact error for the next attempt.

## Promotion gate

Use `✅` only when all of the following are true:

- BytePort has an immutable, owner-authorized plan/status receipt.
- PhenoCompose has a reproducible render whose digest is recorded.
- NanoVMS has a `ready` capability result and representative lifecycle health.
- The provider, host/backend, and target scope are explicit.
- Every record has `Owner`, `Source`, `Verified`, and `Evidence` values.
- Rollback/stop evidence is captured for the same instance or deployment.

Until every condition is linked, the inventory is intentionally non-green.
