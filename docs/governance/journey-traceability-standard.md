# Journey Traceability Standard

**Canonical reference:** `phenotype-infra/docs/governance/journey-traceability-standard.md`
**Schema:** `phenotype-journeys/schema/manifest.schema.json` (JSON Schema draft/2020-12)
**Authority:** Phenotype org governance; adopted by all Phenotype-org repositories

---

## 1. Purpose

Journey traceability ensures that every user-facing specification carries
verifiable visual evidence of its execution. A journey manifest records the
exact state of an interface at each step of a flow — as keyframes (PNG frames)
and a full recording (tape/GIF) — so that:

- Agents can reason about what a feature *actually* looks like at runtime.
- Reviewers can audit claims against captured ground truth.
- CI gates can fail when a spec is updated but its journey is not revalidated.
- Onboarding and regression testing have a replayable artefact.

If a flow is important enough to document, it is important enough to capture
with a keyframe gallery and a replayable recording.

---

## 2. Canonical Manifest Schema

Every journey is serialised as a JSON file conforming to the Phenotype
`Manifest` schema (`phenotype-journeys/schema/manifest.schema.json`).

### Top-level fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `string` | **Yes** | Unique journey identifier (e.g. `plan-deepseek`, `fleet-register`) |
| `intent` | `string` | **Yes** | One-sentence purpose of the journey |
| `recording` | `string \| null` | No | Path to the full tape recording file |
| `recording_gif` | `string \| null` | No | Path to the animated GIF preview |
| `keyframe_count` | `integer` | No | Number of extracted keyframes (default `0`) |
| `passed` | `boolean` | No | Whether the journey passed its last verification (default `false`) |
| `steps` | `Step[]` | No | Ordered list of steps (default `[]`) |
| `verification` | `Verification \| null` | No | Verification result object |

### Example top-level structure

```json
{
  "id": "plan-deepseek",
  "intent": "Demonstrate the hwledger plan command with DeepSeek-V2 model selection",
  "recording": "cli-journeys/tapes/plan-deepseek.tape",
  "recording_gif": "cli-journeys/tapes/plan-deepseek.gif",
  "keyframe_count": 9,
  "passed": true,
  "steps": [ ... ],
  "verification": { ... }
}
```

The `manifest.verified.json` suffix is reserved for the output of a passing
verification run. Source manifests authored by humans carry no suffix.

---

## 3. Step Structure

Each entry in `steps` captures one discrete UI state:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `index` | `integer` | **Yes** | Zero-based step ordinal |
| `slug` | `string` | **Yes** | Short kebab-case identifier for the step (e.g. `model-select`) |
| `intent` | `string` | **Yes** | What the user *intends* to accomplish at this step |
| `screenshot_path` | `string` | **Yes** | Path to the keyframe PNG (e.g. `keyframes/plan-deepseek/frame-003.png`) |
| `description` | `string \| null` | No | Short prose description of what the frame shows |
| `judge_score` | `number \| null` | No | Judge pass score (0.0–1.0) from last verification |
| `assertions` | `StepAssertions \| null` | No | Hard OCR assertions for this step |
| `annotations` | `Annotation[] \| null` | No | Visual callouts overlaid on the keyframe |

### Step example

```json
{
  "index": 2,
  "slug": "model-selection",
  "intent": "User selects DeepSeek-V2 from the model picker",
  "screenshot_path": "cli-journeys/keyframes/plan-deepseek/frame-003.png",
  "description": "Model picker with DeepSeek-V2 highlighted",
  "judge_score": 0.91,
  "assertions": {
    "must_contain": ["DeepSeek-V2", "hwledger"],
    "must_not_contain": ["error:"],
    "ocr_required": false
  },
  "annotations": [
    {
      "bbox": [120, 340, 220, 28],
      "label": "MLA (kv_lora_rank=512)",
      "color": "#89b4fa",
      "style": "solid",
      "kind": "region",
      "note": "Planner auto-detected Multi-head Latent Attention",
      "position": "top-right"
    }
  ]
}
```

---

## 4. Annotation Types

