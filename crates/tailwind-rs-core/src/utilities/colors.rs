//! Color utilities for tailwind-rs
//!
//! This module provides utilities for text, background, border, ring, accent, and caret colors.
//! It includes all Tailwind CSS color palettes and shades.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Color palette values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorPalette {
    // Grays
    Gray, Slate, Zinc, Neutral, Stone,
    // Reds
    Red, Rose, Pink,
    // Oranges
    Orange, Amber, Yellow,
    // Greens
    Lime, Green, Emerald, Teal, Cyan,
    // Blues
    Sky, Blue, Indigo, Violet,
    // Purples
    Purple, Fuchsia,
}

/// Color shade values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorShade {
    Shade50, Shade100, Shade200, Shade300, Shade400,
    Shade500, Shade600, Shade700, Shade800, Shade900, Shade950,
}

/// Color value combining palette and shade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    pub palette: ColorPalette,
    pub shade: ColorShade,
}

impl Color {
    pub fn new(palette: ColorPalette, shade: ColorShade) -> Self {
        Self { palette, shade }
    }
    
    pub fn to_class_name(&self) -> String {
        format!("{}-{}", self.palette, self.shade)
    }
    
    pub fn to_css_value(&self) -> String {
        match (self.palette, self.shade) {
            // Gray palette
            (ColorPalette::Gray, ColorShade::Shade50) => "#f9fafb".to_string(),
            (ColorPalette::Gray, ColorShade::Shade100) => "#f3f4f6".to_string(),
            (ColorPalette::Gray, ColorShade::Shade200) => "#e5e7eb".to_string(),
            (ColorPalette::Gray, ColorShade::Shade300) => "#d1d5db".to_string(),
            (ColorPalette::Gray, ColorShade::Shade400) => "#9ca3af".to_string(),
            (ColorPalette::Gray, ColorShade::Shade500) => "#6b7280".to_string(),
            (ColorPalette::Gray, ColorShade::Shade600) => "#4b5563".to_string(),
            (ColorPalette::Gray, ColorShade::Shade700) => "#374151".to_string(),
            (ColorPalette::Gray, ColorShade::Shade800) => "#1f2937".to_string(),
            (ColorPalette::Gray, ColorShade::Shade900) => "#111827".to_string(),
            (ColorPalette::Gray, ColorShade::Shade950) => "#030712".to_string(),
            
            // Slate palette
            (ColorPalette::Slate, ColorShade::Shade50) => "#f8fafc".to_string(),
            (ColorPalette::Slate, ColorShade::Shade100) => "#f1f5f9".to_string(),
            (ColorPalette::Slate, ColorShade::Shade200) => "#e2e8f0".to_string(),
            (ColorPalette::Slate, ColorShade::Shade300) => "#cbd5e1".to_string(),
            (ColorPalette::Slate, ColorShade::Shade400) => "#94a3b8".to_string(),
            (ColorPalette::Slate, ColorShade::Shade500) => "#64748b".to_string(),
            (ColorPalette::Slate, ColorShade::Shade600) => "#475569".to_string(),
            (ColorPalette::Slate, ColorShade::Shade700) => "#334155".to_string(),
            (ColorPalette::Slate, ColorShade::Shade800) => "#1e293b".to_string(),
            (ColorPalette::Slate, ColorShade::Shade900) => "#0f172a".to_string(),
            (ColorPalette::Slate, ColorShade::Shade950) => "#020617".to_string(),
            
            // Blue palette
            (ColorPalette::Blue, ColorShade::Shade50) => "#eff6ff".to_string(),
            (ColorPalette::Blue, ColorShade::Shade100) => "#dbeafe".to_string(),
            (ColorPalette::Blue, ColorShade::Shade200) => "#bfdbfe".to_string(),
            (ColorPalette::Blue, ColorShade::Shade300) => "#93c5fd".to_string(),
            (ColorPalette::Blue, ColorShade::Shade400) => "#60a5fa".to_string(),
            (ColorPalette::Blue, ColorShade::Shade500) => "#3b82f6".to_string(),
            (ColorPalette::Blue, ColorShade::Shade600) => "#2563eb".to_string(),
            (ColorPalette::Blue, ColorShade::Shade700) => "#1d4ed8".to_string(),
            (ColorPalette::Blue, ColorShade::Shade800) => "#1e40af".to_string(),
            (ColorPalette::Blue, ColorShade::Shade900) => "#1e3a8a".to_string(),
            (ColorPalette::Blue, ColorShade::Shade950) => "#172554".to_string(),
            
            // Red palette
            (ColorPalette::Red, ColorShade::Shade50) => "#fef2f2".to_string(),
            (ColorPalette::Red, ColorShade::Shade100) => "#fee2e2".to_string(),
            (ColorPalette::Red, ColorShade::Shade200) => "#fecaca".to_string(),
            (ColorPalette::Red, ColorShade::Shade300) => "#fca5a5".to_string(),
            (ColorPalette::Red, ColorShade::Shade400) => "#f87171".to_string(),
            (ColorPalette::Red, ColorShade::Shade500) => "#ef4444".to_string(),
            (ColorPalette::Red, ColorShade::Shade600) => "#dc2626".to_string(),
            (ColorPalette::Red, ColorShade::Shade700) => "#b91c1c".to_string(),
            (ColorPalette::Red, ColorShade::Shade800) => "#991b1b".to_string(),
            (ColorPalette::Red, ColorShade::Shade900) => "#7f1d1d".to_string(),
            (ColorPalette::Red, ColorShade::Shade950) => "#450a0a".to_string(),
            
            // Green palette
            (ColorPalette::Green, ColorShade::Shade50) => "#f0fdf4".to_string(),
            (ColorPalette::Green, ColorShade::Shade100) => "#dcfce7".to_string(),
            (ColorPalette::Green, ColorShade::Shade200) => "#bbf7d0".to_string(),
            (ColorPalette::Green, ColorShade::Shade300) => "#86efac".to_string(),
            (ColorPalette::Green, ColorShade::Shade400) => "#4ade80".to_string(),
            (ColorPalette::Green, ColorShade::Shade500) => "#22c55e".to_string(),
            (ColorPalette::Green, ColorShade::Shade600) => "#16a34a".to_string(),
            (ColorPalette::Green, ColorShade::Shade700) => "#15803d".to_string(),
            (ColorPalette::Green, ColorShade::Shade800) => "#166534".to_string(),
            (ColorPalette::Green, ColorShade::Shade900) => "#14532d".to_string(),
            (ColorPalette::Green, ColorShade::Shade950) => "#052e16".to_string(),
            
            // Zinc palette
            (ColorPalette::Zinc, ColorShade::Shade50) => "#fafafa".to_string(),
            (ColorPalette::Zinc, ColorShade::Shade100) => "#f4f4f5".to_string(),
            (ColorPalette::Zinc, ColorShade::Shade200) => "#e4e4e7".to_string(),
            (ColorPalette::Zinc, ColorShade::Shade300) => "#d4d4d8".to_string(),
            (ColorPalette::Zinc, ColorShade::Shade400) => "#a1a1aa".to_string(),
            (ColorPalette::Zinc, ColorShade::Shade500) => "#71717a".to_string(),
            (ColorPalette::Zinc, ColorShade::Shade600) => "#52525b".to_string(),
            (ColorPalette::Zinc, ColorShade::Shade700) => "#3f3f46".to_string(),
            (ColorPalette::Zinc, ColorShade::Shade800) => "#27272a".to_string(),
            (ColorPalette::Zinc, ColorShade::Shade900) => "#18181b".to_string(),
            (ColorPalette::Zinc, ColorShade::Shade950) => "#09090b".to_string(),
            
            // Neutral palette
            (ColorPalette::Neutral, ColorShade::Shade50) => "#fafafa".to_string(),
            (ColorPalette::Neutral, ColorShade::Shade100) => "#f5f5f5".to_string(),
            (ColorPalette::Neutral, ColorShade::Shade200) => "#e5e5e5".to_string(),
            (ColorPalette::Neutral, ColorShade::Shade300) => "#d4d4d4".to_string(),
            (ColorPalette::Neutral, ColorShade::Shade400) => "#a3a3a3".to_string(),
            (ColorPalette::Neutral, ColorShade::Shade500) => "#737373".to_string(),
            (ColorPalette::Neutral, ColorShade::Shade600) => "#525252".to_string(),
            (ColorPalette::Neutral, ColorShade::Shade700) => "#404040".to_string(),
            (ColorPalette::Neutral, ColorShade::Shade800) => "#262626".to_string(),
            (ColorPalette::Neutral, ColorShade::Shade900) => "#171717".to_string(),
            (ColorPalette::Neutral, ColorShade::Shade950) => "#0a0a0a".to_string(),
            
            // Stone palette
            (ColorPalette::Stone, ColorShade::Shade50) => "#fafaf9".to_string(),
            (ColorPalette::Stone, ColorShade::Shade100) => "#f5f5f4".to_string(),
            (ColorPalette::Stone, ColorShade::Shade200) => "#e7e5e4".to_string(),
            (ColorPalette::Stone, ColorShade::Shade300) => "#d6d3d1".to_string(),
            (ColorPalette::Stone, ColorShade::Shade400) => "#a8a29e".to_string(),
            (ColorPalette::Stone, ColorShade::Shade500) => "#78716c".to_string(),
            (ColorPalette::Stone, ColorShade::Shade600) => "#57534e".to_string(),
            (ColorPalette::Stone, ColorShade::Shade700) => "#44403c".to_string(),
            (ColorPalette::Stone, ColorShade::Shade800) => "#292524".to_string(),
            (ColorPalette::Stone, ColorShade::Shade900) => "#1c1917".to_string(),
            (ColorPalette::Stone, ColorShade::Shade950) => "#0c0a09".to_string(),
            
            // Rose palette
            (ColorPalette::Rose, ColorShade::Shade50) => "#fff1f2".to_string(),
            (ColorPalette::Rose, ColorShade::Shade100) => "#ffe4e6".to_string(),
            (ColorPalette::Rose, ColorShade::Shade200) => "#fecdd3".to_string(),
            (ColorPalette::Rose, ColorShade::Shade300) => "#fda4af".to_string(),
            (ColorPalette::Rose, ColorShade::Shade400) => "#fb7185".to_string(),
            (ColorPalette::Rose, ColorShade::Shade500) => "#f43f5e".to_string(),
            (ColorPalette::Rose, ColorShade::Shade600) => "#e11d48".to_string(),
            (ColorPalette::Rose, ColorShade::Shade700) => "#be123c".to_string(),
            (ColorPalette::Rose, ColorShade::Shade800) => "#9f1239".to_string(),
            (ColorPalette::Rose, ColorShade::Shade900) => "#881337".to_string(),
            (ColorPalette::Rose, ColorShade::Shade950) => "#4c0519".to_string(),
            
            // Pink palette
            (ColorPalette::Pink, ColorShade::Shade50) => "#fdf2f8".to_string(),
            (ColorPalette::Pink, ColorShade::Shade100) => "#fce7f3".to_string(),
            (ColorPalette::Pink, ColorShade::Shade200) => "#fbcfe8".to_string(),
            (ColorPalette::Pink, ColorShade::Shade300) => "#f9a8d4".to_string(),
            (ColorPalette::Pink, ColorShade::Shade400) => "#f472b6".to_string(),
            (ColorPalette::Pink, ColorShade::Shade500) => "#ec4899".to_string(),
            (ColorPalette::Pink, ColorShade::Shade600) => "#db2777".to_string(),
            (ColorPalette::Pink, ColorShade::Shade700) => "#be185d".to_string(),
            (ColorPalette::Pink, ColorShade::Shade800) => "#9d174d".to_string(),
            (ColorPalette::Pink, ColorShade::Shade900) => "#831843".to_string(),
            (ColorPalette::Pink, ColorShade::Shade950) => "#500724".to_string(),
            
            // Orange palette
            (ColorPalette::Orange, ColorShade::Shade50) => "#fff7ed".to_string(),
            (ColorPalette::Orange, ColorShade::Shade100) => "#ffedd5".to_string(),
            (ColorPalette::Orange, ColorShade::Shade200) => "#fed7aa".to_string(),
            (ColorPalette::Orange, ColorShade::Shade300) => "#fdba74".to_string(),
            (ColorPalette::Orange, ColorShade::Shade400) => "#fb923c".to_string(),
            (ColorPalette::Orange, ColorShade::Shade500) => "#f97316".to_string(),
            (ColorPalette::Orange, ColorShade::Shade600) => "#ea580c".to_string(),
            (ColorPalette::Orange, ColorShade::Shade700) => "#c2410c".to_string(),
            (ColorPalette::Orange, ColorShade::Shade800) => "#9a3412".to_string(),
            (ColorPalette::Orange, ColorShade::Shade900) => "#7c2d12".to_string(),
            (ColorPalette::Orange, ColorShade::Shade950) => "#431407".to_string(),
            
            // Amber palette
            (ColorPalette::Amber, ColorShade::Shade50) => "#fffbeb".to_string(),
            (ColorPalette::Amber, ColorShade::Shade100) => "#fef3c7".to_string(),
            (ColorPalette::Amber, ColorShade::Shade200) => "#fde68a".to_string(),
            (ColorPalette::Amber, ColorShade::Shade300) => "#fcd34d".to_string(),
            (ColorPalette::Amber, ColorShade::Shade400) => "#fbbf24".to_string(),
            (ColorPalette::Amber, ColorShade::Shade500) => "#f59e0b".to_string(),
            (ColorPalette::Amber, ColorShade::Shade600) => "#d97706".to_string(),
            (ColorPalette::Amber, ColorShade::Shade700) => "#b45309".to_string(),
            (ColorPalette::Amber, ColorShade::Shade800) => "#92400e".to_string(),
            (ColorPalette::Amber, ColorShade::Shade900) => "#78350f".to_string(),
            (ColorPalette::Amber, ColorShade::Shade950) => "#451a03".to_string(),
            
            // Yellow palette
            (ColorPalette::Yellow, ColorShade::Shade50) => "#fefce8".to_string(),
            (ColorPalette::Yellow, ColorShade::Shade100) => "#fef9c3".to_string(),
            (ColorPalette::Yellow, ColorShade::Shade200) => "#fef08a".to_string(),
            (ColorPalette::Yellow, ColorShade::Shade300) => "#fde047".to_string(),
            (ColorPalette::Yellow, ColorShade::Shade400) => "#facc15".to_string(),
            (ColorPalette::Yellow, ColorShade::Shade500) => "#eab308".to_string(),
            (ColorPalette::Yellow, ColorShade::Shade600) => "#ca8a04".to_string(),
            (ColorPalette::Yellow, ColorShade::Shade700) => "#a16207".to_string(),
            (ColorPalette::Yellow, ColorShade::Shade800) => "#854d0e".to_string(),
            (ColorPalette::Yellow, ColorShade::Shade900) => "#713f12".to_string(),
            (ColorPalette::Yellow, ColorShade::Shade950) => "#422006".to_string(),
            
            // Lime palette
            (ColorPalette::Lime, ColorShade::Shade50) => "#f7fee7".to_string(),
            (ColorPalette::Lime, ColorShade::Shade100) => "#ecfccb".to_string(),
            (ColorPalette::Lime, ColorShade::Shade200) => "#d9f99d".to_string(),
            (ColorPalette::Lime, ColorShade::Shade300) => "#bef264".to_string(),
            (ColorPalette::Lime, ColorShade::Shade400) => "#a3e635".to_string(),
            (ColorPalette::Lime, ColorShade::Shade500) => "#84cc16".to_string(),
            (ColorPalette::Lime, ColorShade::Shade600) => "#65a30d".to_string(),
            (ColorPalette::Lime, ColorShade::Shade700) => "#4d7c0f".to_string(),
            (ColorPalette::Lime, ColorShade::Shade800) => "#3f6212".to_string(),
            (ColorPalette::Lime, ColorShade::Shade900) => "#365314".to_string(),
            (ColorPalette::Lime, ColorShade::Shade950) => "#1a2e05".to_string(),
            
            // Emerald palette
            (ColorPalette::Emerald, ColorShade::Shade50) => "#ecfdf5".to_string(),
            (ColorPalette::Emerald, ColorShade::Shade100) => "#d1fae5".to_string(),
            (ColorPalette::Emerald, ColorShade::Shade200) => "#a7f3d0".to_string(),
            (ColorPalette::Emerald, ColorShade::Shade300) => "#6ee7b7".to_string(),
            (ColorPalette::Emerald, ColorShade::Shade400) => "#34d399".to_string(),
            (ColorPalette::Emerald, ColorShade::Shade500) => "#10b981".to_string(),
            (ColorPalette::Emerald, ColorShade::Shade600) => "#059669".to_string(),
            (ColorPalette::Emerald, ColorShade::Shade700) => "#047857".to_string(),
            (ColorPalette::Emerald, ColorShade::Shade800) => "#065f46".to_string(),
            (ColorPalette::Emerald, ColorShade::Shade900) => "#064e3b".to_string(),
            (ColorPalette::Emerald, ColorShade::Shade950) => "#022c22".to_string(),
            
            // Teal palette
            (ColorPalette::Teal, ColorShade::Shade50) => "#f0fdfa".to_string(),
            (ColorPalette::Teal, ColorShade::Shade100) => "#ccfbf1".to_string(),
            (ColorPalette::Teal, ColorShade::Shade200) => "#99f6e4".to_string(),
            (ColorPalette::Teal, ColorShade::Shade300) => "#5eead4".to_string(),
            (ColorPalette::Teal, ColorShade::Shade400) => "#2dd4bf".to_string(),
            (ColorPalette::Teal, ColorShade::Shade500) => "#14b8a6".to_string(),
            (ColorPalette::Teal, ColorShade::Shade600) => "#0d9488".to_string(),
            (ColorPalette::Teal, ColorShade::Shade700) => "#0f766e".to_string(),
            (ColorPalette::Teal, ColorShade::Shade800) => "#115e59".to_string(),
            (ColorPalette::Teal, ColorShade::Shade900) => "#134e4a".to_string(),
            (ColorPalette::Teal, ColorShade::Shade950) => "#042f2e".to_string(),
            
            // Cyan palette
            (ColorPalette::Cyan, ColorShade::Shade50) => "#ecfeff".to_string(),
            (ColorPalette::Cyan, ColorShade::Shade100) => "#cffafe".to_string(),
            (ColorPalette::Cyan, ColorShade::Shade200) => "#a5f3fc".to_string(),
            (ColorPalette::Cyan, ColorShade::Shade300) => "#67e8f9".to_string(),
            (ColorPalette::Cyan, ColorShade::Shade400) => "#22d3ee".to_string(),
            (ColorPalette::Cyan, ColorShade::Shade500) => "#06b6d4".to_string(),
            (ColorPalette::Cyan, ColorShade::Shade600) => "#0891b2".to_string(),
            (ColorPalette::Cyan, ColorShade::Shade700) => "#0e7490".to_string(),
            (ColorPalette::Cyan, ColorShade::Shade800) => "#155e75".to_string(),
            (ColorPalette::Cyan, ColorShade::Shade900) => "#164e63".to_string(),
            (ColorPalette::Cyan, ColorShade::Shade950) => "#083344".to_string(),
            
            // Sky palette
            (ColorPalette::Sky, ColorShade::Shade50) => "#f0f9ff".to_string(),
            (ColorPalette::Sky, ColorShade::Shade100) => "#e0f2fe".to_string(),
            (ColorPalette::Sky, ColorShade::Shade200) => "#bae6fd".to_string(),
            (ColorPalette::Sky, ColorShade::Shade300) => "#7dd3fc".to_string(),
            (ColorPalette::Sky, ColorShade::Shade400) => "#38bdf8".to_string(),
            (ColorPalette::Sky, ColorShade::Shade500) => "#0ea5e9".to_string(),
            (ColorPalette::Sky, ColorShade::Shade600) => "#0284c7".to_string(),
            (ColorPalette::Sky, ColorShade::Shade700) => "#0369a1".to_string(),
            (ColorPalette::Sky, ColorShade::Shade800) => "#075985".to_string(),
            (ColorPalette::Sky, ColorShade::Shade900) => "#0c4a6e".to_string(),
            (ColorPalette::Sky, ColorShade::Shade950) => "#082f49".to_string(),
            
            // Indigo palette
            (ColorPalette::Indigo, ColorShade::Shade50) => "#eef2ff".to_string(),
            (ColorPalette::Indigo, ColorShade::Shade100) => "#e0e7ff".to_string(),
            (ColorPalette::Indigo, ColorShade::Shade200) => "#c7d2fe".to_string(),
            (ColorPalette::Indigo, ColorShade::Shade300) => "#a5b4fc".to_string(),
            (ColorPalette::Indigo, ColorShade::Shade400) => "#818cf8".to_string(),
            (ColorPalette::Indigo, ColorShade::Shade500) => "#6366f1".to_string(),
            (ColorPalette::Indigo, ColorShade::Shade600) => "#4f46e5".to_string(),
            (ColorPalette::Indigo, ColorShade::Shade700) => "#4338ca".to_string(),
            (ColorPalette::Indigo, ColorShade::Shade800) => "#3730a3".to_string(),
            (ColorPalette::Indigo, ColorShade::Shade900) => "#312e81".to_string(),
            (ColorPalette::Indigo, ColorShade::Shade950) => "#1e1b4b".to_string(),
            
            // Violet palette
            (ColorPalette::Violet, ColorShade::Shade50) => "#f5f3ff".to_string(),
            (ColorPalette::Violet, ColorShade::Shade100) => "#ede9fe".to_string(),
            (ColorPalette::Violet, ColorShade::Shade200) => "#ddd6fe".to_string(),
            (ColorPalette::Violet, ColorShade::Shade300) => "#c4b5fd".to_string(),
            (ColorPalette::Violet, ColorShade::Shade400) => "#a78bfa".to_string(),
            (ColorPalette::Violet, ColorShade::Shade500) => "#8b5cf6".to_string(),
            (ColorPalette::Violet, ColorShade::Shade600) => "#7c3aed".to_string(),
            (ColorPalette::Violet, ColorShade::Shade700) => "#6d28d9".to_string(),
            (ColorPalette::Violet, ColorShade::Shade800) => "#5b21b6".to_string(),
            (ColorPalette::Violet, ColorShade::Shade900) => "#4c1d95".to_string(),
            (ColorPalette::Violet, ColorShade::Shade950) => "#2e1065".to_string(),
            
            // Purple palette
            (ColorPalette::Purple, ColorShade::Shade50) => "#faf5ff".to_string(),
            (ColorPalette::Purple, ColorShade::Shade100) => "#f3e8ff".to_string(),
            (ColorPalette::Purple, ColorShade::Shade200) => "#e9d5ff".to_string(),
            (ColorPalette::Purple, ColorShade::Shade300) => "#d8b4fe".to_string(),
            (ColorPalette::Purple, ColorShade::Shade400) => "#c084fc".to_string(),
            (ColorPalette::Purple, ColorShade::Shade500) => "#a855f7".to_string(),
            (ColorPalette::Purple, ColorShade::Shade600) => "#9333ea".to_string(),
            (ColorPalette::Purple, ColorShade::Shade700) => "#7e22ce".to_string(),
            (ColorPalette::Purple, ColorShade::Shade800) => "#6b21a8".to_string(),
            (ColorPalette::Purple, ColorShade::Shade900) => "#581c87".to_string(),
            (ColorPalette::Purple, ColorShade::Shade950) => "#3b0764".to_string(),
            
            // Fuchsia palette
            (ColorPalette::Fuchsia, ColorShade::Shade50) => "#fdf4ff".to_string(),
            (ColorPalette::Fuchsia, ColorShade::Shade100) => "#fae8ff".to_string(),
            (ColorPalette::Fuchsia, ColorShade::Shade200) => "#f5d0fe".to_string(),
            (ColorPalette::Fuchsia, ColorShade::Shade300) => "#f0abfc".to_string(),
            (ColorPalette::Fuchsia, ColorShade::Shade400) => "#e879f9".to_string(),
            (ColorPalette::Fuchsia, ColorShade::Shade500) => "#d946ef".to_string(),
            (ColorPalette::Fuchsia, ColorShade::Shade600) => "#c026d3".to_string(),
            (ColorPalette::Fuchsia, ColorShade::Shade700) => "#a21caf".to_string(),
            (ColorPalette::Fuchsia, ColorShade::Shade800) => "#86198f".to_string(),
            (ColorPalette::Fuchsia, ColorShade::Shade900) => "#701a75".to_string(),
            (ColorPalette::Fuchsia, ColorShade::Shade950) => "#4a044e".to_string(),
        }
    }
    
