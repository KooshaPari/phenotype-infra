# Iconography Standard

**Canonical reference:** `phenotype-infra/docs/governance/iconography-standard.md`
**Authority:** Phenotype org governance; adopted by all Phenotype-org repositories

---

## 1. Overview

Phenotype org uses three distinct icon styles to serve different UI contexts.
All icons are authored as 24×24 SVG files with strict technical rules that
enable dark/light mode switching via `currentColor`, enforce accessibility, and
guarantee consistent rendering at common sizes (16, 20, 24, 32, 48 px).

| Style | Aesthetic | Primary use |
|-------|-----------|-------------|
| **Fluent** | Stroke-based, Apple-like, 1.5 px stroke, rounded caps/joins | macOS, iOS, product UIs, documentation |
| **Material** | Filled + outlined variants, 2 px outline stroke, optical alignment | Android, cross-platform, design-system tokens |
| **Liquid Glass** | Blur backdrop, gradient fills, translucency, iOS 25 style | Hero sections, marketing, premium features |

---

## 2. SVG Technical Specification

Every icon SVG **must** conform to all of the following rules.

### 2.1 Root element

```xml
<svg xmlns="http://www.w3.org/2000/svg"
     viewBox="0 0 24 24"
     width="24" height="24"
     role="img"
     aria-label="...descriptive label..."
     focusable="false">
```

| Attribute | Required | Value | Reason |
|-----------|----------|-------|--------|
| `xmlns` | Yes | `http://www.w3.org/2000/svg` | SVG namespace |
| `viewBox` | Yes | `0 0 24 24` | Consistent scaling |
| `width` | Yes | `24` | Default render size |
| `height` | Yes | `24` | Default render size |
| `role` | Yes | `img` | Accessibility landmark |
| `aria-label` | Yes | non-empty string | Screen-reader text |
| `focusable` | Yes | `false` | Prevents keyboard focus ring |

### 2.2 Colour

- **No hardcoded `fill` or `stroke` colours.** Use `currentColor` for both.
- The parent element's `color` CSS property controls the icon colour in both
  light and dark modes.
- The SVG root must **not** carry a `fill` or `stroke` attribute unless it is
  `fill="none"` or `stroke="currentColor"`.

```xml
<!-- Correct -->
<path stroke="currentColor" stroke-width="1.5" ... />

<!-- Incorrect -->
<path stroke="#000000" ... />
<circle fill="#f38ba8" ... />
```

### 2.3 Accessibility

Every icon must include a `<title>` and `<desc>` inside the SVG:

```xml
<title>Settings</title>
<desc>A gear wheel with six teeth, representing system settings</desc>
```

These elements are read by assistive technologies and improve document outline
navigation.

### 2.4 File naming

```
<icon-name>.svg
```

Use lowercase kebab-case. Example: `settings.svg`, `arrow-left.svg`,
`external-link.svg`.

---

## 3. Fluent Style

### 3.1 Anatomy

Fluent icons are **stroke-based**: the shape is defined by a path with a
consistent stroke and no fill (or `fill="none"`).

| Property | Value |
|----------|-------|
| `stroke-width` | `1.5` |
| `stroke-linecap` | `round` |
| `stroke-linejoin` | `round` |
| `fill` | `none` (or `currentColor` for solid glyphs) |
| `color` inheritance | `currentColor` |

### 3.2 Corner radius

Prefer rounded corners. For rectangles use `rx="1.5"` to `rx="2"`:

```xml
<rect x="2" y="2" width="20" height="20" rx="2"
      fill="none" stroke="currentColor" stroke-width="1.5" />
```

### 3.3 Consistent optical weight

Keep stroke density uniform within a single icon. Do not mix 1 px and 2 px
strokes in the same icon.

### 3.4 Fluent icon set — 20 core icons

```
docs/operations/iconography/fluent/
├── home.svg
├── settings.svg
├── search.svg
├── user.svg
├── folder.svg
├── file.svg
├── terminal.svg
├── build.svg
├── deploy.svg
├── api.svg
├── database.svg
├── key.svg
├── shield.svg
├── clock.svg
├── chart-bar.svg
├── alert-triangle.svg
├── check-circle.svg
├── x-circle.svg
├── arrow-left.svg
├── arrow-right.svg
├── chevron-up.svg
├── chevron-down.svg
├── chevron-left.svg
├── chevron-right.svg
├── plus.svg
├── minus.svg
├── copy.svg
├── paste.svg
├── cut.svg
├── link.svg
└── external-link.svg
```

### 3.5 Fluent SVG template

```xml
<svg xmlns="http://www.w3.org/2000/svg"
     viewBox="0 0 24 24"
     width="24" height="24"
     role="img"
     aria-label="Home"
     focusable="false">
  <title>Home</title>
  <desc>A house outline with a pointed roof and door</desc>
  <path d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
        fill="none" />
</svg>
```

