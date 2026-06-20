# phenotype-infra — just command runner
#
# This file mirrors Taskfile.yml (go-task) for teams that prefer `just`.
# Keep both files in sync.

cargo := "cargo"
manifest := "iac/Cargo.toml"

# ── Rust (iac workspace) ──────────────────────────────────────────────────────

# List available recipes
default:
  just --list

# Build all iac crates
build:
  {{cargo}} build --manifest-path {{manifest}} --workspace --all-features

# Run all iac tests
test:
  {{cargo}} test --manifest-path {{manifest}} --workspace --all-features

# Format iac code
fmt:
  {{cargo}} fmt --manifest-path {{manifest}} --all

# Check iac code (fast compilation check)
check:
  {{cargo}} check --manifest-path {{manifest}} --workspace --all-features

# Lint with clippy
lint:
  {{cargo}} clippy --manifest-path {{manifest}} --workspace --all-features --all-targets -- -D warnings

# Audit security advisories (permissive — advisory db may be stale locally)
audit:
  cargo deny --manifest-path {{manifest}} check advisories || true

# Full quality gate: fmt → lint → test → audit
quality: fmt lint test audit

# ── Terraform ─────────────────────────────────────────────────────────────────

# Validate terraform formatting and syntax
tf-validate:
  terraform -chdir=iac/terraform fmt -check -recursive
  terraform -chdir=iac/terraform validate

# Plan terraform changes (human must `apply`)
tf-plan:
  terraform -chdir=iac/terraform plan
