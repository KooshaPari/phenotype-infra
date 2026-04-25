# Org Pages Coverage Matrix

**Generated:** 2026-04-25
**Companion to:** `org-pages-architecture.md`

Coverage of current per-project hosted surfaces across the 93 active KooshaPari repos.

## Summary

| Layer | Count | Notes |
|-------|-------|-------|
| Active repos (non-archived) | 93 | of 156 total |
| With `homepageUrl` set | 26 | 28% coverage |
| With working hosted docs (HTTP 200) | ~12 | many `homepageUrl` values 404 |
| On Vercel under personal account | 5 | `triple-m`→`tm.k.com`, `odin-ttt`→`ttt.k.com`, `phenotype-previews-smoketest`→`preview.k.com`, plus 2 fixit-go variants |
| On `*.kooshapari.com` (working) | 3 | `tm`, `ttt`, `preview` only |
| On `*.kooshapari.com` (broken, HTTP 530) | 8+ | all wildcard-CNAME→dead-tunnel: `docs`, `byte`, `trace`, `zen`, `ci`, `forgejo`, `mcp-sandbox`, `atomcp`, ... |

## Hosted-docs coverage by repo

| Repo | homepageUrl | Status | Target |
|------|-------------|--------|--------|
| AgilePlus | `kooshapari.github.io/AgilePlus/` | 200 | gh-pages |
| thegent | `kooshapari.github.io/thegent/` | 200 | gh-pages |
| hwLedger | `kooshapari.github.io/hwLedger/` | 200 | gh-pages |
| Tracera | `kooshapari.github.io/trace/` | 404 | gh-pages stale (repo renamed Tracera, pages source not updated) |
| Civis | `kooshapari.github.io/civ/` | 404 | gh-pages stale (repo renamed) |
| cliproxyapi-plusplus | `kooshapari.github.io/cliproxyapi-plusplus/` | unknown | gh-pages |
| agentapi-plusplus | `kooshapari.github.io/agentapi-plusplus/` | unknown | gh-pages |
| helios-cli | `kooshapari.github.io/helios-cli/` | unknown | gh-pages |
| helios-router | `kooshapari.github.io/helios-router/` | unknown | gh-pages |
| Parpoura | `kooshapari.github.io/parpour/` | unknown | gh-pages stale (renamed) |
| QuadSGM | `kooshapari.github.io/4sgm/` | unknown | gh-pages stale (renamed) |
| Dino | `kooshapari.github.io/Dino/` | unknown | gh-pages |
| dinoforge-packs | `kooshapari.github.io/Dino` | unknown | shares Dino docs |
| HeliosLab | `blackboard.sh/colab/` | external | not org-controlled |
| portage | `harborframework.com/` | external | renamed brand |
| atoms.tech | `atoms-final.vercel.app` | Vercel | should be `atoms.kooshapari.com` |
| odin-TTT | `odin-ttt.vercel.app` | Vercel | aliased to `ttt.kooshapari.com` |
| triple-m | (no homepageUrl) | Vercel | aliased to `tm.kooshapari.com` |
| PhenoHandbook | `phenotype.dev/handbook` | unregistered | `phenotype.dev` not yet provisioned |
| PhenoSpecs | `phenotype.dev/specs` | unregistered | same |
| phenotype-registry | `phenotype.dev/registry` | unregistered | same |
| phenotype-omlx | `omlx.ai` | external | separate brand |
| phenotype-ops-mcp | `ops.city` | external | nanos/ops third-party |
| Planify | `plane.so` | external | upstream |
| 67 other active repos | (none) | none | no hosted surface |

## Per-subdomain probe (`*.kooshapari.com`)

| Subdomain | DNS resolves | HTTP | Backend |
|-----------|--------------|------|---------|
| `projects` | yes (CF wildcard) | 530 | dead tunnel |
| `docs` | yes (CF wildcard) | 530 | dead tunnel |
| `preview` | yes (Vercel A) | 200 | `phenotype-previews-smoketest` |
| `tm` | yes (Vercel) | 200 | `triple-m` |
| `ttt` | yes (Vercel) | 200 | `odin-ttt` |
| `byte`, `trace`, `zen`, `ci`, `forgejo`, `mcp-sandbox`, `atomcp` | yes (CF wildcard) | 530 | dead tunnel — pending OCI capacity |

## Gap analysis

- **70 repos (75% of active)** have no hosted surface at all.
- **5 repos** with `homepageUrl` point at `phenotype.dev/*` paths that don't yet resolve (PhenoHandbook, PhenoSpecs, phenotype-registry).
- **3 repos** with stale gh-pages URLs (Tracera, Civis, Parpoura, QuadSGM — all renamed without updating Pages source).
- **0 repos** wired into a per-project `<project>.kooshapari.com` Vercel deployment beyond the 2 personal apps (tm, ttt) and 1 smoketest.

## Migration priority (top 10)

Repos with active development + governance topics — first to migrate to per-project Vercel + `<project>.kooshapari.com`:

1. **AgilePlus** — already has 200-status gh-pages docs; quick port.
2. **thegent** — same.
3. **hwLedger** — same.
4. **Tracera** — fix stale Pages, then port.
5. **Civis** — fix stale Pages, then port.
6. **PhenoObservability** — substrate for Layer-3 OTEL panels; needs landing.
7. **phenotype-infra** — should host its own runbooks at `infra.kooshapari.com`.
8. **PhenoSpecs** + **phenotype-registry** + **PhenoHandbook** — together compose `phenotype.dev`; can co-host at `<each>.kooshapari.com` until `.dev` registered.
9. **HexaKit**, **DataKit**, **AuthKit**, **TestingKit**, **ObservabilityKit**, **McpKit**, **PhenoKits** — SDK family; uniform docs treatment.
10. **FocalPoint** — flagship Sidekick collection product.

The remaining ~80 repos can follow a templated rollout (one Ansible playbook run per batch of 10).