Annotations are bounding-box callouts overlaid on keyframe images. Coordinates are
in source-image pixels (top-left origin). All annotation fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bbox` | `[x, y, w, h]` (4 integers) | **Yes** | Bounding box in source-image pixels |
| `label` | `string` | **Yes** | Blind label — must match what the image *actually* shows |
| `color` | `string \| null` | No | Hex colour for the callout border/fill (e.g. `#89b4fa`) |
| `style` | `"solid" \| "dashed"` | No | Callout border style (default `"solid"`) |
| `kind` | `"region" \| "pointer" \| "highlight"` | No | Annotation kind (default `"region"`) |
| `note` | `string \| null` | No | Extra context for reviewers (not shown to blind judges) |
| `position` | `string` | No | Callout anchor position: `auto`, `top-left`, `top-right`, `bottom-left`, `bottom-right`, `center`, `center-top`, `center-bottom`, `custom` |
| `custom` | `{ x, y } \| null` | No | Custom anchor coordinates when `position: "custom"` |

### Annotation kinds

| Kind | Use |
|------|-----|
| `region` | Highlight a UI region or element |
| `pointer` | Indicate a cursor or interaction point |
| `highlight` | Full-area tint for emphasis |

> **Blind label rule:** The `label` must reflect what the image *actually*
> contains. Do not use labels that would only make sense to someone who already
> knows the journey's intent. A judge receiving only the blind description
> should be able to verify the label without consulting the step `intent`.

### Position variety

Vary `position` across consecutive frames so callouts do not stack in the same
corner. Use `auto` to let the renderer pick based on the bbox anchor.

---

## 5. Assertions

Assertions are **hard gates** applied via OCR. They fail the CI build if violated,
regardless of what the judge model scores.

### StepAssertions fields

| Field | Type | Description |
|-------|------|-------------|
| `must_contain` | `string[]` | Every listed substring must appear in the OCR of the keyframe |
| `must_not_contain` | `string[]` | None of the listed substrings may appear |
| `expected_exit` | `integer \| null` | Set on the LAST step only; last keyframe must contain `__EXIT_<N>__` |
| `ocr_required` | `boolean` | Reserved for future "OCR must succeed" gating (default inferred from presence of contain/not_contain) |

### Exit-code sentinel (canonical tape pattern)

Wrap the final command in the VHS tape so the sentinel lands visibly in the
recording:

```vhs
Type "hwledger plan --help; echo __EXIT_$?__"
Enter
Sleep 500ms
```

The `__EXIT_0__` (or `__EXIT_N__`) appears in the last frame. The `assert`
subcommand OCR-scans it and gates on it.

### CLI usage

```bash
phenotype-journey assert docs/journeys/manifests/<spec-id>/manifest.verified.json --strict
```

With `--strict`, exits non-zero when any assertion is violated. Without the flag
the report prints but the process exits 0.

> **Assertion-free journeys:** A journey manifest with zero assertions prints a
> loud warning so they cannot hide.

---

## 6. Agreement Report

The verification loop produces a `Verification` block in the manifest:

```json
{
  "overall_score": 0.94,
  "describe_confidence": 0.97,
  "judge_confidence": 0.91,
  "all_intents_passed": true,
  "mode": "api",
  "timestamp": "2026-04-30T12:00:00Z",
  "assertion_violations": []
}
```

### Intent vs. blind-description comparison

For each step, the verify loop:

1. **Describe** — sends the keyframe to a vision model with *only* the `intent`
   field, asking for a blind natural-language description of what the image
   contains.
2. **Judge** — scores the match between the `intent` and the blind description.
3. **Assert** — runs OCR assertions against the keyframe.

The `overall_score` is the geometric mean of `describe_confidence` and
`judge_confidence`. The `all_intents_passed` boolean reflects whether all judge
scores exceeded the threshold (configurable, default 0.7).

---

## 7. Verification Modes

| Mode | API key required | Describe | Judge | Assertions |
|------|-----------------|----------|-------|------------|
| `mock` | No | Canned responses | Canned scores | Runs |
| `api` | Yes (`ANTHROPIC_API_KEY`) | Real vision model | Real scoring | Runs |

