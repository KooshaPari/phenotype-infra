# ADR 0011 — Direct Apple/WSLc lifecycle default gate

- **Status:** Accepted
- **Date:** 2026-08-03
- **Scope:** PhenoCompose, NanoVMS, BytePort, and phenotype-infra

## Context

The tool-sphere ownership contract assigns composition and deterministic target
renders to PhenoCompose, while NanoVMS owns execution-engine selection,
lifecycle, health, and failures. BytePort owns infrastructure plans and apply
authorization; an execution integration may pass an opaque request or receipt
reference, but it must not copy runtime state.

The current PhenoCompose PR 118 is an observed transitional mismatch. Its
description advertises direct create, rollback, status/down, and health-check
lifecycle for Apple Containers and WSLc, and its implementation permits those
providers in `ensure_apply_capabilities`, routes commands through `container`
and `wslc`, and executes direct create/rollback/health paths. See [PR 118][pr]
and the [implementation at commit `d602e793`][implementation]. This is code
scope evidence only; PR 118 also records that no live Apple Containers or WSLc
daemon was available for verification. It is not a health or deployment
receipt.

## Decision

NanoVMS remains the default lifecycle owner. Apple Containers and WSLc direct
paths in PhenoCompose are transitional implementation lanes and are not
default-enabled by the presence of provider code or unit-test coverage.

BytePort remains an opaque request/receipt surface for these lanes. It may
forward a request and retain a receipt reference, but it must not manage Apple
Containers, WSLc, Podman, distro, container, credential, or health state.

## Default-enablement gate

The Apple Containers and WSLc capability rows remain **pending** and their
default state is **disabled** until one of these gates is satisfied:

1. A NanoVMS-owned lifecycle receipt covers the provider and operation being
   enabled; or
2. An approved transitional ADR and receipt explicitly names the temporary
   lifecycle owner, scope, expiry, rollback path, and handoff to NanoVMS.

Either gate must link all of the following without storing credentials or
secret values in this repository:

- provider and host/extension identity;
- implementation commit and adapter version;
- capability probe and representative image digest;
- create, status, health, stop/rollback, and failure receipt as applicable;
- owner, verified UTC timestamp, and rollback/expiry evidence.

Until the receipt exists, direct Apple/WSLc execution may be used only as an
explicit development or verification opt-in. It must not be selected by an
organization default, release preset, mesh inventory row, or BytePort plan,
and no health claim may be inferred from source inspection or a passing test.

## Transitional capability rows

These rows intentionally remain pending. `TBD` is not evidence of health.

| Capability | Status | Default | Lifecycle owner | Evidence |
| --- | --- | --- | --- | --- |
| Podman through NanoVMS | pending | contract only | NanoVMS | TBD |
| Apple Containers direct lane | pending | disabled | NanoVMS (contract) | TBD |
| WSLc direct lane | pending | disabled | NanoVMS (contract) | TBD |
| BytePort | pending | opaque receipt | request / lifecycle | TBD |

## Consequences

- PhenoCompose can continue developing adapters without silently changing the
  foundation's lifecycle authority.
- A release cannot promote Apple Containers or WSLc from implementation to
  default capability without a verifiable receipt or an explicitly approved
  transitional record.
- The compute-mesh inventory remains fail-closed: capability rows stay pending
  until owner, source, verification time, and evidence are linked.

## Related

- [Tool-sphere ownership contract][ownership]
- [Compute mesh state contract][mesh-state]
- [ADR 0001 — Hybrid Compute Mesh](0001-hybrid-compute-mesh.md)

[pr]: https://github.com/KooshaPari/PhenoCompose/pull/118
[implementation]: https://github.com/KooshaPari/PhenoCompose/pull/118/files
[ownership]: ../governance/tool-sphere-ownership.md
[mesh-state]: ../governance/compute-mesh-state.md
