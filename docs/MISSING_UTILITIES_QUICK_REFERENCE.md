# 🚨 Missing Utilities Quick Reference

## Overview

Quick reference for the ~29 missing Tailwind CSS utilities that need parser implementation.

---

## 📝 Typography (14 missing)

| Utility | Classes | CSS Property | Priority |
|---------|---------|--------------|----------|
| **Font Smoothing** | `.font-smooth-*` | `font-smooth` | Medium |
| **Font Stretch** | `.font-stretch-*` | `font-stretch` | Medium |
| **Font Variant Numeric** | `.font-variant-numeric-*` | `font-variant-numeric` | Low |
| **Line Clamp** | `.line-clamp-*` | `-webkit-line-clamp` | **High** |
| **List Style Image** | `.list-image-*` | `list-style-image` | Low |
| **List Style Position** | `.list-position-*` | `list-style-position` | Low |
| **List Style Type** | `.list-*` | `list-style-type` | Medium |
| **Text Overflow** | `.text-ellipsis`, `.text-clip` | `text-overflow` | **High** |
| **Text Wrap** | `.text-wrap-*` | `text-wrap` | Medium |
| **Text Indent** | `.indent-*` | `text-indent` | Medium |
| **Word Break** | `.break-*` | `word-break` | **High** |
| **Overflow Wrap** | `.break-words`, `.break-normal` | `overflow-wrap` | **High** |
| **Hyphens** | `.hyphens-*` | `hyphens` | Low |
| **Content** | `.content-*` | `content` | Medium |

---

## 🎨 Backgrounds (3 missing)

| Utility | Classes | CSS Property | Priority |
|---------|---------|--------------|----------|
| **Background Attachment** | `.bg-attachment-*` | `background-attachment` | Medium |
| **Background Clip** | `.bg-clip-*` | `background-clip` | **High** |
| **Background Origin** | `.bg-origin-*` | `background-origin` | Medium |

---

## 🔲 Borders (4 missing)

| Utility | Classes | CSS Property | Priority |
|---------|---------|--------------|----------|
| **Outline Width** | `.outline-*` | `outline-width` | **High** |
| **Outline Color** | `.outline-*-*` | `outline-color` | **High** |
| **Outline Style** | `.outline-*` | `outline-style` | Medium |
| **Outline Offset** | `.outline-offset-*` | `outline-offset` | Medium |

---

## ✨ Effects (2 missing)

| Utility | Classes | CSS Property | Priority |
|---------|---------|--------------|----------|
| **Mix Blend Mode** | `.mix-blend-*` | `mix-blend-mode` | Low |
| **Background Blend Mode** | `.bg-blend-*` | `background-blend-mode` | Low |

---

## 🖱️ Interactivity (5 missing)

| Utility | Classes | CSS Property | Priority |
|---------|---------|--------------|----------|
| **Accent Color** | `.accent-*` | `accent-color` | Medium |
| **Field Sizing** | `.field-*` | `field-sizing` | Low |
| **Scroll Snap Align** | `.snap-*` | `scroll-snap-align` | Medium |
| **Scroll Snap Stop** | `.snap-*` | `scroll-snap-stop` | Low |
| **Scroll Snap Type** | `.snap-*` | `scroll-snap-type` | Medium |

---

## 📦 Break Control (1 missing)

| Utility | Classes | CSS Property | Priority |
|---------|---------|--------------|----------|
| **Box Decoration Break** | `.decoration-*` | `box-decoration-break` | Low |

---

## 🎯 Implementation Priority

### **🔴 High Priority (8 utilities)**
- `line-clamp-*` - Text truncation
- `text-ellipsis`, `text-clip` - Text overflow
- `break-*` - Word breaking
- `break-words`, `break-normal` - Overflow wrap
- `outline-*` - Focus states
- `bg-clip-*` - Background clipping

### **🟡 Medium Priority (15 utilities)**
- Typography utilities (font-smooth, font-stretch, etc.)
- Background utilities (bg-attachment, bg-origin)
- Border utilities (outline-style, outline-offset)
- Interactivity utilities (accent-color, scroll-snap)

### **🟢 Low Priority (6 utilities)**
- Advanced blend modes
- Field sizing
- Box decoration break
- Font variant numeric
- List utilities
- Hyphens

---

## 🛠️ Parser Implementation Order

1. **`AdvancedTypographyParser`** (14 utilities) - High impact
2. **`OutlineParser`** (4 utilities) - Accessibility
3. **`AdvancedBackgroundParser`** (3 utilities) - Visual effects
4. **`AdvancedInteractivityParser`** (5 utilities) - Modern features
5. **`AdvancedBlendModeParser`** (2 utilities) - Advanced effects
6. **`BoxDecorationBreakParser`** (1 utility) - Edge case

---

## 📊 Coverage Progress

- **Current**: 74 parsers (~90% coverage)
- **After Phase 1**: 76 parsers (~95% coverage)
- **After Phase 2**: 80 parsers (~98% coverage)
- **After Phase 3**: 83 parsers (~100% coverage)

**Total missing**: 6 parsers, ~29 utilities

---

*Quick reference for development planning*
