# Compute Mesh Specification

Full topology of the 7-node Phenotype compute mesh. Authoritative source for node inventory, MagicDNS names, port assignments, ACLs, and data flows.

## Node inventory

| # | Node | Provider | Shape | Role | Public IP? | MagicDNS |
|---|------|----------|-------|------|------------|----------|
| 1 | oci-primary | Oracle Cloud | Ampere A1 Flex (2 OCPU / 12 GiB) | Forgejo + Vaultwarden + Woodpecker server | No (CF Tunnel only) | `oci-primary` |
| 2 | oci-secondary | Oracle Cloud | Ampere A1 Flex (2 OCPU / 12 GiB) | Woodpecker agent, Prometheus, backup | No | `oci-secondary` |
| 3 | gcp-e2 | GCP | e2-micro (2 vCPU / 1 GiB) | Tertiary runner, uptime sentinel | No | `gcp-e2` |
| 4 | aws-lambda | AWS | Lambda + API Gateway | GitHub webhook → Forgejo mirror fanout | Yes (API GW) | n/a (stateless) |
| 5 | cf-edge | Cloudflare | Workers + Tunnel + R2 | Public edge, routing, (Phase 2) registry mirror | Yes (cf.phenotype.io) | n/a |
| 6 | home-mac | Home | Apple Silicon, 64 GiB | Heavy runner (cargo release, VRAM) | No | `home-mac` |
| 7 | hetzner-burst | Hetzner (Phase 2) | CAX11 Arm | Paid spillover for heavy queues | No | `hetzner-burst` |

## Tailscale tailnet

- **Tailnet:** `<OWNER_GMAIL>.ts.net` (default MagicDNS suffix)
- **Tags:** `tag:backbone` (OCI), `tag:runner` (all CI), `tag:home` (home-mac), `tag:burst` (hetzner)
- **Auth keys:** ephemeral, 1-hour TTL, auto-expiring, provisioned via Terraform (see `iac/terraform/tailscale.tf`)

## Network segmentation

```
                     ┌─────────────────────┐
                     │  Cloudflare Edge    │ (cf-edge)
                     │  tunnel + workers   │
                     └──────────┬──────────┘
                                │  HTTPS (CF Tunnel)
                     ┌──────────┴──────────┐
                     │  oci-primary        │ ← Forgejo + Vaultwarden + Woodpecker server
                     └─┬─────────┬─────────┘
             Tailscale │         │ Tailscale
          ┌────────────┘         └─────────────┐
          │                                    │
  ┌───────┴────────┐                ┌─────────┴────────┐
  │ oci-secondary  │                │ gcp-e2           │
  │ (CI agent)     │                │ (CI agent + mon) │
  └────────────────┘                └──────────────────┘
          │ Tailscale                          │
  ┌───────┴────────┐                ┌─────────┴──────────┐
  │ home-mac       │                │ hetzner-burst      │
  │ (heavy runner) │                │ (Phase 2 spill)    │
  └────────────────┘                └────────────────────┘

  aws-lambda (webhook fanout) ← public API GW → GitHub webhooks
                             → Forgejo push mirror
```

## Port assignments

| Service | Port | Exposure |
|---------|------|----------|
| Forgejo HTTP | 3000 | Tailscale only; CF Tunnel on 443 externally |
| Forgejo SSH | 22022 | Tailscale only |
| Vaultwarden | 8080 | Tailscale only; CF Tunnel on 443 (subdomain) |
| Woodpecker server gRPC | 9000 | Tailscale only |
| Woodpecker server HTTP | 8000 | Tailscale only; CF Tunnel on 443 (subdomain) |
| Prometheus | 9090 | Tailscale only |
| Grafana | 3001 | Tailscale only; CF Tunnel (admin-only path) |
| hw-mesh-agent-bus (Phase 2) | 50051 | Tailscale only |

## ACLs (Tailscale)

Rough JSON sketch (finalize in `iac/terraform/tailscale.tf`):

- `tag:runner` → `tag:backbone:9000` (Woodpecker gRPC only)
- `tag:backbone` ↔ `tag:backbone` (full)
- admin user → `*:22` (SSH anywhere, with audit)
- `tag:home` → `tag:backbone:9000` only (no inbound)

## Data flows

1. **Push:** dev pushes to GitHub (public) → GitHub webhook → aws-lambda → Forgejo mirror API → Woodpecker pipeline trigger.
2. **CI:** Woodpecker server on oci-primary dispatches to label-matched agent (oci-secondary / gcp-e2 / home-mac).
3. **Build output:** agent uploads artifacts to oci-secondary filesystem (Phase 1); (Phase 2) to Cloudflare R2 mirror.
4. **Secrets:** agent fetches from Vaultwarden via `bw` CLI using scoped service account.

## Rollback reference

See `docs/specs/rollback-kill-switch-spec.md` for per-node disable procedures. Global kill-switch: revert all repos' `.woodpecker.yml` → `.github/workflows/` (GitHub Actions) via Git revert of the migration commit.

## Provider-neutral reconciliation contract

BytePort, PhenoCompose, and NanoVMS exchange a desired-state intent and
evidence-bearing receipts. Provider adapters may change; the intent and receipt
shape must not.

### Intent invariants

Every apply or reconcile request MUST include:

| Field | Contract |
|---|---|
| `intent_id` | Stable caller-generated identifier for the requested operation. |
| `desired_state_digest` | SHA-256 of the canonical provider-neutral desired state. |
| `composition_digest` | SHA-256 of the immutable PhenoCompose render, when composition is involved. |
| `correlation_id` | Trace identifier propagated unchanged across all three hops. |
| `owner` | Owning component or tenant identity; never inferred from provider credentials. |
| `provider` | Adapter name only; credentials and mutable provider handles are excluded. |
| `requested_at` | UTC timestamp recorded by the initiating component. |

The same `intent_id` with the same `desired_state_digest` is idempotent and
MUST return the original outcome. Reusing an `intent_id` with a different
digest MUST fail closed as an intent conflict. A stale observed generation MUST
not overwrite a newer desired state.

### Reconciliation phases and receipt

Adapters MUST expose the phases `plan`, `apply`, `observe`, and `rollback`.
`plan` is non-mutating. `apply` may mutate only resources owned by the adapter.
`observe` records provider status without changing desired state. `rollback`
is compensating action for resources created by the same intent and MUST retain
the original receipt.

Every terminal response MUST include:

```yaml
intent_id: <stable id>
correlation_id: <trace id>
desired_state_digest: sha256:<digest>
composition_digest: sha256:<digest-or-null>
provider: <adapter id>
backend: <execution backend-or-null>
phase: plan | apply | observe | rollback
status: planned | applied | observed | rolled_back | failed | conflict
started_at: <UTC>
finished_at: <UTC>
resource_refs: [<opaque provider references>]
rollback_ref: <receipt id-or-null>
error_code: <machine code-or-null>
```

Receipts are evidence, not a second state store. Provider credentials,
secrets, and mutable resource snapshots MUST NOT appear in intents,
composition manifests, runtime metadata, or this specification.
