<!--
  phenotype-infra — Compute/Infra Consolidation Monorepo

  This monorepo is created as part of the L1-Alpha wave of the Master DAG
  (Compute/Infra + Observability Consolidation). It absorbs seven previously
  separate repositories into a single polyglot workspace.

  Slop issues are expected and intentionally present as part of an HITL-less /
  minimized AI-DD metaproject of learning, refining, and building brute-force
  training for both agents and the human operator.
-->

![License](https://img.shields.io/github/license/KooshaPari/phenotype-infra?style=flat-square)
![AI-Slop](https://img.shields.io/badge/AI--DD-Slop%20Expected-orange?style=flat-square)
![AI-Only-Maintained](https://img.shields.io/badge/Planned%20%26%20Maintained%20by-AI%20Agents%20Only-red?style=flat-square)

# phenotype-infra — Compute/Infra Consolidation Monorepo

> **Consolidation target** for nanovms (Node.js, 3-tier isolation) + PhenoCompose (Rust FFI + driver) + BytePort (Svelte tooling) + absorbed forks (HexaKit `golangci-ext`, KVirtualStage `kvirtualdesktop-{core,mcp-server}`, thegent `thegent-utils`).

This monorepo is created as part of the **L1-Alpha** wave of the Master DAG
(Compute/Infra + Observability Consolidation). It absorbs seven previously
separate repositories into a single polyglot workspace:

| Component | Source | Language | Role |
|-----------|--------|----------|------|
| **nanovms-core** | `nanovms` | Node.js | 3-tier isolation (WASM/gVisor/Firecracker) — scaffolded stub |
| **kodevibego-ffi** | `KodeVibeGo` | Go → Rust FFI | Lint-tooling FFI bridge |
| **nvms-ffi** | `PhenoCompose` | Rust | `extern "C"` FFI bindings |
| **pheno-compose** | `PhenoCompose` | Rust | High-level Rust driver |
| **pheno-config** | `PhenoCompose` | Rust | Shared configuration (TOML/env) |
| **credential-manager** | `KVirtualStage` | Rust | Cross-platform credential vault |
| **kvirtualdesktop-core** | `KVirtualStage` | Rust | MCP protocol types + transport + OAuth2/JWT/PKCE |
| **kvirtualdesktop-mcp-server** | `KVirtualStage` | Rust | MCP server binary (frozen — pre-existing errors, kept for ref) |
| **kvirtual** | `KVirtualStage` | Rust | CLI front-end |
| **thegent-utils** | `thegent-workspace` | Rust | env/`which` detection helpers |
| **byteport** | `BytePort` | Svelte/TS | Infra tooling UI |

## Architecture

```
phenotype-infra/
├── crates/
│   ├── nanovms-core/             # Go source → libnvms_core.a (via CGo)
│   ├── nvms-ffi/                 # Rust FFI bindings
│   ├── pheno-compose/            # High-level Rust driver
│   ├── pheno-config/             # Shared configuration
│   ├── kodevibego-ffi/           # Go→Rust FFI for KodeVibeGo tooling
│   ├── credential-manager/       # Rust credential vault
│   ├── kvirtualdesktop-core/     # MCP protocol + OAuth2/PKCE/JWT
│   ├── kvirtualdesktop-mcp-server/ # MCP server (frozen)
│   ├── kvirtual/                 # CLI front-end
│   └── thegent-utils/            # env/`which` detection helpers
├── tests/
│   └── integration_test.rs       # 28 cross-crate integration tests
├── tools/
│   └── byteport/                 # Svelte infra tooling
├── docs/
│   ├── adr/                      # Architecture Decision Records
│   ├── specs/                    # Specifications
│   ├── governance/               # Governance docs
│   └── audit/                    # Audit scorecards (71+ pillars)
└── .github/
    ├── workflows/                # CI/CD
    └── protection/               # Branch protection JSON policies
```

## Quick Start

```bash
# Build Rust workspace (10 crates)
cargo build --workspace

# Run all integration tests (28 tests)
cargo test --workspace

# Quick check across all crates
cargo check --workspace

# Build Go static lib for nvms-ffi Mode A (prebuilt)
make nvms-c-archive

# BytePort (Svelte frontend)
cd tools/byteport && npm install && npm run dev
```

## Branches

- `master` — default, stable
- `kvd/oauth2-5x-fixes` — kvirtualdesktop-core oauth2 5.x API migration
- `feat/nanovms-unarchive` — nanovms un-archive + Node.js grade integration

## License

Apache-2.0