    pub fn all_colors() -> Vec<Color> {
        let mut colors = Vec::new();
        for palette in ColorPalette::all_palettes() {
            for shade in ColorShade::all_shades() {
                colors.push(Color::new(palette, shade));
            }
        }
        colors
    }
}

impl ColorPalette {
    pub fn all_palettes() -> Vec<ColorPalette> {
        vec![
            ColorPalette::Gray, ColorPalette::Slate, ColorPalette::Zinc, ColorPalette::Neutral, ColorPalette::Stone,
            ColorPalette::Red, ColorPalette::Rose, ColorPalette::Pink,
            ColorPalette::Orange, ColorPalette::Amber, ColorPalette::Yellow,
            ColorPalette::Lime, ColorPalette::Green, ColorPalette::Emerald, ColorPalette::Teal, ColorPalette::Cyan,
            ColorPalette::Sky, ColorPalette::Blue, ColorPalette::Indigo, ColorPalette::Violet,
            ColorPalette::Purple, ColorPalette::Fuchsia,
        ]
    }
}

impl ColorShade {
    pub fn all_shades() -> Vec<ColorShade> {
        vec![
            ColorShade::Shade50, ColorShade::Shade100, ColorShade::Shade200, ColorShade::Shade300, ColorShade::Shade400,
            ColorShade::Shade500, ColorShade::Shade600, ColorShade::Shade700, ColorShade::Shade800, ColorShade::Shade900, ColorShade::Shade950,
        ]
    }
}

