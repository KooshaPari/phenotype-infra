# Compute Mesh State

Live status of the Phenotype compute mesh. The OCI section below is auto-managed
by `oci-post-acquire` on lottery success. Other providers are hand-edited.

This file is an inventory and evidence index, not a provider control plane. The
provider repositories and their deployment state remain the source of truth;
entries here must point operators back to that source before an apply or a
recovery action. BytePort owns AWS-like provisioning, PhenoCompose owns
composition, and NanoVMS owns execution. This document only records the
cross-repository view. The evidence collection template and handoff runbook
are in [compute-mesh-evidence-runbook.md](compute-mesh-evidence-runbook.md).

## Status contract (fail closed)

`✅` means that a read-only health or inventory check completed and the row has
an owner, authoritative source, UTC verification time, and linked evidence.
`⏳` means the provider is pending or its health is unknown. `❌` is reserved
for a directly observed failure. `TBD` is a field value, not a healthy status.

A row with `Owner`, `Source`, `Verified`, or `Evidence` set to `TBD` MUST NOT
use `✅`. Keep it at `⏳` until the evidence record is complete. This prevents
an inventory placeholder from being mistaken for a deployment assertion.

## Entry contract

Every provider row must be kept alongside these four facts (in the `Notes`
column when a dedicated column is not practical):

- **Owner:** the repository or team accountable for the provider integration.
- **Source:** a stable path or URL to the authoritative configuration/state.
- **Verified:** the UTC date of the last read-only health or inventory check.
- **Evidence:** a runbook, CI job, or incident record that supports the status.

If any fact is unknown, use `TBD` and open a follow-up issue; do not infer
health from a stale row. Provider credentials and mutable state never belong in
this file.

| Provider | Status | Owner | Source | Verified (UTC) | Evidence | Notes |
| ---------- | ------ | ----- | ------ | -------------- | -------- | ----- |
| Hetzner CAX11 | ⏳ | TBD | TBD | TBD | TBD | P2 burst; health TBD |
| Fly.io | ⏳ | TBD | TBD | TBD | TBD | not in ADR/spec; scope TBD |
| Cloudflare Workers | ⏳ | TBD | TBD | TBD | TBD | `cf-edge`; health TBD |
| Vercel | ⏳ | TBD | TBD | TBD | TBD | external; ADR/spec scope TBD |
| Supabase | ⏳ | TBD | TBD | TBD | TBD | external; ADR/spec scope TBD |
| OCI Always-Free | ⏳ | TBD | TBD | TBD | TBD | lottery pending |

## ADR/spec alignment

The authoritative seven-node inventory is defined by [ADR 0001][adr-0001] and
[compute-mesh-spec.md][mesh-spec]. The provider summary above is intentionally
broader than that node list and must not be read as proof that an external
integration is deployed.

- `oci-primary` and `oci-secondary` are the OCI backbone.
- `gcp-e2`, `aws-lambda`, `cf-edge`, and `home-mac` are specified mesh nodes.
- `hetzner-burst` is a Phase-2 candidate; it is not an active capacity claim.
- Fly.io, Vercel, and Supabase remain unverified integration candidates until
  an owning source and evidence receipt are recorded.

[adr-0001]: ../adr/0001-hybrid-compute-mesh.md
[mesh-spec]: ../specs/compute-mesh-spec.md

<!-- The `oci-post-acquire` orchestrator will append/replace an
"## OCI Status: ✅ ACQUIRED" block below this line on success. Do not
hand-edit between the AUTO-INSERTED markers; the next run will overwrite. -->
