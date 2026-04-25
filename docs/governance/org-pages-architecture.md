# Org Pages Architecture — `*.kooshapari.com`

**Status:** Proposal (2026-04-25)
**Owner:** Koosha (personal portfolio surface)
**Related:**
- `repos/CONSOLIDATED_DOMAIN_MAP.md` (Phenotype-org product domains: `phenotype.dev`, `agileplus.dev`, ...)
- `iac/ansible/playbooks/install-cloudflared.yml` (current tunnel-based DNS)
- `repos/PhenoProc/python/pheno_kits/infra/core/tunnels.py` (tunnel-domain logic)

---

## Problem

`kooshapari.com` is the personal/identity domain. Current state:

- `*.kooshapari.com` wildcard CNAME → dead Cloudflare tunnel `6ae140ae-...cfargotunnel.com` → returns **HTTP 530** for every subdomain.
- Only working subdomain: `preview.kooshapari.com` (Vercel A record `76.76.21.21`).
- Configured-but-broken: `docs`, `byte`, `trace`, `zen`, `ci`, `forgejo`, `tm`, `ttt`, `atomcp`, `mcp-sandbox` (most rely on the dead tunnel).
- No org-level portfolio surface exists yet.

The user wants a three-layer surface:

1. **Org portfolio:** `projects.kooshapari.com` — index of all KooshaPari projects (90+ repos).
2. **Per-project landing/docs:** `<project>.kooshapari.com`.
3. **Per-project dev-env microfrontends:** OTEL/QA/observability surfaces per project.

This is **distinct** from the Phenotype-org ecosystem domains (`phenotype.dev`, `agileplus.dev`, etc.) — those are product brands; `kooshapari.com` is the personal portfolio overlay. Many projects will be reachable from both surfaces (e.g., AgilePlus at `agileplus.dev` *and* `agileplus.kooshapari.com`).

---

## Architecture

### Layer 1 — Org portfolio (`projects.kooshapari.com`)

Single static site listing every active KooshaPari repo with filter/search.

- **Repo:** `KooshaPari/projects-landing` (new) — VitePress or Next.js static export.
- **Data source:** `gh repo list KooshaPari --json name,description,url,repositoryTopics,homepageUrl,pushedAt,isArchived` at build time. Cached as `data/repos.json`. Rebuild nightly via GitHub Actions cron (no Actions billing impact — cron only writes to repo, no concurrent runners).
- **Filtering:** by topic (`phenotype-ecosystem`, `agent-framework`, `infrastructure`, `deprecated`, `governance`, `cli`, `sdk`, ...) and by status (active vs archived).
- **Cards:** name, description, topic chips, last-pushed, GitHub link, homepage (if `homepageUrl` set).
- **Hosting:** Vercel project `projects-landing` → custom domain `projects.kooshapari.com`. Free tier sufficient (static, < 5 MB).

### Layer 2 — Per-project landing (`<project>.kooshapari.com`)

