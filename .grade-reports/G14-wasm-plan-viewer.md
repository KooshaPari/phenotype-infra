# G14: WASM IaC Plan Viewer in Browser — Architecture Spike

**Date:** 2026-06-25
**Unit:** G14
**Type:** wasm/spike
**Epic:** epic_G — SOTA polish & external differentiation
**Pool:** infra

## 1. Current IaC Plan Viewing Workflow

The phenotype-infra monorepo uses Terraform (OpenTofu-compatible modules in `iac/terraform/`) for declarative
infrastructure definition across OCI, GCP, AWS, Cloudflare, Tailscale, and Hetzner. The current plan workflow is:

```
developer workstation                  remote CI (GitHub Actions)
┌──────────────────────┐              ┌──────────────────────────┐
│ terraform plan       │  ─────PR──→  │ terraform-plan.yml       │
│   (human, local)     │              │   ├─ fmt -check           │
│                      │              │   ├─ init -backend=false  │
│ stdout → copy/paste  │              │   └─ validate             │
│   → PR description   │              │                            │
│                      │              │ NOTE: plan requires creds; │
│ terraform apply      │              │ skipped in CI by design.   │
│   (human-only)       │              └──────────────────────────┘
└──────────────────────┘
```

**Pain points:**
- **No structured plan diff in PRs.** The `terraform plan` output is human-readable text — there is no
  structured, machine-parseable diff attached to PRs. Reviewers must mentally diff JSON or trust the PR
  author's textual summary.
- **No credential-free review surface.** Plans are currently impossible in CI because credentials are
  never committed. The `terraform-plan.yml` workflow explicitly skips `terraform plan` for this reason
  (see `iac/terraform/README.md`).
- **Collaboration bottleneck.** The person who runs `terraform plan` locally owns the plan artifact.
  Other team members cannot independently inspect or comment on resource diffs without re-running
  locally.
- **No visualization.** Terraform plan output is dense text. There is no interactive diff view,
  resource dependency graph, or change-impact summary.

## 2. How a WASM-Based Plan Viewer Would Work

The core idea is **compile the Terraform/OpenTofu plan parser to WASM** so plan files can be
parsed and rendered entirely in the browser without a server-side credentials path.

### 2.1 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Browser                                 │
│                                                                 │
│  ┌──────────────────────────────────────┐                       │
│  │  terraform.js (wasm-pack bundle)     │                       │
│  │  ┌──────────────────────────────┐   │                       │
│  │  │ hcl-json-parser.wasm         │   │  ← compiled from Rust │
│  │  │   (parses plan JSON output   │   │     via wasm-pack      │
│  │  │    into structured resource  │   │                       │
│  │  │    diffs)                    │   │                       │
│  │  └──────────────────────────────┘   │                       │
│  │  ┌──────────────────────────────┐   │                       │
│  │  │ diff-tree.wasm               │   │  ← compiled from Rust │
│  │  │   (computes resource-level   │   │     (standalone)       │
│  │  │    change sets, dependency   │   │                       │
│  │  │    ordering, impact scores)  │   │                       │
│  │  └──────────────────────────────┘   │                       │
│  └──────────────────────────────────────┘                       │
│                                                                 │
│  ┌──────────────────────────────────────┐                       │
│  │  Render Layer (TypeScript/React)     │                       │
│  │  ┌──────────┐ ┌───────────┐ ┌─────┐ │                       │
│  │  │ Diff View│ │Graph View │ │Impact│ │                       │
│  │  │  (table) │ │ (DAG viz) │ │Score │ │                       │
│  │  └──────────┘ └───────────┘ └─────┘ │                       │
│  └──────────────────────────────────────┘                       │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Data Flow

1. **User uploads or pastes a `terraform plan -json` output** (or a `.tfplan` binary decoded
   to JSON via `terraform show -json <plan>`).
2. **Plan JSON is parsed by the WASM module** running in a Web Worker to avoid blocking the
   UI thread. This module:
   - Validates the JSON schema against the Terraform plan format
   - Extracts resource changes (create/delete/update/replace)
   - Resolves attribute-level diffs
   - Identifies `Before` / `After` for each changed attribute
3. The WASM module consumes the parsed resource list and:
   - Builds a dependency graph from `depends_on` references
   - Computes a topological change ordering
   - Assigns an impact score per resource (e.g., `aws_instance.web` → HIGH impact,
     `cloudflare_record.www` → LOW impact)
4. **Browser render layer** displays:
   - A sortable, filterable table of resource changes (with before/after side-by-side)
   - A DAG visualization of resource dependencies (similar to `terraform graph`)
   - A collapsed-summary section (X resources to create, Y to destroy, Z to modify)
   - A "dangerous change" highlighter (replacement of stateful resources)

