---
title: Banner Builder YAML Configuration Reference
timestamp: 2026-11-06T06:49:16
description: Complete reference of all YAML configuration fields for banbu and Banner Builder, designed for both human readers and LLM YAML generation.
published: true
---

This document describes the complete YAML configuration format accepted by `banbu` (`banner-builder`). It serves both as a human-readable manual and as a structured specification for Large Language Models (LLMs) generating Banner Builder YAML files.

---

## Complete Annotated YAML Schema

```yaml
---
# REQUIRED top-level fields
width: 1000                   # u32: Width of the output PNG image in pixels (> 0)
height: 500                   # u32: Height of the output PNG image in pixels (> 0)
text: "Hello World!"          # string: Centered main text (use `text:` for empty text; max 90 bytes)

# OPTIONAL top-level fields
size: 24                      # i32 (default: 24): Font size in pixels for `text`
background_color: "FFFFFF"    # string (default: "FFFFFF"): 6- or 8-digit hex RGB/RGBA color (no '#' prefix)

# OPTIONAL gradient background (overrides `background_color` when `background.gradient` is present)
background:
  gradient:
    type: linear              # string (default: "linear"): Only "linear" is supported
    angle: 90                 # f32 (default: 90.0): Angle in degrees
    stops:                    # list (required if `gradient` is set; minimum 2 stops)
      - color: "1E3C72"       # Stop object with explicit percentage position ("0%".."100%")
        position: "0%"
      - "2A5298"              # Or plain 6-/8-digit hex color string (position auto-interpolated)
      - color: "FDBB2D"
        position: "100%"

# OPTIONAL list of images to embed (rendered before text, in list order)
embed:
  - file: "logo.png"          # string (required): Path relative to the directory containing the YAML file
    x: 100                    # u32 (required): Left X coordinate in pixels (0 <= x + img_width <= width)
    y: 20                     # u32 (required): Top Y coordinate in pixels (0 <= y + img_height <= height)
    width: 200                # u32 (optional): Target width in pixels; height scales proportionally
    radius: 30                # u32 | string (optional, alias: `border_radius`): e.g. 30, "30px", "25%", "50%", "round", "circle"
    transparent_color: "FFFFFF" # string (optional): 6- or 8-digit hex color in the image to make transparent

# OPTIONAL list of positioned text lines (rendered after centered `text`, in list order)
lines:
  - text: "First line"        # string (required): Text content for this line
    x: 50                     # u32 (required): Left X coordinate (or right X anchor when `rtl: true`)
    y: 400                    # u32 (required): Top Y coordinate in pixels
    size: 40                  # i32 (default: 24): Font size in pixels
    color: "000000FF"         # string (default: "000000FF"): 6- or 8-digit hex RGB/RGBA color (no '#')
    rtl: false                # bool (default: false): Right-to-left text rendering
```

---

## Color Format Specification

All color fields (`background_color`, `lines[].color`, `background.gradient.stops`, `embed[].transparent_color`) use uppercase or lowercase hexadecimal strings **without** a leading `#`:

- **6-character RGB hex**: `RRGGBB` (e.g., `FFFFFF` for opaque white, `FF0000` for opaque red). Alpha defaults to `FF` (`255`).
- **8-character RGBA hex**: `RRGGBBAA` (e.g., `FF000080` for 50% transparent red, `000000FF` for opaque black).

> **Note for LLMs**: Always quote hex color strings that consist only of digits (e.g., `"000000"` or `"123456"`) or ensure they are valid YAML strings so YAML parsers do not interpret them as integers or octal numbers. Never include a leading `#` (which starts a YAML comment if unquoted or fails hex validation if quoted).

---

## Top-Level Fields (`Banner`)

