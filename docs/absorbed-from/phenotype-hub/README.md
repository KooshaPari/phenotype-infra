# phenotype-hub (absorbed)

| Field | Value |
| --- | --- |
| **Source repo** | [phenotype-hub][phenotype-hub-repo] |
| **Absorption date** | 2026-06-18 |
| **Registry wave** | G19 (wave15 stub absorption) |
| **Disposition** | ABSORB |
| **Canonical owner** | **phenotype-infra** (this repo) |

## Summary

`phenotype-hub` was an archived governance scaffold: agent instructions,
functional-requirements tracking, journey traceability, and worklogs. It had no
runtime implementation. Per registry wave G19, its documentation role is
absorbed here. The archived repo remains read-only for history; new edits belong
in `phenotype-infra`.

## Absorbed document index

- `README.md`: hub overview and layout; this file plus
  [`ABSORPTION_INDEX.md`](../../ABSORPTION_INDEX.md).
- `AGENTS.md`: agent governance contract; see
  [governance policies](../../governance/).
- `CLAUDE.md`: Claude/Codex workflow notes; see the repository-root
  `AGENTS.md` and `CLAUDE.md` files when present.
- `CONTRIBUTING.md`: contribution guide; see the root
  [`CONTRIBUTING.md`](../../../CONTRIBUTING.md).
- `FUNCTIONAL_REQUIREMENTS.md`: FR traceability stub; see
  [specifications](../../specs/) and journey manifests.
- `SECURITY.md`: security reporting; see the root
  [`SECURITY.md`](../../../SECURITY.md).
- `docs/worklogs/README.md`: work audit index; see
  [session worklogs](../../sessions/).
- `docs/worklogs/worklog.md`: work audit entries; see
  [session worklogs](../../sessions/).
- `docs/operations/journey-traceability.md`: journey evidence standard; see
  [the canonical document](../../operations/journey-traceability.md).
- `docs/operations/iconography/SPEC.md`: iconography specification; see
  [the canonical document](../../operations/iconography/SPEC.md).
- `docs/journeys/manifests/README.md`: journey manifest index; see
  [the canonical document](../../journeys/manifests/README.md).
- `.github/`: GitHub workflows and templates; see the repository
  [`.github/`](../../../.github/) directory.

## Registry reference

- `phenotype-registry` project stub:
  `projects/phenotype-hub.json` (`disposition: ABSORB`,
  `absorb_target: phenotype-infra`).
- ECOSYSTEM_MAP P8: merge the hub scaffold into infra; the registry keeps only
  a redirect.

## Do not

- Open new feature work against the archived `phenotype-hub` repository.
- Treat `phenotype-hub` as a runtime or routing SSOT. Use the
  `phenotype-infra` ADRs, specs, and runbooks instead.

[phenotype-hub-repo]: https://github.com/KooshaPari/phenotype-hub
