# Absorption index

Canonical index of archived repos absorbed or retired into the Phenotype platform layer. Each entry links to a disposition README under `docs/absorbed-from/<repo>/`.

## Wave G19 (2026-06-18)

Registry wave G19 stub absorption — owner repo: **phenotype-infra**. Registry ledger: `phenotype-registry` wave15 execution doc (`docs/operations/wave15-execution-2026-06-17.md`).

| Repo | Disposition | Absorption README | Notes |
|------|-------------|-------------------|-------|
| [phenotype-hub](https://github.com/KooshaPari/phenotype-hub) | ABSORB | [docs/absorbed-from/phenotype-hub/README.md](absorbed-from/phenotype-hub/README.md) | Governance scaffold docs; canonical owner is this repo |
| [vibeproxy-monitoring-unified](https://github.com/KooshaPari/vibeproxy-monitoring-unified) | RETIRE | [docs/absorbed-from/vibeproxy-monitoring-unified/README.md](absorbed-from/vibeproxy-monitoring-unified/README.md) | Empty stub; pointer to [cliproxyapi-plusplus VIBEPROXY_ABSORPTION](https://github.com/KooshaPari/cliproxyapi-plusplus/blob/main/docs/VIBEPROXY_ABSORPTION.md) |

## Adding entries

When a future wave absorbs an archived repo into `phenotype-infra`:

1. Add `docs/absorbed-from/<repo>/README.md` with source URL, absorption date, disposition, and canonical mapping.
2. Append a row to the appropriate wave section in this file.
