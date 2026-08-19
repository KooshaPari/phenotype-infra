# DESIGN.md — phenotype-infra

## Overview

**phenotype-infra** is the polyrepo compute mesh orchestrator for the phenotype ecosystem. It provisions and manages infrastructure across OCI, Cloudflare Workers, GCP, and AWS using Rust crates, Terraform modules, and Ansible playbooks.

## Architecture

```
phenotype-infra/
├── crates/                # Rust crates (orchestrator, OCI client, CF tunnel)
│   ├── orchestrator/      # Core orchestration logic
│   ├── oci-client/        # OCI registry interactions
│   └── cf-tunnel/         # Cloudflare Tunnel management
├── iac/                   # Infrastructure-as-Code
│   ├── terraform/         # Terraform modules per cloud provider
│   └── ansible/           # Ansible playbooks for config management
├── cloudflare-tunnel/     # Tunnel config and deployment
├── configs/               # Environment-specific configuration
├── scripts/               # Operational scripts
├── tests/                 # Integration tests
└── tools/                 # Dev tooling and utilities
```

## Key Design Decisions

1. **Rust orchestrator** — type-safe cloud API interactions with compile-time validation
2. **Multi-cloud by default** — OCI primary, CF Workers for edge, GCP/AWS for overflow
3. **Terraform + Ansible split** — Terraform for provisioning, Ansible for post-provision config
4. **Polyrepo-aware** — orchestrator tracks repos and their infrastructure needs per-deployment

## Data Flow

```
Orchestrator (Rust) → Terraform Apply → Cloud Provisioning → Ansible Config → Health Check → CF Tunnel Registration
```

## Non-Goals

- Kubernetes cluster management (relies on cloud-native K8s offerings)
- Cost optimization / FinOps dashboards (tracked in phenotype-org-audits)
- CI/CD pipeline management (handled by GitHub Actions in each repo)

## Status

- v0.x — Active development with validation reports (A-01 through A-10)
- Cloudflare Tunnel integration operational
- Multi-cloud provisioning partially complete

## References

- [AGENTS.md](./AGENTS.md) — LLM contributor guidelines
- [grade.sh](./grade.sh) — Quality gate runner
- [A-*_VALIDATION.md](./A-01_VALIDATION.md) — Validation reports per component
