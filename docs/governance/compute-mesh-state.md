# Compute Mesh State

Live status of the Phenotype compute mesh. The OCI section below is auto-managed
by `oci-post-acquire` on lottery success. Other providers are hand-edited.

This file is an inventory and evidence index, not a provider control plane. The
provider repositories and their deployment state remain the source of truth;
entries here must point operators back to that source before an apply or a
recovery action. BytePort owns AWS-like provisioning, PhenoCompose owns
composition, and NanoVMS owns execution. This document only records the
cross-repository view.

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
|----------|--------|-------|--------|----------------|----------|-------|
| Hetzner CAX11 | ✅ | TBD | TBD | TBD | TBD | primary control plane |
| Fly.io | ✅ | TBD | TBD | TBD | TBD | edge workers |
| Cloudflare Workers | ✅ | TBD | TBD | TBD | TBD | router |
| Vercel | ✅ | TBD | TBD | TBD | TBD | UI hosting |
| Supabase | ✅ | TBD | TBD | TBD | TBD | managed PG |
| OCI Always-Free | ⏳ | TBD | TBD | TBD | TBD | pending lottery acquisition |

<!-- The `oci-post-acquire` orchestrator will append/replace an
"## OCI Status: ✅ ACQUIRED" block below this line on success. Do not
hand-edit between the AUTO-INSERTED markers; the next run will overwrite. -->
