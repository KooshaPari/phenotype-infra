# Changelog

All notable changes to `phenotype-infra` are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Bootstrap `docs-lint.yml` (Vale + actionlint) GitHub Actions workflow.
- Repository hygiene stubs: `SECURITY.md`, `CONTRIBUTING.md`, `CODEOWNERS`.

### Notes

- `dependabot.yml` (github-actions weekly + terraform weekly across
  `iac/terraform/{,oci,gcp,aws,cloudflare}`) was already in place at repo
  creation and is unchanged.
- Existing workflows (`ansible-lint.yml`, `docs-check.yml`, `terraform-plan.yml`)
  predate this bootstrap pass and continue to run alongside `docs-lint.yml`.
