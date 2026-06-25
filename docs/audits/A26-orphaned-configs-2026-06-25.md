# A26 — Orphaned Configs Sweep (`configs/`)

**Date:** 2026-06-25
**Branch:** `sweep/A26-orphaned-configs`
**Commit:** `8a65199`
**Epic:** `epic_A` — Hygiene garden & branch slim
**Type:** deadcode

---

## Scope

All files under `configs/` — per-service `.example` configuration files.

## Methodology

1. Enumerate every file under `configs/`.
2. For each config file, search the entire repository for:
   - Direct path references (`configs/<service>/<file>`)
   - Ansible playbook source references (`install-<service>.yml`)
   - Jinja2 template usage (`<service>.j2`)
   - CI pipeline references (`.github/workflows/`, `.woodpecker/`)
   - Runbook citations (`docs/runbooks/`)
   - Spec/governance document references
3. If zero cross-references exist, classify as **orphaned** and move to `archive/configs/`.

## Inventory

| # | Config File | Service | References | Disposition |
|---|-------------|---------|------------|-------------|
| 1 | `configs/cloudflared/config.yml.example` | Cloudflare Tunnel | `docs/runbooks/day2-cloudflare-tunnel.md:31` (`sudo cp configs/cloudflared/config.yml.example ...`) | **KEPT** |
| 2 | ~~`configs/forgejo/app.ini.example`~~ | Forgejo | **None.** Ansible uses `forgejo-app.ini.j2` template; no doc references this path. | **→ `archive/configs/forgejo/`** |
| 3 | ~~`configs/vaultwarden/config.env.example`~~ | Vaultwarden | **None.** Ansible writes `.env` inline via `copy` module; no doc references this path. | **→ `archive/configs/vaultwarden/`** |
| 4 | ~~`configs/woodpecker/server.env.example`~~ | Woodpecker | **None.** Ansible writes `server.env` inline via `copy` module; no doc references this path. | **→ `archive/configs/woodpecker/`** |

## Detailed Findings

### `configs/cloudflared/config.yml.example` — KEPT

This file is explicitly referenced in the Day 2 runbook:

```
docs/runbooks/day2-cloudflare-tunnel.md:31:
  sudo cp configs/cloudflared/config.yml.example /etc/cloudflared/config.yml
```

It documents the expected structure for manual tunnel setup outside the
Ansible-driven path (which uses `cloudflared-config.yml.j2`).

### `configs/forgejo/app.ini.example` — MOVED TO ARCHIVE

- Ansible `install-forgejo.yml` uses `forgejo-app.ini.j2` to template
  `/etc/forgejo/app.ini` at deployment time.
- No document references `configs/forgejo/` by path.
- The `.example` was a human-readable documentation artifact duplicated by
  the Jinja2 template. Operators reference the playbook directly.
- Moved to `archive/configs/forgejo/app.ini.example`.

### `configs/vaultwarden/config.env.example` — MOVED TO ARCHIVE

- Ansible `install-vaultwarden.yml` writes `/etc/vaultwarden/.env` inline
  via `ansible.builtin.copy` with the same environment variables.
- No document references `configs/vaultwarden/` by path.
- The `.example` was a human-readable reference superseded by the Ansible
  playbook's inline configuration.
- Moved to `archive/configs/vaultwarden/config.env.example`.

### `configs/woodpecker/server.env.example` — MOVED TO ARCHIVE

- Ansible `install-woodpecker.yml` writes `/etc/woodpecker/server.env` inline
  via `ansible.builtin.copy` with the same structure.
- No document references `configs/woodpecker/` by path.
- The `.example` was a human-readable reference superseded by the Ansible
  playbook's inline configuration.
- Moved to `archive/configs/woodpecker/server.env.example`.

## Summary

| Metric | Count |
|--------|-------|
| Total config files inventoried | 4 |
| Referenced (kept in place) | 1 |
| Orphaned (moved to archive) | 3 |
| Archive location | `archive/configs/<service>/` |

## Verification

- `configs/` now contains only `cloudflared/config.yml.example`.
- `archive/configs/` contains `forgejo/app.ini.example`,
  `vaultwarden/config.env.example`, `woodpecker/server.env.example`.
- `git diff --cached --name-status` shows 3 renames from `configs/` to `archive/configs/`.

## References

- Previous audit: `iac/.grade-reports/A26-orphaned-configs-audit.md` (2026-06-24)
   — initial pass that classified all 4 as referenced. Second pass found 3 orphaned.
- Epic A: Hygiene garden & branch slim — DAG unit A26.
