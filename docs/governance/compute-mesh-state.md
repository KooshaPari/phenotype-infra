# Compute Mesh State

This is the cross-repository inventory of the Phenotype compute mesh. It is an
evidence index, not a provider control plane: provider repositories and their
deployment systems remain authoritative. **No row in this snapshot asserts
that a provider is healthy or currently deployed unless the required receipt
is linked.**

The topology and intended node roles are defined by [ADR 0001][adr-0001] and
the [compute-mesh specification][mesh-spec]. Ownership boundaries for
`substrate`, `sharecli`, `phenodag`, BytePort, PhenoCompose, and NanoVMS are
defined in [tool-sphere-ownership.md][ownership].

## Status contract (fail closed)

`✅` means a read-only health or inventory check completed and the row has an
owner, authoritative source, UTC verification time, and linked evidence.
`⏳` means the provider or capability is pending, unverified, or unknown.
`❌` is reserved for a directly observed failure. `TBD` is a field value, not
a healthy status.

A row with `Owner`, `Source`, `Verified`, or `Evidence` set to `TBD` MUST NOT
use `✅`. The specification is an intended-topology source; it is not a live
health receipt. Provider credentials and mutable state never belong in this
file.

## Specified mesh nodes

The rows below intentionally remain pending until each node has a current
read-only receipt. `Source` points to the topology specification only; it does
not prove that the node exists, is reachable, or is configured as described.

| Node | Status | Owner | Source | Verified | Evidence |
| --- | --- | --- | --- | --- | --- |
| `oci-primary` | ⏳ | TBD | [mesh spec][mesh-spec] | TBD | TBD |
| `oci-secondary` | ⏳ | TBD | [mesh spec][mesh-spec] | TBD | TBD |
| `gcp-e2` | ⏳ | TBD | [mesh spec][mesh-spec] | TBD | TBD |
| `aws-lambda` | ⏳ | TBD | [mesh spec][mesh-spec] | TBD | TBD |
| `cf-edge` | ⏳ | TBD | [mesh spec][mesh-spec] | TBD | TBD |
| `home-mac` | ⏳ | TBD | [mesh spec][mesh-spec] | TBD | TBD |
| `hetzner-burst` | ⏳ | TBD | [mesh spec][mesh-spec] | TBD | TBD |

The roles in the specification (Forgejo, runners, edge, webhook fan-out, and
backup) are descriptive only. They are not health or deployment evidence.

## Candidate integrations

These entries preserve the wider provider surface without implying that an
integration is deployed. They remain pending until an owning source and
evidence receipt are recorded.

| Provider | Status | Owner | Source | Verified | Evidence |
| --- | --- | --- | --- | --- | --- |
| Hetzner CAX11 | ⏳ | TBD | TBD | TBD | TBD |
| Fly.io | ⏳ | TBD | TBD | TBD | TBD |
| Cloudflare Workers | ⏳ | TBD | TBD | TBD | TBD |
| Vercel | ⏳ | TBD | TBD | TBD | TBD |
| Supabase | ⏳ | TBD | TBD | TBD | TBD |
| OCI Always-Free | ⏳ | TBD | TBD | TBD | TBD |

These candidates have no recorded owning source or receipt. `cf-edge` is the
specification node for Cloudflare Workers; that mapping does not prove health.

## Execution substrate capability lanes

Podman, Apple Containers, and the first-party WSL Containers extension are
execution targets behind the NanoVMS/PhenoCompose boundary. They are not
additional cloud providers and do not change BytePort's ownership. The adapter
contract and required receipt fields are defined in
[tool-sphere ownership][ownership].

| Capability | Status | Owner | Source | Verified | Evidence |
| --- | --- | --- | --- | --- | --- |
| Podman | ⏳ | contract only | [ownership] | TBD | TBD |
| Apple Containers | ⏳ | contract only | [ownership] | TBD | TBD |
| WSL Containers extension | ⏳ | contract only | [ownership] | TBD | TBD |

The Apple and WSL rows describe first-party capability lanes. Each needs a
host/extension identity, probe, image digest, and lifecycle receipt.

## Release-governance gaps

- `docs/specs/rollback-kill-switch-spec.md` names the Phase-2 implementation
  `iac/scripts/kill-switch.rs`, but that file is absent from this revision.
  Rollback readiness is therefore **not established**. Follow-up issue: `TBD`
  (owner must link a tracked issue before release approval).
- Every pending provider/capability row needs an owner, authoritative source,
  UTC verification time, and receipt. Follow-up issue: `TBD` (owner must link
  a tracked issue before enabling the row or marking it healthy).

[adr-0001]: ../adr/0001-hybrid-compute-mesh.md
[mesh-spec]: ../specs/compute-mesh-spec.md
[ownership]: tool-sphere-ownership.md

<!-- The `oci-post-acquire` orchestrator will append/replace an
"## OCI Status: ✅ ACQUIRED" block below this line on success. Do not
hand-edit between the AUTO-INSERTED markers; the next run will overwrite. -->
