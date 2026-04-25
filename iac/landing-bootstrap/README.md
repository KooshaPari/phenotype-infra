# phenotype-landing-bootstrap

Tier 2 org-pages bootstrap automation. Given a slug + repo, scaffolds a
`<slug>.kooshapari.com` landing page end-to-end:

1. CF DNS CNAME via API
2. Astro template scaffold (or 301 stub if `.dev` exists)
3. GitHub repo create + push
4. Vercel link + deploy + custom domain attach

## Usage

```bash
cargo run -p phenotype-landing-bootstrap -- \
  --slug thegent \
  --repo KooshaPari/thegent

# Or with explicit metadata:
cargo run -p phenotype-landing-bootstrap -- \
  --slug hwledger \
  --repo KooshaPari/hwLedger \
  --title hwLedger \
  --tagline "LLM capacity planner, fleet ledger, and desktop inference runtime"
```

## Env

- `CF_API_TOKEN` (or `--cf-token-file ~/.cloudflare-token`)
- `CF_ZONE_ID` (default: kooshapari.com zone)
- `GITHUB_TOKEN` (via `gh auth token`)

## Idempotency

Each step is a no-op if already complete. Re-running re-deploys to Vercel.

## Coexistence rule

If the source repo's `homepageUrl` contains `.dev`, the tool skips the full
landing scaffold and instead emits a redirect-only `vercel.json` that 301s
`<slug>.kooshapari.com/*` → `<canonical-.dev>/*`. Per
[org-pages-default-pattern.md](../../../docs/governance/org-pages-default-pattern.md).
