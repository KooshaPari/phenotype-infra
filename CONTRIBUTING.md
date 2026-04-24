# Contributing to phenotype-infra

Standardized Phenotype enterprise contribution guidelines for infrastructure
changes.

## Development Workflow

- **ADR before IaC.** Any new node, provider, or topology change requires an
  accepted ADR in `docs/adr/` before the `iac/` scaffold lands.
- **Runbook before node.** Every node in the mesh has a matching runbook in
  `docs/runbooks/`.
- **Plan-only from agents.** `terraform apply` and `ansible-playbook` execution
  are human-only. Agents may `fmt`, `validate`, `plan`, `--check`, and open PRs.
- Run `vale .` and `actionlint .github/workflows/` locally before pushing.
- Document operator-facing changes in `CHANGELOG.md`.

## Commit Style

Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`).

## See also

- `AGENTS.md`, `CLAUDE.md` — agent operational rules
- `SETUP_STEPS.md` — bootstrap procedure
