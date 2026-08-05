# NanoVMS source of truth and compatibility

**Status:** active ownership policy
**Owner:** `KooshaPari/phenotype-infra`

NanoVMS is a foundation component of `phenotype-infra`. The canonical
implementation, tests, build metadata, and release changes live under
[`crates/nanovms-core/`](../../crates/nanovms-core/). New NanoVMS work must be
opened against `phenotype-infra`; it must not be added to a second checkout or
published only to the historical `KooshaPari/nanovms` repository.

## Repository and path mapping

| Historical reference | Canonical location now | Guidance |
| --- | --- | --- |
| `KooshaPari/nanovms` repository | `KooshaPari/phenotype-infra` | Read-only provenance and migration history; do not use it as the implementation source. |
| `cmd/nanovms`, `pkg/`, `internal/` | `crates/nanovms-core/{cmd,pkg,internal}` | Build and test from the nested Go module. |
| `nvms-sdk` examples under `sdk/rust` | `crates/nvms-ffi` and the workspace crates | Use the workspace package for new Rust integrations. |
| NanoVMS design and journey docs | `crates/nanovms-core/docs/` plus `docs/adr/` | Keep links relative to this checkout so they remain valid after the archive is unavailable. |

## Go module compatibility

The nested module currently declares `github.com/kooshapari/nanovms`. That
module path is retained for source compatibility with existing Go imports and
is **not** a pointer to the archived repository. Do not run `go mod init` to
change it as part of a documentation or migration-only change. New code should
import packages from the local module while working in
`phenotype-infra/crates/nanovms-core`; a module-path migration requires its own
compatibility plan and release note.

The Go import snippets in the ADRs are therefore intentionally left unchanged:
they document the compatibility surface, not an external checkout. When a
snippet needs to be executed, use the canonical nested module and verify its
package path against `go list ./...`.

## Ownership boundaries

- `crates/nanovms-core/` owns NanoVMS isolation adapters, Go core tests, and
  the NanoVMS-specific documentation.
- `crates/nvms-ffi/` owns the Rust FFI boundary to that core.
- `crates/pheno-compose/` owns composition and orchestration; it consumes the
  FFI/API boundary rather than copying NanoVMS adapters.
- `tools/byteport/` owns provider-neutral deployment and compute-mesh
  integration; it does not become a second NanoVMS implementation.

Substrate, `sharecli`, and `phenodag` remain owned by their respective
repositories or workspace components. This document records the NanoVMS
boundary only; it does not absorb those projects or authorize changes to their
contracts.

## Historical links and releases

The archived repository may be useful for provenance, but its issues, release
assets, container tags, and installation instructions are not current
`phenotype-infra` release interfaces. Use the canonical repository's
[issues](https://github.com/KooshaPari/phenotype-infra/issues), CI, and release
artifacts once published. If an old document must cite the archive for
historical comparison, label it as historical and link back to this page.
