# KodeVibeGo — Absorption Provenance

## Source

- **Original repository:** KodeVibe (standalone Go project)
- **Clone path:** `C:\Users\koosh\_tmp_kodevibego`
- **Absorption date:** 2026-06-24
- **Absorbed into:** `phenotype-infra/tools/kodevibego/`

## What was absorbed

The entire Go lint/analysis engine was migrated. This includes:

| Directory            | Purpose                                                     |
|----------------------|-------------------------------------------------------------|
| `cmd/cli/`           | CLI entrypoint (`kodevibe` binary)                          |
| `cmd/server/`        | HTTP server entrypoint (`kodevibe-server` binary)           |
| `internal/models/`   | Shared data types (Issue, ScanResult, Config, etc.)         |
| `internal/utils/`    | Utility functions (cache, metrics, git, formatting)         |
| `pkg/config/`        | Configuration loading/validation (viper + yaml)             |
| `pkg/dashboard/`     | Real-time dashboard with WebSocket                          |
| `pkg/fix/`           | Auto-fix engine for code issues                             |
| `pkg/mcp/`           | MCP client for AI-driven code analysis                      |
| `pkg/report/`        | Report generation (text, JSON, HTML, JUnit)                 |
| `pkg/scanner/`       | Core scanning engine with concurrent vibe checks            |
| `pkg/scoring/`       | Advanced scoring and quality metrics                        |
| `pkg/server/`        | HTTP REST API + WebSocket server (Gin)                      |
| `pkg/ui/`            | Interactive TUI                                             |
| `pkg/vibes/`         | Vibe checkers (security, code, perf, file, git, deps, doc)  |
| `pkg/watch/`         | File watcher with live re-scanning                          |

## What stayed behind

| Item                | Reason                                                                 |
|---------------------|------------------------------------------------------------------------|
| `vscode-extension/` | TypeScript VS Code extension — belongs in its own repo or PhenoMCP     |
| `docs/`             | Project documentation (standalone — moved with the project)            |
| `scripts/`          | Build/CI helper scripts                                                |
| `.github/`          | CI workflows — not relevant in monorepo context                        |
| Project docs        | README, CHANGELOG, SPEC.md, etc. — preserved as reference              |

## Module path

Changed from `kodevibe` to `github.com/KooshaPari/phenotype-infra/tools/kodevibego`.

## Build

```bash
cd tools/kodevibego
go build ./cmd/cli        # CLI binary
go build ./cmd/server     # Server binary
go test ./...             # Run tests
```

## Notes

- The `pkg/mcp/` package was kept with the engine because it depends on
  `internal/models` types. If a standalone Go MCP SDK is needed later, the
  MCP client types and protocol logic can be extracted into a separate Go
  module at that time.