Each repo with a docs surface gets a subdomain. Uses VitePress (already shipped via session #248 across ~28 repos).

- **DNS:** `CNAME <project>.kooshapari.com → cname.vercel-dns.com` (managed via `phenotype-infra/iac/ansible/playbooks/`, new `cloudflare-dns.yml` playbook using existing `Zone:DNS:Edit` token).
- **Hosting:** one Vercel project per repo, linked to `main` branch of `KooshaPari/<repo>`.
- **Build:** repo's existing `docs/` VitePress build (output → `docs/.vitepress/dist`).
- **Convention:** repo `name` → subdomain in lowercase (e.g., `AgilePlus` → `agileplus.kooshapari.com`).
- **Coexistence with `phenotype.dev`:** product-branded repos (AgilePlus, Tracera, FocalPoint, ...) get *both* `<product>.kooshapari.com` (personal portfolio link) AND `<product>.dev` (product brand) — both Vercel projects can coexist; pick one as canonical, other as 301 redirect.

### Layer 3 — Per-project dev-env microfrontends

Path-based within each project's Vercel deployment:

```
<project>.kooshapari.com/
├── /                    → landing page (README/overview)
├── /docs                → VitePress docs
├── /otel                → OTEL traces/metrics (PhenoObservability iframe or proxy)
├── /qa                  → test reports, coverage, FR traceability dashboard
└── /preview/<pr-id>     → PR preview deploys (Vercel preview routes)
```

**Why path-based, not subdomain-based:**

- Single TLS cert per project (Vercel auto-provisions).
- Single Vercel project per repo (free tier accommodates).
- Simpler cross-linking (relative URLs).
- Lower DNS-record sprawl (one CNAME per project, not four).

**Microfrontend wiring:**

- `/docs` — VitePress build output mounted at `/docs/`.
- `/otel` — embeds `PhenoObservability` Grafana panels via iframe with project-scoped query params, OR static export of last-N-days trace summaries from PhenoObservability's OTLP store.
- `/qa` — built from `agileplus validate --json` output + coverage from `cargo llvm-cov` / `pytest --cov` + Tracera FR traceability JSON. Static dashboard regenerated per build.
- `/preview/<pr-id>` — Vercel's native PR preview URLs aliased into the project's path namespace (or kept as `<pr>-<project>.vercel.app` and linked from PR comments).

---

## DNS Migration Plan

### Step 1: Remove the dead wildcard

Current: `*.kooshapari.com CNAME 6ae140ae-...cfargotunnel.com` returns 530 for everything not explicitly overridden.

**Action:** Replace with a default 404-or-redirect. Options:

- **A.** Delete the wildcard; rely on per-project explicit CNAMEs (cleaner, but every new project needs DNS work).
- **B.** Point `*` at a static "project not found" page on Vercel that links back to `projects.kooshapari.com` (better UX, recommended).

Recommended: **B**. Add a Vercel project `kooshapari-fallback` with a 404 page; CNAME `*.kooshapari.com → cname.vercel-dns.com`; configure Vercel to attach `*.kooshapari.com` and serve fallback for un-registered subdomains.

### Step 2: Restore `docs.kooshapari.com`

Currently 530. Either remove if unused, or point at `phenodocs` Vercel deploy.

### Step 3: Per-project CNAMEs

Add one CNAME per project that ships a landing page. Driven by `iac/ansible/playbooks/cloudflare-dns.yml` (new) reading from `iac/ansible/group_vars/dns.yml`:

```yaml
# iac/ansible/group_vars/dns.yml
kooshapari_subdomains:
  - { name: projects, target: cname.vercel-dns.com, project: projects-landing }
  - { name: agileplus, target: cname.vercel-dns.com, project: agileplus-landing }
  - { name: tracera, target: cname.vercel-dns.com, project: tracera-landing }
  - { name: hwledger, target: cname.vercel-dns.com, project: hwledger-landing }
  # ...
```

Token scope: `Zone:DNS:Edit` on `kooshapari.com` (already provisioned per `iac/ansible/README.md`).

### Step 4: Salvage tunnel-routed subdomains

`byte`, `trace`, `zen`, `ci`, `forgejo`, `mcp-sandbox`, `atomcp` — these were intended for the OCI tunnel-hosted services (Forgejo, Woodpecker, Vaultwarden, zen-mcp). The OCI Ampere capacity-block (per memory MEMORY.md "Compute mesh state") leaves these orphaned. Treat each as a separate decision:

| Subdomain | Intended target | Action |
|-----------|-----------------|--------|
| `forgejo.kooshapari.com` | OCI Forgejo | Keep tunnel CNAME; document as "pending OCI capacity" |
| `ci.kooshapari.com` | OCI Woodpecker | Same |
| `zen.kooshapari.com` | zen-mcp-server | Migrate to Vercel/Fly.io if MCP server can run there; else pending OCI |
| `byte.kooshapari.com` | BytePort | Likely pending; BytePort marked deprecated in topics |
| `trace.kooshapari.com` | Tracera | Migrate to Vercel; Tracera is `homepageUrl: kooshapari.github.io/trace/` already |
| `mcp-sandbox.kooshapari.com` | PhenoProc MCP test | Pending OCI |
| `atomcp.kooshapari.com` | atoms.tech MCP | Pending OCI |
| `tm.kooshapari.com` | triple-m (Vercel) | Working |
| `ttt.kooshapari.com` | odin-ttt (Vercel) | Working |
| `preview.kooshapari.com` | phenotype-previews-smoketest | Working |
| `docs.kooshapari.com` | phenodocs | Broken; re-attach to Vercel |

---

## Rollout (agent-driven, aggressive)

| Phase | Tasks | Effort |
|-------|-------|--------|
| **P0** (this session) | Architecture doc (this file). Scaffold `KooshaPari/projects-landing` repo. Coverage matrix of current per-project deploys. | 5 min |
| **P1** | Build portfolio site: VitePress + auto-generated cards from `gh repo list`. Push to `projects-landing`. Vercel project link. | 10 min |
| **P2** | DNS: add `projects.kooshapari.com → cname.vercel-dns.com` via CF API (token in `1Password://Phenotype/CF API Token`). Replace wildcard with fallback project. | 5 min |
| **P3** | Codify DNS in Ansible (`cloudflare-dns.yml` playbook + `dns.yml` group_vars). Migrate top-10 projects to per-project Vercel deployments. | 30 min, parallel subagents per project |
| **P4** | Path-based microfrontends per project: `/docs`, `/otel`, `/qa` mounting. Requires PhenoObservability OTLP endpoint stable. | gated on Layer-3 substrate |

Single agent can complete P0–P2 (~20 min). P3–P4 are parallel-subagent fan-outs.

---

## Cross-Project Reuse Opportunities

- **`projects-landing`'s build script** (gh repo list → JSON → cards) is reusable for `phenotype.dev` org hub. Extract to shared package: `packages/repo-portfolio-data` (TypeScript, npm-publishable).
- **DNS Ansible playbook** (`cloudflare-dns.yml`) is reusable for `phenotype.dev`, `agileplus.dev`, etc. Parameterize by zone.
- **`/qa` traceability dashboard component** is the same data the AgilePlus dashboard renders — extract to `@phenotype/design`'s `<TraceabilityMatrix>` component.

---

## Open Decisions (require user input)

1. **Canonical domain conflict:** for product-branded repos (AgilePlus, Tracera, FocalPoint), is canonical `<product>.dev` (Phenotype-org brand) or `<product>.kooshapari.com` (personal portfolio)? Recommend `<product>.dev` canonical, `<product>.kooshapari.com` as 301 redirect — but only after the `.dev` domains are registered (currently all marked "pending" in CONSOLIDATED_DOMAIN_MAP).
2. **Wildcard fallback strategy:** delete (option A) vs Vercel 404 page (option B)?
3. **OCI-pending subdomains:** keep dead tunnel CNAMEs (returns 530) vs delete and re-add when OCI ready?
4. **`projects-landing` framework:** VitePress (consistent with rest of org docs) vs Next.js (richer interactivity for filter/search/cards)?

Recommend: **(1)** `.dev` canonical with `.kooshapari.com` redirect; **(2)** option B (Vercel fallback page); **(3)** delete dead CNAMEs to clear 530 noise; **(4)** Next.js — portfolio benefits from client-side filter/search/sort, and we already deploy Next on Vercel.