### 2.3 Key Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Parse format | `terraform show -json` output, not raw `.tfplan` | The JSON plan format is stable across Terraform 1.x and OpenTofu 1.x. Binary `.tfplan` is version-specific and harder to parse in WASM. |
| WASM compilation target | `wasm32-unknown-unknown` (no WASI) | No filesystem or system calls needed. Pure computation (parsing + diffing). |
| Web Worker | Dedicated Worker per viewer session | Avoids blocking UI during plan parse. Multiple tabs each get their own worker. |
| Graph rendering | d3-graphviz or cytoscape.js in JS | WASM handles data processing; JS handles DOM and SVG rendering. |
| Language for WASM modules | Rust (via `wasm-pack`) | The monorepo is Rust-native for operational crates (`iac/*`). Existing Rust culture and build tooling already known. |

## 3. Browser-Side Rendering Approach

### 3.1 Component Tree

```
<PlanViewer>
  ├── <PlanDropZone>          — Drag-and-drop or file-picker for plan JSON
  ├── <PlanSummary>           — Aggregated stats: create/destroy/modify counts
  ├── <PlanDiffTable>         — Sortable table of resource changes
  │   ├── <ResourceRow>       — One per changed resource
  │   │   └── <AttrDiff>      — Before/after for each changed attribute
  │   └── <Pagination>        — For large plans (1000s of resources)
  ├── <DependencyGraph>       — Interactive DAG of resource dependencies
  │   └── cytoscape.js / d3-force
  └── <ImpactSummary>         — High/Medium/Low impact breakdown
```

### 3.2 State Management

```typescript
interface PlanViewerState {
  rawPlanJson: string | null;
  parsed: ParsedPlan | null;
  parseError: string | null;
  parseStatus: 'idle' | 'parsing' | 'done' | 'error';
  sortColumn: string;
  sortDirection: 'asc' | 'desc';
  filterTier: 'all' | 'create' | 'destroy' | 'modify' | 'recreate';
  selectedResource: string | null;
}
```

### 3.3 No Server Backend Required

The entire viewer is **client-side only**. There is no server component, no API gateway, and
no credential path. The plan JSON is parsed entirely in the browser WASM runtime. This is the
critical advantage: it enables plan review **without granting CI or PR workflows access to
cloud credentials**.

Production deployment is a static site (GitHub Pages, Cloudflare Pages, or similar). No
backend infrastructure needed.

## 4. Integration Points with Current `iac/` Code

### 4.1 CI Workflow (`terraform-plan.yml`)

The existing `terraform-plan.yml` at `.github/workflows/terraform-plan.yml` currently skips
`plan` due to credentials. Integration options:

**Option A — Lightest touch:**
Add a step that users can optionally trigger with `workflow_dispatch` providing a JSON plan
artifact. The artifact is then linked from the PR. The viewer links to the artifact URL.

**Option B — PR comment bot:**
A GitHub Action that watches for plan JSON artifacts in PRs and posts a link to the WASM
viewer pre-loaded with that plan. The plan never touches a server — the JS loads it from
the artifact URL and processes it client-side.

**Option C — Offline companion:**
The user runs `terraform plan -json > plan.json` locally, then
`npx @phenotype/plan-viewer plan.json` opens a browser tab with the processed view.
This is the simplest integration and matches the current security posture
(no credentials in CI).

### 4.2 Existing Rust Crates (`iac/` workspace)

The `iac/` Rust workspace has the following relevant tooling:

| Crate | Relevance |
|-------|-----------|
| `oci-lottery` | A1.Flex capacity daemon that runs Terraform locally — could optionally emit structured plan JSON for the viewer |
| `oci-post-acquire` | Runs post-Terraform hooks — no direct plan relevance |
| `observability` | Existing observability tooling — could be extended to expose plan diff metrics |
| `phenotype-logging-stub` | Logging (indirect — used for worker telemetry if added) |

The WASM module would be a **new crate** under `iac/wasm-plan-viewer/` or
`tools/plan-viewer/` depending on whether it is infra-adjacent (lives in `iac/` workspace
as a separate member) or tooling (lives in `tools/` with its own build system).

**Recommendation:** New crate `iac/wasm-plan-viewer/` in the `iac/` workspace, since it is
directly tied to the IaC workflow and benefits from shared dependency pinning.

### 4.3 Terraform / OpenTofu Compatibility

The viewer must handle output from:
- **Terraform 1.7+** (pinned in `providers.tf`: `>= 1.7`)
- **OpenTofu 1.6+** (JSON plan format is API-compatible with Terraform)
- **`terraform show -json <planfile>`** output
- **`terraform plan -json`** streaming output

The JSON plan schema is defined by the Terraform JSON output format
(hashicorp/terraform/blob/main/docs/json-output-format.md). The WASM parser should
validate and normalize both Terraform and OpenTofu variants.

