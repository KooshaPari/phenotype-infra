# Stage-Gates Reusable Contract (v1)

`stage-gates` is promoted as a versioned reusable contract surface for canary-first rollout.

## Canonical Contract

- Contract metadata: `contracts/stage-gates-v1.contract.json`
- Canonical template: `templates/stage-gates/v1/stage-gates.yml`
- Compatibility alias: `templates/stage-gates.yml`

## Strictness Signals

The v1 contract requires strictness wiring via:

- `STAGE_GATES_STRICT` support in the stage-gates workflow template.
- CodeRabbit severity policy in `templates/.coderabbit.yaml`.

## Dependency Fallback Mapping

The canary template-sync lane consumes owner-scoped fallback mappings from:

- `templates/template-sync/dependency-fallbacks.csv`

Columns:

- `owner`
- `dependency_key` (`template_repo` or `stage_gates_repo`)
- `fallback_slug`
- `fallback_workspace_dir`
- `remediation_hint`

This enables deterministic fallback resolution and actionable repo-level remediation hints when dependency checkout fails.

## SemVer Policy

- Major: breaking gate/stage/output contract changes.
- Minor: backward-compatible stage/gate additions.
- Patch: non-breaking fixes and docs-only updates.

## Validation

Run:

```bash
bash scripts/validate_stage_gates_contract.sh
```

This checks contract metadata and required artifacts for the versioned stage-gates surface.
