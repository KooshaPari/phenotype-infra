# phenotype-infra — CLAUDE.md

Defers to `~/.claude/CLAUDE.md` for global rules. Repo-specific overrides below.

## Repo-specific rules

- **Terraform apply is human-only.** Agents may `terraform fmt`, `terraform validate`, `terraform plan`, and open PRs. `terraform apply` requires explicit user confirmation in the terminal session where the user is present.
- **Vaultwarden is read-only from agents.** Agents may fetch credentials for use within a task but must not add, rotate, or delete entries without user confirmation.
- **No `gh repo create` from agents.** Per the sandbox policy / #67 lesson, public-repo creation is denied. The user runs `gh repo create` manually (see `SETUP_STEPS.md`).
- **ADR before IaC.** Any new node, provider, or topology change requires an accepted ADR in `docs/adr/` before the `iac/` scaffold lands.
- **Runbook before node.** Every node in the mesh has a matching runbook in `docs/runbooks/`.
- **Scripting hierarchy** (from `~/.claude/CLAUDE.md`): Rust default; Zig/Mojo/Go with one-line justification; Bash only as ≤5-line glue with justification comment. Terraform and Ansible are exempt as domain-specific tools.

## Safe-to-edit map

| Directory | Autonomous agent? |
|-----------|-------------------|
| `docs/` | Yes (docs, ADR stubs, runbooks) |
| `configs/*.example` | Yes |
| `iac/terraform/` | Plan-only (PR, never apply) |
| `iac/ansible/` | Plan-only (dry-run, never execute) |
| `iac/scripts/` | Yes, with scripting-hierarchy justification |
| `.github/workflows/` | Yes |

## See also

- `AGENTS.md` — agent operational rules
- `docs/governance/security-policy.md` — token + SSH rotation
- `docs/governance/incident-response.md` — outage playbooks
- `docs/governance/journey-traceability-standard.md` — required journey
  evidence pattern for docs that describe real flows
