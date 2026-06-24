# KVirtualStage (absorbed)

| Field | Value |
|-------|-------|
| **Source repo** | `_tmp_kvirtualstage` (local) / https://github.com/KooshaPari/KVirtualStage |
| **Absorption date** | 2026-06-24 |
| **Disposition** | ABSORB |
| **Canonical owner** | **phenotype-infra** (this repo) |

## Summary

`KVirtualStage` provided desktop virtualization, automation engine, credential management, GPU/media recording, and a Go SDK. Its components are now canonical in `phenotype-infra`; the source checkout at `_tmp_kvirtualstage` is preserved read-only for history.

## Absorbed component index

| Source path | Role | Canonical surface in phenotype-infra |
|-------------|------|-------------------------------------|
| `kvirtualdesktop/` | Rust desktop automation CLI | `crates/kvirtual/` |
| `kvirtualdesktop-core/` | Core types/traits | `crates/kvirtualdesktop-core/` |
| `kvirtualdesktop-mcp-server/` | MCP server | `crates/kvirtualdesktop-mcp-server/` |
| `credential_manager/` | Credential vault + crypto + OAuth2 | `crates/credential-manager/` |
| `kvirtualstage-go/` | Go SDK (CLI, server, TUI, client) | `tools/kvirtualstage-go/` |
| `architecture/*.md` | Architecture docs | `docs/absorbed/kvirtualstage/architecture/` |
| System summary docs | Provenance + evolution docs | `docs/absorbed/kvirtualstage/` |

## Provenance

- `docs/absorbed/kvirtualstage/BUILD.md` — detailed component map, workspace integration notes, verification commands
- All Rust crates compile under `cargo check -p <name>`
- Go SDK at `tools/kvirtualstage-go/` has its own `go.mod`/`Makefile`

## Do not

- Open new feature work against the archived `_tmp_kvirtualstage` checkout.
- Treat `_tmp_kvirtualstage` as a runtime or routing SSOT — use `phenotype-infra` crates, tools, and docs instead.