---

## 4. Material Style

### 4.1 Anatomy

Material icons have two variants:

| Variant | Stroke | Fill |
|---------|--------|------|
| **Outline** | 2 px stroke, `fill="none"` | Transparent |
| **Filled** | No stroke | `fill="currentColor"` |

### 4.2 Outline variant

```xml
<path d="M..." stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none" />
```

### 4.3 Filled variant

```xml
<path d="M..." fill="currentColor" />
```

### 4.4 Optical alignment

Material icons are optically centred. The geometric centre may differ from the
visual centre due to glyph weight. Adjust `transform` sparingly to correct
optical drift:

```xml
<g transform="translate(0.5, 0)">
  <!-- icon paths -->
</g>
```

### 4.5 Material icon set structure

```
docs/operations/iconography/material/
├── home-outline.svg
├── home-filled.svg
├── settings-outline.svg
├── settings-filled.svg
├── search-outline.svg
├── search-filled.svg
├── user-outline.svg
├── user-filled.svg
├── folder-outline.svg
├── folder-filled.svg
├── file-outline.svg
├── file-filled.svg
├── terminal-outline.svg
├── terminal-filled.svg
├── build-outline.svg
├── build-filled.svg
├── deploy-outline.svg
├── deploy-filled.svg
├── api-outline.svg
├── api-filled.svg
├── database-outline.svg
├── database-filled.svg
├── key-outline.svg
├── key-filled.svg
├── shield-outline.svg
├── shield-filled.svg
├── clock-outline.svg
├── clock-filled.svg
├── chart-bar-outline.svg
├── chart-bar-filled.svg
├── alert-triangle-outline.svg
├── alert-triangle-filled.svg
├── check-circle-outline.svg
├── check-circle-filled.svg
├── x-circle-outline.svg
└── x-circle-filled.svg
```

---

## 5. Liquid Glass Style

### 5.1 Aesthetic

Liquid Glass icons use backdrop blur, gradient fills, and translucency to
replicate the iOS/macOS 25 "glass" aesthetic. They are decorative and
attention-directing; they should **not** be used for functional navigation.

### 5.2 Backdrop blur

Use SVG `feGaussianBlur` filter for the glass effect:

```xml
<defs>
  <filter id="glass-blur" x="-20%" y="-20%" width="140%" height="140%">
    <feGaussianBlur in="SourceGraphic" stdDeviation="3" result="blur" />
    <feColorMatrix in="blur" type="matrix"
      values="1 0 0 0 0
              0 1 0 0 0
              0 0 1 0 0
              0 0 0 0.6 0"
      result="frosted" />
  </filter>
</defs>
```

### 5.3 Gradient fills

Use `linearGradient` for glass highlights:

```xml
<linearGradient id="glass-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
  <stop offset="0%" stop-color="white" stop-opacity="0.4" />
  <stop offset="100%" stop-color="white" stop-opacity="0.1" />
</linearGradient>
```

### 5.4 Translucency layers

Stack at least two layers to achieve the glass depth:

```xml
<!-- Base shape -->
<rect x="2" y="2" width="20" height="20" rx="4"
      fill="url(#glass-gradient)" filter="url(#glass-blur)" />
<!-- Inner highlight stroke -->
<rect x="2" y="2" width="20" height="20" rx="4"
      stroke="rgba(255,255,255,0.3)" stroke-width="1" fill="none" />
```

### 5.5 iOS/macOS style adaptation

- Corner radius: `rx="6"` to `rx="8"` for a pillowy feel.
- Use `backdrop-filter` CSS in HTML wrappers: `backdrop-filter: blur(12px)`.
- Fallback: solid background + reduced opacity when `backdrop-filter` is
  unsupported.

---

## 6. Combined Icon Sprite

Every repo that uses icons **must** maintain a combined SVG sprite at:

```
docs/operations/iconography/icons.svg
```

The sprite concatenates all icon `<symbol>` definitions, enabling `<use
href="#icon-name">` consumption without per-file HTTP requests.

### 6.1 Sprite format

```xml
<svg xmlns="http://www.w3.org/2000/svg" style="display:none">
  <symbol id="icon-home" viewBox="0 0 24 24" role="img">
    <title>Home</title>
    <desc>A house outline</desc>
    <path d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2..."
          stroke="currentColor" stroke-width="1.5"
          stroke-linecap="round" stroke-linejoin="round" fill="none" />
  </symbol>
  <symbol id="icon-settings" viewBox="0 0 24 24" role="img">
    <title>Settings</title>
    <desc>A gear wheel</desc>
    <!-- ... -->
  </symbol>
  <!-- additional symbols -->
</svg>
```

