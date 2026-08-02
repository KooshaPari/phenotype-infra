# phenotype cloudflared tunnel — config source-of-truth

**Status:** MVP-complete. Tunnel `phenotype` (UUID `6daf5b3d-127e-43a7-8af1-67559de3aa60`)
runs on the user's box with `cloudflared tunnel run --token <jwt>`, which means
`config_src: cloudflare` and the **local `~/.cloudflared/config.yml` is dead**.

This directory is the **pinned declarative source-of-truth** for the tunnel
ingress rules. The workflow is:

```
ingress.json (committed)  -->  ./apply.sh  -->  PUT to CF API  -->  daemon auto-reload ~15s
```

## Why this exists

The Cloudflare daemon uses a `--token` JWT and does NOT consult
`~/.cloudflared/config.yml` for ingress. The only way to change the routing
table is to PUT a new config to the CF API. Without a checked-in source-of-truth,
every change is a one-off curl — and divergence between the live config and
local expectations is a footgun (we already hit this: local `config.yml` was
3 hostnames, live is 4 — agileplus.pheno.studio:8888 was added via API and
the local file drifted).

## Layout

| File | Purpose |
| --- | --- |
| `ingress.json` | Pinned ingress rules (JSON). Committed. |
| `apply.sh` | Idempotent PUT from `ingress.json` to CF API. |
| `README.md` | This file. |

## Usage

```bash
# From a secure box with CF_API_TOKEN in env (load ~/.env first):
./apply.sh

# After editing ingress.json:
./apply.sh                       # push
git diff ingress.json            # review
git commit -m "tunnel: add new-host.pheno.studio -> localhost:9000"
```

The daemon auto-reloads within ~15 seconds of a successful PUT. **Do not
kill the daemon** (shared with other chats per memory).

## Current ingress (v3, as of 2026-07-05)

| Hostname | Origin | App |
| --- | --- | --- |
| `api.pheno.studio` | `http://localhost:3000` | phenotype API gateway |
| `trace.pheno.studio` | `http://localhost:8080` | Tracera (down — Dev-7et) |
| `agileplus.pheno.studio` | `http://localhost:8888` | AgilePlus dashboard |
| `agileplus-classic.pheno.studio` | `http://localhost:3002` | AgilePlus classic (legacy) |
| (fallback) | `http_status:404` | catch-all |

## Daemon management

The cloudflared process is **currently running but not daemonized** — no
Windows Service, no scheduled task. It will not survive a reboot.
Daemonization is the scope of `Dev-9c3` (proper service install, no more
nohup). Until that lands, do not reboot this box.

## Verifying

```bash
# Live state
curl -sS -H "Authorization: Bearer $CF_API_TOKEN" \
  "https://api.cloudflare.com/client/v4/accounts/49dce512822987e0522f0faeffbcc0c8/cfd_tunnel/6daf5b3d-127e-43a7-8af1-67559de3aa60/configurations"

# End-to-end routing
for h in api.pheno.studio trace.pheno.studio agileplus.pheno.studio agileplus-classic.pheno.studio; do
  echo "$h -> $(curl -sS -o /dev/null -w '%{http_code}' "https://$h/")"
done
```

## Why not Terraform?

`phenotype-infra/iac/terraform/cloudflare/tunnel.tf` exists as a stub but
the active `cloudflare_tunnel` resource is commented out. Terraform would
require importing the existing tunnel (destroyed-and-recreated semantics
on `cloudflare_tunnel` resource are dangerous — would invalidate the JWT
and orphan the daemon). A pinned JSON + `apply.sh` is the lowest-risk
declarative pattern that matches the user's "wrap>handroll, xDD-first"
doctrine without taking the production tunnel down for an import.

If we ever want IaC, the upgrade path is: (1) extract rules to terraform
variables, (2) use `cloudflare_tunnel_config` resource (separate from
`cloudflare_tunnel` — non-destructive), (3) run `terraform apply` as a
sync step. This is a `BLOCK A horizon DAG` follow-up, not a blocker.