impl std::fmt::Display for ColorPalette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ColorPalette::Gray => "gray",
            ColorPalette::Slate => "slate",
            ColorPalette::Zinc => "zinc",
            ColorPalette::Neutral => "neutral",
            ColorPalette::Stone => "stone",
            ColorPalette::Red => "red",
            ColorPalette::Rose => "rose",
            ColorPalette::Pink => "pink",
            ColorPalette::Orange => "orange",
            ColorPalette::Amber => "amber",
            ColorPalette::Yellow => "yellow",
            ColorPalette::Lime => "lime",
            ColorPalette::Green => "green",
            ColorPalette::Emerald => "emerald",
            ColorPalette::Teal => "teal",
            ColorPalette::Cyan => "cyan",
            ColorPalette::Sky => "sky",
            ColorPalette::Blue => "blue",
            ColorPalette::Indigo => "indigo",
            ColorPalette::Violet => "violet",
            ColorPalette::Purple => "purple",
            ColorPalette::Fuchsia => "fuchsia",
        };
        write!(f, "{}", name)
    }
}

impl std::fmt::Display for ColorShade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shade = match self {
            ColorShade::Shade50 => "50",
            ColorShade::Shade100 => "100",
            ColorShade::Shade200 => "200",
            ColorShade::Shade300 => "300",
            ColorShade::Shade400 => "400",
            ColorShade::Shade500 => "500",
            ColorShade::Shade600 => "600",
            ColorShade::Shade700 => "700",
            ColorShade::Shade800 => "800",
            ColorShade::Shade900 => "900",
            ColorShade::Shade950 => "950",
        };
        write!(f, "{}", shade)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding text color utilities to a class builder