### 6.2 Consumption

```html
<svg width="24" height="24" aria-label="Home" focusable="false">
  <use href="#icon-home" />
</svg>
```

---

## 7. Per-Repo Structure

Each Phenotype org repository using icons must maintain:

```
<project>/
├── docs/
│   └── operations/
│       └── iconography/
│           ├── fluent/          # 24×24 Fluent SVGs
│           ├── material/        # 24×24 Material outline + filled SVGs
│           ├── liquid-glass/    # 24×24 Liquid Glass SVGs
│           ├── icons.svg        # Combined sprite (auto-generated)
│           └── README.md        # Repo-specific notes, any deviations
└── .github/
    └── workflows/
        └── icon-lint.yml       # SVG lint CI workflow (see §8)
```

**Do not commit icon files outside this structure.** If icons are consumed via
NPM packages (e.g. `@fluentui/react-icons`), the sprite is still required in
`docs/` for documentation and docsite use.

---

## 8. CI Enforcement

### 8.1 SVG lint workflow

Create `.github/workflows/icon-lint.yml` in the repo:

```yaml
name: Icon Lint

on:
  push:
    paths:
      - 'docs/operations/iconography/**/*.svg'
  pull_request:
    paths:
      - 'docs/operations/iconography/**/*.svg'

jobs:
  lint-icons:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Lint SVG files
        run: |
          set -e
          ICONS=$(find docs/operations/iconography -name "*.svg" -type f)

          if [ -z "$ICONS" ]; then
            echo "No SVG icons found — skipping."
            exit 0
          fi

          for icon in $ICONS; do
            echo "Linting $icon"

            # 1. Valid XML
            python3 -c "import xml.etree.ElementTree as ET; ET.parse('$icon')" \
              || { echo "FAIL: $icon is not valid XML"; exit 1; }

            # 2. Required attributes
            python3 - <<'PYEOF' || exit 1
            import xml.etree.ElementTree as ET
            import sys

            tree = ET.parse('$icon')
            root = tree.getroot()

            errors = []
            if root.get('viewBox') != '0 0 24 24':
                errors.append(f"viewBox must be '0 0 24 24', got '{root.get('viewBox')}'")
            if root.get('role') != 'img':
                errors.append(f"role must be 'img', got '{root.get('role')}'")
            if not root.get('aria-label'):
                errors.append("aria-label is required")
            if root.get('focusable') != 'false':
                errors.append("focusable must be 'false'")

            # 3. No hardcoded fill/stroke colours
            for el in root.iter():
                fill = el.get('fill', '')
                stroke = el.get('stroke', '')
                if fill not in ('', 'none', 'currentColor') and not fill.startswith('url('):
                    errors.append(f"hardcoded fill='{fill}' in {el.tag}; use currentColor")
                if stroke not in ('', 'none', 'currentColor') and not stroke.startswith('url('):
                    errors.append(f"hardcoded stroke='{stroke}' in {el.tag}; use currentColor")

            if errors:
                for e in errors:
                    print(f"FAIL {icon}: {e}")
                sys.exit(1)
            print(f"OK: $icon")
            PYEOF
          done

          echo "All icons passed lint."
```

### 8.2 Enforcement rules

| Rule | Failure mode |
|------|-------------|
| Valid XML | Malformed SVG is rejected |
| `viewBox="0 0 24 24"` | Non-24×24 icons break layout grids |
| `role="img"` | Accessibility violation |
| `aria-label` present | Screen-reader users cannot identify icons |
| `focusable="false"` | Icons capture keyboard focus unexpectedly |
| `currentColor` for colour | No dark-mode support |
| No hardcoded hex/RGB fills | Icons ignore CSS colour tokens |

---

## 9. Annotation Requirements

Every icon SVG must carry all three accessibility annotations:

```xml
<svg ... role="img" aria-label="Settings" focusable="false">
  <title>Settings</title>
  <desc>A gear wheel with six teeth used to represent system settings or preferences</desc>
  <!-- paths -->
</svg>
```

| Element | Purpose |
|---------|---------|
| `role="img"` | Identifies the element as an image landmark for assistive tech |
| `aria-label` | Short label shown by screen readers in place of the icon |
| `<title>` | SVG document title, also read by AT |
| `<desc>` | Long description for complex icons |

> **Note:** `aria-label` and `<title>` must be identical strings. Both are read
> in different AT modes; keeping them in sync prevents contradictory
> announcements.

---

## References

- Fluent UI Icons: https://fluentui.com/icons
- Material Symbols: https://fonts.google.com/icons
- SVG accessibility: https://www.w3.org/WAI/tutorials/images/informative/#svg-images
- Liquid Glass iOS 25: Apple Human Interface Guidelines (glass effects)