```bash
# Mock mode (offline, no key needed)
phenotype-journey verify journeys/manifests/plan-deepseek/manifest.json

# Live mode
ANTHROPIC_API_KEY=sk-ant-... phenotype-journey verify \
  journeys/manifests/plan-deepseek/manifest.json --live
```

The `verification.mode` field in the output manifest records which mode was used.

---

## 8. Asset Layout

```
<project>/
├── docs/
│   ├── journeys/
│   │   └── manifests/
│   │       └── <journey-id>/
│   │           ├── manifest.json              # source manifest
│   │           └── manifest.verified.json     # post-verification output
│   └── public/
│       └── journeys/
│           └── <journey-id>/
│               ├── keyframes/
│               │   └── frame-###.png          # numbered 001, 002, …
│               ├── recording.tape            # VHS source tape
│               └── recording.gif             # animated preview
└── cli-journeys/                             # alternative location (legacy)
    └── keyframes/
        └── <journey-id>/
            └── frame-###.png
```

- **Keyframe filenames** — zero-padded three-digit ordinal: `frame-001.png`,
  `frame-002.png`, … `frame-042.png`.
- **Tape `id`** — the tape file path is recorded as the `recording` field in the
  manifest. The `tape id` in `recording_gif` refers to the same artefact.
- **Frame index** — 1-based in `shot-annotations.yaml`; `index` in the manifest
  JSON is 0-based. Convert accordingly.

---

## 9. CI Gate

A `manifest.verified.json` **must** exist for every spec tagged as user-facing.
CI fails if any such manifest is missing or if verification fails.

### Generic workflow template

See [`ci-journey-gate.yml`](./ci-journey-gate.yml) for the canonical reusable
workflow. The key logic:

```yaml
name: Journey Gate
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  journey-gate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Validate journey manifests
        run: |
          MANIFESTS=$(find . -name "manifest.verified.json" \
            -not -path "*/node_modules/*" \
            -not -path "*/target/*" \
            -not -path "*/.git/*" 2>/dev/null)

          if [ -z "$MANIFESTS" ]; then
            echo "ERROR: No journey manifests found."
            echo "Add docs/journeys/manifests/<spec>/manifest.verified.json"
            echo "Once manifests exist, remove --warn-only below."
            exit 1
          fi

          echo "Found journey manifests:"
          echo "$MANIFESTS"

          for manifest in $MANIFESTS; do
            echo "Validating $manifest"
            phenotype-journey validate "$manifest" || exit 1
            phenotype-journey assert "$manifest" --strict || exit 1
          done
```

To include in a repo, copy the workflow file and commit it as
`.github/workflows/journey-gate.yml`. Do not modify the logic; extend the
template only for repo-specific paths.

---

## 10. Screenshot Placeholder Policy

A `manifest.json` that references non-existent or placeholder keyframes is
**not** a valid manifest.

| State | CI result |
|-------|-----------|
| `docs/journeys/manifests/<spec>/manifest.json` missing | **FAIL** |
| Keyframe PNGs missing or SS-`PLACEHOLDER` | **FAIL** |
| `manifest.verified.json` missing | **FAIL** (user-facing specs only) |
| All keyframes present, verification passes | **PASS** |

> **SS placeholder rule:** Every keyframe PNG must be a real capture from an
> actual run. CI must fail until placeholder screenshots are replaced with
> validated journey runs. Do not use generic stock screenshots.

To create a stub manifest:

```bash
phenotype-journey init <journey-name>
```

This scaffolds the manifest with empty steps. Fill in steps and run
`phenotype-journey record` to populate keyframes before re-running the gate.

---

## 11. Video Requirements

Video artefacts derived from journey keyframes must meet these requirements:

| Requirement | Detail |
|-------------|--------|
| **TTS narration** | Every user-facing video must include voice-over narration explaining the flow |
| **Professional editing** | Cut cleanly between keyframes; no abrupt jumps or jarring transitions |
| **No duplicate keyframes** | Each keyframe used as a video moment must represent a distinct UI state |
| **Smooth derivation** | Video frames should derive visually smoothly from keyframe captures (e.g. via scaling, not hard-cut replacement) |
| **Recording linked** | Video description or caption must link back to the original tape/recording file |