pub trait TextColorUtilities {
    fn text_color(self, color: Color) -> Self;
    fn text_transparent(self) -> Self;
    fn text_current(self) -> Self;
    fn text_black(self) -> Self;
    fn text_white(self) -> Self;
}

impl TextColorUtilities for ClassBuilder {
    fn text_color(self, color: Color) -> Self {
        self.class(format!("text-{}", color.to_class_name()))
    }
    
    fn text_transparent(self) -> Self {
        self.class("text-transparent")
    }
    
    fn text_current(self) -> Self {
        self.class("text-current")
    }
    
    fn text_black(self) -> Self {
        self.class("text-black")
    }
    
    fn text_white(self) -> Self {
        self.class("text-white")
    }
}

/// Trait for adding background color utilities to a class builder
pub trait BackgroundColorUtilities {
    fn background_color(self, color: Color) -> Self;
    fn background_transparent(self) -> Self;
    fn background_current(self) -> Self;
    fn background_black(self) -> Self;
    fn background_white(self) -> Self;
}

impl BackgroundColorUtilities for ClassBuilder {
    fn background_color(self, color: Color) -> Self {
        self.class(format!("bg-{}", color.to_class_name()))
    }
    
    fn background_transparent(self) -> Self {
        self.class("bg-transparent")
    }
    