| Field | Type | Required | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `width` | `u32` | **Yes** | — | Width of the generated banner in pixels (e.g., `1000`). |
| `height` | `u32` | **Yes** | — | Height of the generated banner in pixels (e.g., `500`). |
| `text` | `string` | **Yes** | — | Main text centered both horizontally and vertically on the banner. Wraps automatically at 30 characters per line. Always rendered in black (`000000FF`). **Constraint**: Must still be present even when unused (write `text: ""` or `text:`), and its length must not exceed `90` bytes. |
| `size` | `i32` | No | `24` | Font size (in pixels) for the centered `text`. |
| `background_color` | `string` | No | `"FFFFFF"` | Solid background color in 6- or 8-character hex format. Ignored if `background.gradient` is provided. |
| `background` | `Background` | No | `null` | Structured background configuration (see [Background & Gradients](#background--gradients)). |
| `embed` | ` list<Embed>` | No | `[]` | List of images to composite onto the background (see [Embedded Images (`embed`)](#embedded-images-embed)). |
| `lines` | `list<Text>` | No | `[]` | List of individually positioned text lines (see [Text Lines (`lines`)](#text-lines-lines)). |

---

## Background & Gradients

When `background.gradient` is defined, it overrides `background_color`.

### `background.gradient` Fields

| Field | Type | Required | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `type` | `string` | No | `"linear"` | Gradient type. Currently only `"linear"` is supported. |
| `angle` | `f32` | No | `90.0` | Linear gradient angle in degrees (`0` = left-to-right, `90` = top-to-bottom). |
| `stops` | `list<GradientStopInput>` | **Yes** | — | At least **2** color stops. |

### `stops` Item Formats

Each item in `background.gradient.stops` can be either:
1. **A color string**: e.g., `"FF5733"`
2. **An object with `color` and optional `position`**:
   - `color` (`string`, required): 6- or 8-digit hex color.
   - `position` (`string`, optional): Percentage string between `"0%"` and `"100%"` (e.g., `"25%"`).
   - Stop positions must be non-decreasing from first to last. Any stops without an explicit `position` are evenly interpolated between their neighboring positioned stops (with the first stop defaulting to `"0%"` and the last to `"100%"`).

---

## Embedded Images (`embed`)

Each entry in `embed` overlays an external image (PNG, JPG, etc.) onto the banner. Images are drawn **after** the background and **before** `text` and `lines`.

| Field | Type | Required | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `file` | `string` | **Yes** | — | Path to the image file, resolved relative to the directory containing the YAML file. |
| `x` | `u32` | **Yes** | — | Left X coordinate (in pixels) where the top-left corner of the image is placed. |
| `y` | `u32` | **Yes** | — | Top Y coordinate (in pixels) where the top-left corner of the image is placed. |
| `width` | `u32` | No | Original width | Resizes the image to this width in pixels while preserving aspect ratio (`height = width * orig_height / orig_width`). |
| `radius` | `u32 \| string` | No | `null` | Corner radius or circular mask (also accepts alias `border_radius`). Supports anti-aliased edges. |
| `transparent_color` | `string` | No | `null` | 6- or 8-digit hex color in the source image to treat as transparent (with automatic edge un-matting for smooth blending). |

### Supported `radius` (or `border_radius`) Values
- **Integer or pixel string** (e.g., `30` or `"30px"`): Rounds the four corners of the image with a `30` pixel radius (clamped to `min(width, height) / 2`).
- **Percentage string `< 50%`** (e.g., `"25%"`): Sets the corner radius to that percentage of `min(width, height)`.
- **`"50%"`, `"round"`, or `"circle"`**: Masks the image into a centered circle with diameter `min(width, height)`.

### Placement Constraints
- The embedded image (after optional `width` resizing) **must fit completely within the banner**:
  - `x + embedded_width <= banner.width`
  - `y + embedded_height <= banner.height`

---

## Text Lines (`lines`)

Use `lines` for multi-line layouts, custom coordinates, custom colors, or right-to-left (RTL) text.

| Field | Type | Required | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `text` | `string` | **Yes** | — | The text string to draw on this line (does not auto-wrap). |
| `x` | `u32` | **Yes** | — | Horizontal coordinate in pixels. Left edge when `rtl: false`; **right edge** when `rtl: true`. |
| `y` | `u32` | **Yes** | — | Vertical top coordinate in pixels. |
| `size` | `i32` | No | `24` | Font size in pixels for this line. |
| `color` | `string` | No | `"000000FF"` | Text color in 6- or 8-digit hex format (no `#`). |
| `rtl` | `bool` | No | `false` | When `true`, reverses the character order for right-to-left scripts (such as Hebrew or Arabic) and right-aligns the line at `x` (requires `x >= rendered_text_width`). |

---

## Checklist for LLMs Generating Banner Builder YAML

1. **Always include `width`, `height`, and `text`** at the top level. If only `lines` or `embed` are desired, set `text: ""` (or `text:`).
2. **Never prefix hex colors with `#`**, and quote hex strings that contain only digits (`"000000"`, `"112233"`).
3. **Keep `text` under 90 bytes**; for longer or multi-line positioned text, set `text: ""` and use the `lines` list instead.
4. **Ensure embedded images fit inside the canvas**: verify `x + width <= banner.width` and `y + resized_height <= banner.height`.
5. **For RTL text (`rtl: true`)**, set `x` near the right side of the banner (e.g., `x: 950` on a `1000`-wide banner) because `x` is the right edge of the text.