> **No keyframe duplication:** Do not use the same keyframe as two consecutive
> video moments just to extend video length. If a step has no meaningful state
> change, skip it or use a transition effect.

---

## 12. Annotation Requirements

| Requirement | Detail |
|-------------|--------|
| **Blind labels required** | Every annotation must have a `label` that describes what the image actually shows, not why it is important |
| **Label precision** | Labels must match image content precisely; do not annotate with intent or expected-value labels |
| **Cursor in recordings** | Edit cursor movements out of tape recordings before keyframe extraction; only final UI state matters |
| **OCR assertions** | Use `must_contain` / `must_not_contain` on text-heavy steps |
| **Position variety** | Vary `position` across consecutive frames to avoid callout stacking |

### Cursor editing example

If the recording captures the cursor moving to a button, trim the tape so the
keyframe shows only the result of the click, not the cursor mid-flight.

---

## 13. MSOT Captures

MSOT (Multi-Step Operating Thread) captures are **separate** from keyframes.
They document edge cases, error paths, or specific failure states.

| Rule | Detail |
|------|--------|
| **Separate directory** | Store MSOT captures in `keyframes/<journey>/msot/` (not in the main keyframe set) |
| **Separate manifest entry** | MSOT steps are tracked in a separate manifest (e.g. `manifest.msot.json`) |
| **Separate annotations** | Annotate MSOT frames independently from normal keyframe annotations |
| **No CI gate on MSOT** | MSOT manifests are informational only; they do not gate CI by default |

---

## 14. Adoption Checklist

Complete this checklist when adding journey traceability to a new repository.

### Pre-work

- [ ] Identify all specs tagged as `user-facing` in the repo.
- [ ] Confirm `phenotype-journey` CLI is available in the project PATH
  (`brew install phenotype-journey` or via `cargo install`).
- [ ] Confirm `tesseract` OCR is installed: `brew install tesseract` (macOS) or
    `apt-get install tesseract-ocr` (Debian). Exit 1 if missing.

### Per-spec journey setup

For each user-facing spec `<spec-id>`:

- [ ] `phenotype-journey init docs/journeys/manifests/<spec-id>`
- [ ] Write step `intent` fields for each meaningful UI state.
- [ ] Run `phenotype-journey record --tape <path> --out docs/journeys/`
- [ ] Extract keyframes: `phenotype-journey extract docs/journeys/tapes/<tape>`
- [ ] Verify in mock mode: `phenotype-journey verify docs/journeys/manifests/<spec-id>/manifest.json`
- [ ] Add OCR assertions to steps: `must_contain`, `must_not_contain`
- [ ] Add annotations to each step (at minimum: one `region` annotation with a
    blind `label` per step)
- [ ] Run with assertions: `phenotype-journey assert docs/journeys/manifests/<spec-id>/manifest.json --strict`
- [ ] Run live verification (if `ANTHROPIC_API_KEY` available):
    `phenotype-journey verify docs/journeys/manifests/<spec-id>/manifest.json --live`
- [ ] Commit `manifest.verified.json` alongside the spec.

### CI integration

- [ ] Copy `ci-journey-gate.yml` to `.github/workflows/journey-gate.yml` in the repo.
- [ ] Confirm the workflow runs on push and PR to `main`.
- [ ] Remove the `exit 1` stub once at least one `manifest.verified.json` exists.
- [ ] Verify CI fails when a keyframe PNG is missing or a new spec is added without a manifest.

### Maintenance

- [ ] Re-verify after any spec change that alters the UI flow.
- [ ] Sync keyframes to `docs/public/journeys/`:
    `phenotype-journey sync --from journeys --to docs/public/journeys`
- [ ] Update `manifest.verified.json` after re-verification; do not commit stale verified manifests.

---

## References

- Schema source: `phenotype-journeys/schema/manifest.schema.json`
- CLI: `phenotype-journeys/bin/phenotype-journey/`
- Core library: `phenotype-journeys/crates/phenotype-journey-core/`
- Shot annotations registry: `phenotype-journeys/data/shot-annotations.yaml`
- thegent journey guide: `thegent/docs/operations/journey-traceability.md`
- Example journeys: `hwLedger/apps/cli-journeys/` (legacy; migrating to this standard)