    fn background_current(self) -> Self {
        self.class("bg-current")
    }
    
    fn background_black(self) -> Self {
        self.class("bg-black")
    }
    
    fn background_white(self) -> Self {
        self.class("bg-white")
    }
}

/// Trait for adding border color utilities to a class builder
pub trait BorderColorUtilities {
    fn border_color(self, color: Color) -> Self;
    fn border_transparent(self) -> Self;
    fn border_current(self) -> Self;
    fn border_black(self) -> Self;
    fn border_white(self) -> Self;
}

impl BorderColorUtilities for ClassBuilder {
    fn border_color(self, color: Color) -> Self {
        self.class(format!("border-{}", color.to_class_name()))
    }
    
    fn border_transparent(self) -> Self {
        self.class("border-transparent")
    }
    
    fn border_current(self) -> Self {
        self.class("border-current")
    }
    
    fn border_black(self) -> Self {
        self.class("border-black")
    }
    
    fn border_white(self) -> Self {
        self.class("border-white")
    }
}

/// Trait for adding ring color utilities to a class builder
pub trait RingColorUtilities {
    fn ring_color(self, color: Color) -> Self;
    fn ring_transparent(self) -> Self;
    fn ring_current(self) -> Self;
    fn ring_black(self) -> Self;
    fn ring_white(self) -> Self;
}