## 5. Effort Estimate and Dependencies

### 5.1 Phases

| Phase | Effort | Deliverable |
|-------|--------|-------------|
| **P1 — Prototype** | 2-3 days | WASM module that parses a Terraform plan JSON and outputs structured diffs. Bare-bones HTML page that shows a table. |
| **P2 — Interactive UI** | 4-5 days | Full React/TypeScript UI with diff table, sorting, filtering, pagination, before/after column view. Integrate with a DAG layout library. |
| **P3 — CI Integration** | 2-3 days | GitHub Action that uploads plan JSON as artifact and posts a viewer link on PRs. GitHub Pages deployment for the static viewer site. |
| **P4 — Polish** | 2-3 days | Performance optimization for 5000+ resource plans, keyboard navigation, copy-to-clipboard, URL-hash state sharing, dark mode. |
| **Total** | **10-14 days** | Full feature |

### 5.2 Dependencies

| Dependency | Version | License | Risk |
|------------|---------|---------|------|
| `wasm-pack` | ≥0.13 | MIT/Apache-2.0 | Low — widely used, maintained by rustwasm team |
| `wasm-bindgen` | ≥0.2.100 | MIT/Apache-2.0 | Low — core wasm tooling |
| `serde_json` / `serde` | 1.x (already in workspace) | MIT/Apache-2.0 | None — already a dependency |
| `cytoscape` or `d3-graphviz` | latest (npm) | MIT | Low — mature JS graph libraries |
| React 18+ | ≥18 (npm) | MIT | Low — team familiarity expected |
| `tinygo` (if Go route) | ≥0.35 | BSD-3 | Medium — team has more Rust muscle than Go-WebAssembly experience |

### 5.3 No-Go Risks

| Risk | Mitigation |
|------|-----------|
| Terraform JSON plan format changes | Pin parser to a specific plan schema version; add CI snapshot tests against fixture files |
| Large plans overwhelm browser WASM | Run WASM in a Web Worker; chunk processing; add progress callback; 5000-resource plan ≈ 10-15MB JSON — fine for modern browsers |
| Binary `.tfplan` files require `terraform show` | The viewer only accepts JSON input (from `terraform show -json` or `terraform plan -json`). The out-of-band decoding step is documented in the user workflow. |
| WASM compilation target gaps | Target `wasm32-unknown-unknown` (not WASI). Avoid `std::fs` and `std::net` — pure computation only. |

### 5.4 Existing WASM Code in Repo

The monorepo already has substantial WASM-adjacent infrastructure:
- **`crates/nanovms-core/`** (Go): `pkg/tier/wasm.go` — a WASM runtime adapter for running
  trusted workloads in sandboxes. WASM is a first-class execution tier in the NVMS system.
- **`tools/byteport/frontend/web/`**: Contains web frontend tooling with existing WASM
  dependency (e.g., `@oxc-parser/binding-wasm32-wasi`, `@rolldown/binding-wasm32-wasi` in
  `yarn.lock`). The web tooling infrastructure already exists and is understood.
- **`crates/pheno-compose/`**: `Tier::Wasm` sandbox support — shows WASM is an established
  runtime tier.

However, **no existing code compiles a WASM module for browser-side consumption** (no
`wasm-pack` build scripts, no `wasm-bindgen` attributes, no `#[wasm_bindgen]` exports).
This spike would be the first browser-WASM integration in the monorepo.

## 6. Recommendation

**Proceed to P1 (prototype).**

The WASM plan viewer directly addresses three documented pain points:
1. Removes the credential-in-CI blocker for plan review
2. Provides structured, diffable, visual plan output that can be shared in PRs
3. Leverages existing Rust infrastructure and WASM tooling already partially in the monorepo

The 2-3 day prototype (P1) will validate the critical risk: whether a Terraform plan JSON
can be parsed efficiently enough in browser WASM to be useful. If the P1 output is positive,
proceed with the full 10-14 day implementation.

## Appendix: Quick-Start P1 Prototype Plan

```bash
# Create the WASM crate in the iac/ workspace
cd iac
cargo new --lib wasm-plan-viewer
cd wasm-plan-viewer

# Cargo.toml additions:
#   [lib]
#   crate-type = ["cdylib"]
#   [dependencies]
#   wasm-bindgen = "0.2"
#   serde = { workspace = true, features = ["derive"] }
#   serde_json = { workspace = true }

# Build WASM
wasm-pack build --target web

# Serve
npx serve pkg/
```

The resulting `.wasm` binary hosts a `parse_plan(json: &str) -> JsValue` function callable from
TypeScript. A <100-line HTML page + inline script demonstrates the end-to-end flow.
