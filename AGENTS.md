# AGENTS.md — Agent Operating Rules for phenotype-infra

Companion to `CLAUDE.md`. Applies to every automated agent (Claude Code, Forge, Codex, Gemini, etc.).

## Scope of autonomy

| Directory | Read | Edit autonomously | Apply/execute |
|-----------|------|-------------------|---------------|
| `docs/adr/` | yes | yes (new ADRs, stubs) | n/a |
| `docs/specs/` | yes | yes | n/a |
| `docs/runbooks/` | yes | yes | n/a |
| `docs/governance/` | yes | yes | n/a |
| `configs/*.example` | yes | yes | n/a |
| `configs/*` (non-example) | yes | no | n/a |
| `iac/terraform/` | yes | yes (plan-only) | **no — human only** |
| `iac/ansible/` | yes | yes (dry-run only) | **no — human only** |
| `iac/scripts/` | yes | yes | yes (read-only scripts like `health-check.sh`) |
| `.github/workflows/` | yes | yes | n/a (CI runs them) |

## Workflows that touch production

The following workflows, if triggered, will touch live infrastructure. Agents MUST NOT dispatch these without explicit user approval:

- `iac/scripts/bootstrap-oci.sh` — runs `terraform apply` on OCI
- `iac/scripts/register-home-runner.sh` — installs a launchd plist on the user's Mac
- Any `ansible-playbook` command targeting a real inventory
- Any `terraform apply` across all providers

Safe for agent execution:

- `iac/scripts/health-check.sh` — read-only Tailscale pings
- `terraform fmt`, `terraform validate`, `terraform plan`
- `ansible-lint`, `ansible-playbook --check --diff` (dry-run)

## Credential handling

- **Never** commit secrets. Use `<PLACEHOLDER>` tokens in all files.
- **Never** echo a credential into logs or chat output.
- Vaultwarden is the canonical store; agents read, humans write.

## Branch/PR discipline

- Feature work in `feat/<short-slug>` branches.
- Runbook additions in `docs/<slug>` branches.
- ADR additions in `adr/<number>-<slug>` branches.
- All changes go through PR; no direct pushes to `main`.

## Scripting policy enforcement

Every new script file must:

1. Be Rust unless justified in a top-of-file comment.
2. If bash/sh, be ≤5 lines and carry an inline justification comment.
3. If Python/TypeScript, only exist as part of a pre-existing Python/TS runtime (none currently in this repo).

Terraform and Ansible are exempt — they are domain-specific config languages.

## Kill-switch awareness

If a node or provider is failing, agents should:

1. Check `docs/specs/rollback-kill-switch-spec.md` for the documented revert path.
2. Propose a PR that disables the affected runner label or reverts to GitHub Actions.
3. Never attempt provider API destruction calls.