impl RingColorUtilities for ClassBuilder {
    fn ring_color(self, color: Color) -> Self {
        self.class(format!("ring-{}", color.to_class_name()))
    }
    
    fn ring_transparent(self) -> Self {
        self.class("ring-transparent")
    }
    
    fn ring_current(self) -> Self {
        self.class("ring-current")
    }
    
    fn ring_black(self) -> Self {
        self.class("ring-black")
    }
    
    fn ring_white(self) -> Self {
        self.class("ring-white")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_color_utilities() {
        let classes = ClassBuilder::new()
            .text_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .text_color(Color::new(ColorPalette::Red, ColorShade::Shade600))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("text-blue-500"));
        assert!(css_classes.contains("text-red-600"));
    }
    
    #[test]
    fn test_background_color_utilities() {
        let classes = ClassBuilder::new()
            .background_color(Color::new(ColorPalette::Gray, ColorShade::Shade100))
            .background_color(Color::new(ColorPalette::Green, ColorShade::Shade500))
            .background_color(Color::new(ColorPalette::Slate, ColorShade::Shade800))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-gray-100"));
        assert!(css_classes.contains("bg-green-500"));
        assert!(css_classes.contains("bg-slate-800"));
    }
    
    #[test]
    fn test_border_color_utilities() {
        let classes = ClassBuilder::new()
            .border_color(Color::new(ColorPalette::Gray, ColorShade::Shade300))
            .border_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("border-gray-300"));
        assert!(css_classes.contains("border-blue-500"));
    }
    
    #[test]
    fn test_ring_color_utilities() {
        let classes = ClassBuilder::new()
            .ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .ring_color(Color::new(ColorPalette::Red, ColorShade::Shade600))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("ring-blue-500"));
        assert!(css_classes.contains("ring-red-600"));
    }
    
    #[test]
    fn test_special_color_values() {
        let classes = ClassBuilder::new()
            .text_transparent()
            .background_transparent()
            .border_transparent()
            .text_current()
            .background_current()
            .border_current()
            .text_black()
            .background_white()
            .border_black()
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("text-transparent"));
        assert!(css_classes.contains("bg-transparent"));
        assert!(css_classes.contains("border-transparent"));
        assert!(css_classes.contains("text-current"));
        assert!(css_classes.contains("bg-current"));
        assert!(css_classes.contains("border-current"));
        assert!(css_classes.contains("text-black"));
        assert!(css_classes.contains("bg-white"));
        assert!(css_classes.contains("border-black"));
    }
    
    #[test]
    fn test_slate_color_css_values() {
        // Test that slate colors have proper CSS hex values
        let slate_800 = Color::new(ColorPalette::Slate, ColorShade::Shade800);
        assert_eq!(slate_800.to_css_value(), "#1e293b");
        
        let slate_50 = Color::new(ColorPalette::Slate, ColorShade::Shade50);
        assert_eq!(slate_50.to_css_value(), "#f8fafc");
        
        let slate_500 = Color::new(ColorPalette::Slate, ColorShade::Shade500);
        assert_eq!(slate_500.to_css_value(), "#64748b");
        
        // Test that class names are generated correctly
        assert_eq!(slate_800.to_class_name(), "slate-800");
        assert_eq!(slate_50.to_class_name(), "slate-50");
        assert_eq!(slate_500.to_class_name(), "slate-500");
    }
    
    #[test]
    fn test_missing_color_palettes_css_values() {
        // Test that missing color palettes have proper CSS hex values
        // Zinc palette
        let zinc_500 = Color::new(ColorPalette::Zinc, ColorShade::Shade500);
        assert_eq!(zinc_500.to_css_value(), "#71717a");
        
        // Neutral palette
        let neutral_500 = Color::new(ColorPalette::Neutral, ColorShade::Shade500);
        assert_eq!(neutral_500.to_css_value(), "#737373");
        
        // Stone palette
        let stone_500 = Color::new(ColorPalette::Stone, ColorShade::Shade500);
        assert_eq!(stone_500.to_css_value(), "#78716c");
        
        // Rose palette
        let rose_500 = Color::new(ColorPalette::Rose, ColorShade::Shade500);
        assert_eq!(rose_500.to_css_value(), "#f43f5e");
        
        // Pink palette
        let pink_500 = Color::new(ColorPalette::Pink, ColorShade::Shade500);
        assert_eq!(pink_500.to_css_value(), "#ec4899");
        
        // Orange palette
        let orange_500 = Color::new(ColorPalette::Orange, ColorShade::Shade500);
        assert_eq!(orange_500.to_css_value(), "#f97316");
        
        // Amber palette
        let amber_500 = Color::new(ColorPalette::Amber, ColorShade::Shade500);
        assert_eq!(amber_500.to_css_value(), "#f59e0b");
        
        // Yellow palette
        let yellow_500 = Color::new(ColorPalette::Yellow, ColorShade::Shade500);
        assert_eq!(yellow_500.to_css_value(), "#eab308");
        
        // Lime palette
        let lime_500 = Color::new(ColorPalette::Lime, ColorShade::Shade500);
        assert_eq!(lime_500.to_css_value(), "#84cc16");
        
        // Emerald palette
        let emerald_500 = Color::new(ColorPalette::Emerald, ColorShade::Shade500);
        assert_eq!(emerald_500.to_css_value(), "#10b981");
        
        // Teal palette
        let teal_500 = Color::new(ColorPalette::Teal, ColorShade::Shade500);
        assert_eq!(teal_500.to_css_value(), "#14b8a6");
        
        // Cyan palette
        let cyan_500 = Color::new(ColorPalette::Cyan, ColorShade::Shade500);
        assert_eq!(cyan_500.to_css_value(), "#06b6d4");
        
        // Sky palette
        let sky_500 = Color::new(ColorPalette::Sky, ColorShade::Shade500);
        assert_eq!(sky_500.to_css_value(), "#0ea5e9");
        
        // Indigo palette
        let indigo_500 = Color::new(ColorPalette::Indigo, ColorShade::Shade500);
        assert_eq!(indigo_500.to_css_value(), "#6366f1");
        
        // Violet palette
        let violet_500 = Color::new(ColorPalette::Violet, ColorShade::Shade500);
        assert_eq!(violet_500.to_css_value(), "#8b5cf6");
        
        // Purple palette
        let purple_500 = Color::new(ColorPalette::Purple, ColorShade::Shade500);
        assert_eq!(purple_500.to_css_value(), "#a855f7");
        
        // Fuchsia palette
        let fuchsia_500 = Color::new(ColorPalette::Fuchsia, ColorShade::Shade500);
        assert_eq!(fuchsia_500.to_css_value(), "#d946ef");
    }

    #[test]
    fn test_all_color_palettes_display() {
        // Test that all color palettes display correctly
        assert_eq!(format!("{}", ColorPalette::Gray), "gray");
        assert_eq!(format!("{}", ColorPalette::Slate), "slate");
        assert_eq!(format!("{}", ColorPalette::Zinc), "zinc");
        assert_eq!(format!("{}", ColorPalette::Neutral), "neutral");
        assert_eq!(format!("{}", ColorPalette::Stone), "stone");
        assert_eq!(format!("{}", ColorPalette::Red), "red");
        assert_eq!(format!("{}", ColorPalette::Rose), "rose");
        assert_eq!(format!("{}", ColorPalette::Pink), "pink");
        assert_eq!(format!("{}", ColorPalette::Orange), "orange");
        assert_eq!(format!("{}", ColorPalette::Amber), "amber");
        assert_eq!(format!("{}", ColorPalette::Yellow), "yellow");
        assert_eq!(format!("{}", ColorPalette::Lime), "lime");
        assert_eq!(format!("{}", ColorPalette::Green), "green");
        assert_eq!(format!("{}", ColorPalette::Emerald), "emerald");
        assert_eq!(format!("{}", ColorPalette::Teal), "teal");
        assert_eq!(format!("{}", ColorPalette::Cyan), "cyan");
        assert_eq!(format!("{}", ColorPalette::Sky), "sky");
        assert_eq!(format!("{}", ColorPalette::Blue), "blue");
        assert_eq!(format!("{}", ColorPalette::Indigo), "indigo");
        assert_eq!(format!("{}", ColorPalette::Violet), "violet");
        assert_eq!(format!("{}", ColorPalette::Purple), "purple");
        assert_eq!(format!("{}", ColorPalette::Fuchsia), "fuchsia");
    }

    #[test]
    fn test_all_color_shades_display() {
        // Test that all color shades display correctly
        assert_eq!(format!("{}", ColorShade::Shade50), "50");
        assert_eq!(format!("{}", ColorShade::Shade100), "100");
        assert_eq!(format!("{}", ColorShade::Shade200), "200");
        assert_eq!(format!("{}", ColorShade::Shade300), "300");
        assert_eq!(format!("{}", ColorShade::Shade400), "400");
        assert_eq!(format!("{}", ColorShade::Shade500), "500");
        assert_eq!(format!("{}", ColorShade::Shade600), "600");
        assert_eq!(format!("{}", ColorShade::Shade700), "700");
        assert_eq!(format!("{}", ColorShade::Shade800), "800");
        assert_eq!(format!("{}", ColorShade::Shade900), "900");
        assert_eq!(format!("{}", ColorShade::Shade950), "950");
    }

    #[test]
    fn test_color_display() {
        // Test that Color displays correctly
        let color = Color::new(ColorPalette::Blue, ColorShade::Shade500);
        assert_eq!(format!("{}", color), "blue-500");
        
        let color2 = Color::new(ColorPalette::Red, ColorShade::Shade800);
        assert_eq!(format!("{}", color2), "red-800");
    }

    #[test]
    fn test_color_palette_all_palettes() {
        // Test that all_palettes returns all available palettes
        let palettes = ColorPalette::all_palettes();
        assert_eq!(palettes.len(), 22);
        
        // Test that all expected palettes are present
        assert!(palettes.contains(&ColorPalette::Gray));
        assert!(palettes.contains(&ColorPalette::Slate));
        assert!(palettes.contains(&ColorPalette::Zinc));
        assert!(palettes.contains(&ColorPalette::Neutral));
        assert!(palettes.contains(&ColorPalette::Stone));
        assert!(palettes.contains(&ColorPalette::Red));
        assert!(palettes.contains(&ColorPalette::Rose));
        assert!(palettes.contains(&ColorPalette::Pink));
        assert!(palettes.contains(&ColorPalette::Orange));
        assert!(palettes.contains(&ColorPalette::Amber));
        assert!(palettes.contains(&ColorPalette::Yellow));
        assert!(palettes.contains(&ColorPalette::Lime));
        assert!(palettes.contains(&ColorPalette::Green));
        assert!(palettes.contains(&ColorPalette::Emerald));
        assert!(palettes.contains(&ColorPalette::Teal));
        assert!(palettes.contains(&ColorPalette::Cyan));
        assert!(palettes.contains(&ColorPalette::Sky));
        assert!(palettes.contains(&ColorPalette::Blue));
        assert!(palettes.contains(&ColorPalette::Indigo));
        assert!(palettes.contains(&ColorPalette::Violet));
        assert!(palettes.contains(&ColorPalette::Purple));
        assert!(palettes.contains(&ColorPalette::Fuchsia));
    }

    #[test]
    fn test_color_shade_all_shades() {
        // Test that all_shades returns all available shades
        let shades = ColorShade::all_shades();
        assert_eq!(shades.len(), 11);
        
        // Test that all expected shades are present
        assert!(shades.contains(&ColorShade::Shade50));
        assert!(shades.contains(&ColorShade::Shade100));
        assert!(shades.contains(&ColorShade::Shade200));
        assert!(shades.contains(&ColorShade::Shade300));
        assert!(shades.contains(&ColorShade::Shade400));
        assert!(shades.contains(&ColorShade::Shade500));
        assert!(shades.contains(&ColorShade::Shade600));
        assert!(shades.contains(&ColorShade::Shade700));
        assert!(shades.contains(&ColorShade::Shade800));
        assert!(shades.contains(&ColorShade::Shade900));
        assert!(shades.contains(&ColorShade::Shade950));
    }

    #[test]
    fn test_color_all_colors() {
        // Test that all_colors returns all combinations
        let colors = Color::all_colors();
        assert_eq!(colors.len(), 242); // 22 palettes * 11 shades = 242
        
        // Test that some expected colors are present
        assert!(colors.contains(&Color::new(ColorPalette::Blue, ColorShade::Shade500)));
        assert!(colors.contains(&Color::new(ColorPalette::Red, ColorShade::Shade600)));
        assert!(colors.contains(&Color::new(ColorPalette::Green, ColorShade::Shade700)));
    }

    #[test]
    fn test_comprehensive_color_utilities() {
        // Test comprehensive usage of all color utility methods
        let classes = ClassBuilder::new()
            // Text colors
            .text_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .text_color(Color::new(ColorPalette::Red, ColorShade::Shade600))
            .text_transparent()
            .text_current()
            .text_black()
            .text_white()
            
            // Background colors
            .background_color(Color::new(ColorPalette::Gray, ColorShade::Shade100))
            .background_color(Color::new(ColorPalette::Green, ColorShade::Shade500))
            .background_transparent()
            .background_current()
            .background_black()
            .background_white()
            
            // Border colors
            .border_color(Color::new(ColorPalette::Gray, ColorShade::Shade300))
            .border_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .border_transparent()
            .border_current()
            .border_black()
            .border_white()
            
            // Ring colors
            .ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .ring_color(Color::new(ColorPalette::Red, ColorShade::Shade600))
            .ring_transparent()
            .ring_current()
            .ring_black()
            .ring_white()
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Verify text colors
        assert!(css_classes.contains("text-blue-500"));
        assert!(css_classes.contains("text-red-600"));
        assert!(css_classes.contains("text-transparent"));
        assert!(css_classes.contains("text-current"));
        assert!(css_classes.contains("text-black"));
        assert!(css_classes.contains("text-white"));
        
        // Verify background colors
        assert!(css_classes.contains("bg-gray-100"));
        assert!(css_classes.contains("bg-green-500"));
        assert!(css_classes.contains("bg-transparent"));
        assert!(css_classes.contains("bg-current"));
        assert!(css_classes.contains("bg-black"));
        assert!(css_classes.contains("bg-white"));
        
        // Verify border colors
        assert!(css_classes.contains("border-gray-300"));
        assert!(css_classes.contains("border-blue-500"));
        assert!(css_classes.contains("border-transparent"));
        assert!(css_classes.contains("border-current"));
        assert!(css_classes.contains("border-black"));
        assert!(css_classes.contains("border-white"));
        
        // Verify ring colors
        assert!(css_classes.contains("ring-blue-500"));
        assert!(css_classes.contains("ring-red-600"));
        assert!(css_classes.contains("ring-transparent"));
        assert!(css_classes.contains("ring-current"));
        assert!(css_classes.contains("ring-black"));
        assert!(css_classes.contains("ring-white"));
    }

    #[test]
    fn test_color_serialization() {
        // Test that colors can be serialized and deserialized
        let color = Color::new(ColorPalette::Blue, ColorShade::Shade500);
        let serialized = serde_json::to_string(&color).unwrap();
        let deserialized: Color = serde_json::from_str(&serialized).unwrap();
        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_color_palette_serialization() {
        // Test that color palettes can be serialized and deserialized
        let palette = ColorPalette::Blue;
        let serialized = serde_json::to_string(&palette).unwrap();
        let deserialized: ColorPalette = serde_json::from_str(&serialized).unwrap();
        assert_eq!(palette, deserialized);
    }

    #[test]
    fn test_color_shade_serialization() {
        // Test that color shades can be serialized and deserialized
        let shade = ColorShade::Shade500;
        let serialized = serde_json::to_string(&shade).unwrap();
        let deserialized: ColorShade = serde_json::from_str(&serialized).unwrap();
        assert_eq!(shade, deserialized);
    }

    #[test]
    fn test_color_equality_and_hash() {
        // Test that colors can be compared for equality and hashed
        let color1 = Color::new(ColorPalette::Blue, ColorShade::Shade500);
        let color2 = Color::new(ColorPalette::Blue, ColorShade::Shade500);
        let color3 = Color::new(ColorPalette::Red, ColorShade::Shade500);
        
        assert_eq!(color1, color2);
        assert_ne!(color1, color3);
        
        // Test that equal colors have the same hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        color1.hash(&mut hasher1);
        color2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
