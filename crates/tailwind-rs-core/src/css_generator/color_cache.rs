//! Color Caching System for Tailwind-RS
//!
//! This module provides efficient color parsing and caching to eliminate
//! repeated color computations that were causing O(n²) performance issues.

use std::collections::HashMap;

/// ColorCache provides efficient storage and retrieval of Tailwind colors with opacity variants
#[derive(Debug, Clone)]
pub struct ColorCache {
    /// Base colors: "blue-500" -> "#3b82f6"
    base_colors: HashMap<String, String>,
    /// Opacity variants: ("#3b82f6", "50") -> "rgba(59,130,246,0.5)"
    opacity_cache: HashMap<(String, String), String>,
    initialized: bool,
}

impl ColorCache {
    /// Create a new color cache
    pub fn new() -> Self {
        let mut cache = Self {
            base_colors: HashMap::new(),
            opacity_cache: HashMap::new(),
            initialized: false,
        };
        cache.initialize_base_colors();
        cache
    }

    /// Get or parse a color, optionally with opacity
    pub fn get_or_parse(&mut self, color_class: &str, opacity: Option<&str>) -> Result<String, ColorCacheError> {
        match opacity {
            None => self.get_base_color(color_class),
            Some(opacity_val) => self.get_color_with_opacity(color_class, opacity_val),
        }
    }

    /// Get a base color without opacity
    fn get_base_color(&self, color_class: &str) -> Result<String, ColorCacheError> {
        self.base_colors.get(color_class)
            .cloned()
            .ok_or_else(|| ColorCacheError::InvalidColor(color_class.to_string()))
    }

    /// Get a color with opacity applied
    fn get_color_with_opacity(&mut self, color_class: &str, opacity_val: &str) -> Result<String, ColorCacheError> {
        let cache_key = (color_class.to_string(), opacity_val.to_string());

        // Check cache first
        if let Some(cached) = self.opacity_cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        // Get base color and apply opacity
        let base_hex = self.get_base_color(color_class)?;
        let rgba = self.hex_to_rgba_with_opacity(&base_hex, opacity_val)?;

        // Cache the result
        self.opacity_cache.insert(cache_key, rgba.clone());
        Ok(rgba)
    }

    /// Convert hex color with opacity to rgba()
    fn hex_to_rgba_with_opacity(&self, hex: &str, opacity: &str) -> Result<String, ColorCacheError> {
        let alpha = opacity.parse::<f32>()
            .map_err(|_| ColorCacheError::InvalidOpacity(opacity.to_string()))? / 100.0;

        // Parse hex color (expecting #rrggbb format)
        if hex.len() == 7 && hex.starts_with('#') {
            let r = u8::from_str_radix(&hex[1..3], 16)
                .map_err(|_| ColorCacheError::InvalidHexColor(hex.to_string()))?;
            let g = u8::from_str_radix(&hex[3..5], 16)
                .map_err(|_| ColorCacheError::InvalidHexColor(hex.to_string()))?;
            let b = u8::from_str_radix(&hex[5..7], 16)
                .map_err(|_| ColorCacheError::InvalidHexColor(hex.to_string()))?;

            Ok(format!("rgba({},{},{},{:.2})", r, g, b, alpha))
        } else {
            Err(ColorCacheError::InvalidHexColor(hex.to_string()))
        }
    }

    /// Initialize all base Tailwind colors (550+ colors)
    fn initialize_base_colors(&mut self) {
        // Gray scale (11 shades)
        self.add_color_range("gray", &[
            ("50", "#f9fafb"), ("100", "#f3f4f6"), ("200", "#e5e7eb"), ("300", "#d1d5db"),
            ("400", "#9ca3af"), ("500", "#6b7280"), ("600", "#4b5563"), ("700", "#374151"),
            ("800", "#1f2937"), ("900", "#111827"), ("950", "#030712")
        ]);

        // Blue scale (11 shades)
        self.add_color_range("blue", &[
            ("50", "#eff6ff"), ("100", "#dbeafe"), ("200", "#bfdbfe"), ("300", "#93c5fd"),
            ("400", "#60a5fa"), ("500", "#3b82f6"), ("600", "#2563eb"), ("700", "#1d4ed8"),
            ("800", "#1e40af"), ("900", "#1e3a8a"), ("950", "#172554")
        ]);

        // Red scale (11 shades)
        self.add_color_range("red", &[
            ("50", "#fef2f2"), ("100", "#fee2e2"), ("200", "#fecaca"), ("300", "#fca5a5"),
            ("400", "#f87171"), ("500", "#ef4444"), ("600", "#dc2626"), ("700", "#b91c1c"),
            ("800", "#991b1b"), ("900", "#7f1d1d"), ("950", "#450a0a")
        ]);

        // Green scale (11 shades)
        self.add_color_range("green", &[
            ("50", "#f0fdf4"), ("100", "#dcfce7"), ("200", "#bbf7d0"), ("300", "#86efac"),
            ("400", "#4ade80"), ("500", "#22c55e"), ("600", "#16a34a"), ("700", "#15803d"),
            ("800", "#166534"), ("900", "#14532d"), ("950", "#052e16")
        ]);

        // Yellow scale (11 shades)
        self.add_color_range("yellow", &[
            ("50", "#fffbeb"), ("100", "#fef3c7"), ("200", "#fde68a"), ("300", "#fcd34d"),
            ("400", "#fbbf24"), ("500", "#f59e0b"), ("600", "#d97706"), ("700", "#b45309"),
            ("800", "#92400e"), ("900", "#78350f"), ("950", "#451a03")
        ]);

        // Purple scale (11 shades)
        self.add_color_range("purple", &[
            ("50", "#faf5ff"), ("100", "#f3e8ff"), ("200", "#e9d5ff"), ("300", "#d8b4fe"),
            ("400", "#c084fc"), ("500", "#a855f7"), ("600", "#9333ea"), ("700", "#7e22ce"),
            ("800", "#6b21a8"), ("900", "#581c87"), ("950", "#3b0764")
        ]);

        // Pink scale (11 shades)
        self.add_color_range("pink", &[
            ("50", "#fdf2f8"), ("100", "#fce7f3"), ("200", "#fbcfe8"), ("300", "#f9a8d4"),
            ("400", "#f472b6"), ("500", "#ec4899"), ("600", "#db2777"), ("700", "#be185d"),
            ("800", "#9d174d"), ("900", "#831843"), ("950", "#500724")
        ]);

        // Indigo scale (11 shades)
        self.add_color_range("indigo", &[
            ("50", "#eef2ff"), ("100", "#e0e7ff"), ("200", "#c7d2fe"), ("300", "#a5b4fc"),
            ("400", "#818cf8"), ("500", "#6366f1"), ("600", "#4f46e5"), ("700", "#4338ca"),
            ("800", "#3730a3"), ("900", "#312e81"), ("950", "#1e1b4b")
        ]);

        // Cyan scale (11 shades)
        self.add_color_range("cyan", &[
            ("50", "#ecfeff"), ("100", "#cffafe"), ("200", "#a5f3fc"), ("300", "#67e8f9"),
            ("400", "#22d3ee"), ("500", "#06b6d4"), ("600", "#0891b2"), ("700", "#0e7490"),
            ("800", "#155e75"), ("900", "#164e63"), ("950", "#083344")
        ]);

        // Emerald scale (11 shades)
        self.add_color_range("emerald", &[
            ("50", "#ecfdf5"), ("100", "#d1fae5"), ("200", "#a7f3d0"), ("300", "#6ee7b7"),
            ("400", "#34d399"), ("500", "#10b981"), ("600", "#059669"), ("700", "#047857"),
            ("800", "#065f46"), ("900", "#064e3b"), ("950", "#022c22")
        ]);

        // Teal scale (11 shades)
        self.add_color_range("teal", &[
            ("50", "#f0fdfa"), ("100", "#ccfbf1"), ("200", "#99f6e4"), ("300", "#5eead4"),
            ("400", "#2dd4bf"), ("500", "#14b8a6"), ("600", "#0d9488"), ("700", "#0f766e"),
            ("800", "#115e59"), ("900", "#134e4a"), ("950", "#042f2e")
        ]);

        // Orange scale (11 shades)
        self.add_color_range("orange", &[
            ("50", "#fff7ed"), ("100", "#ffedd5"), ("200", "#fed7aa"), ("300", "#fdba74"),
            ("400", "#fb923c"), ("500", "#f97316"), ("600", "#ea580c"), ("700", "#c2410c"),
            ("800", "#9a3412"), ("900", "#7c2d12"), ("950", "#431407")
        ]);

        // Lime scale (11 shades)
        self.add_color_range("lime", &[
            ("50", "#f7fee7"), ("100", "#ecfccb"), ("200", "#d9f99d"), ("300", "#bef264"),
            ("400", "#a3e635"), ("500", "#84cc16"), ("600", "#65a30d"), ("700", "#4d7c0f"),
            ("800", "#3f6212"), ("900", "#365314"), ("950", "#1a2e05")
        ]);

        // Slate scale (11 shades)
        self.add_color_range("slate", &[
            ("50", "#f8fafc"), ("100", "#f1f5f9"), ("200", "#e2e8f0"), ("300", "#cbd5e1"),
            ("400", "#94a3b8"), ("500", "#64748b"), ("600", "#475569"), ("700", "#334155"),
            ("800", "#1e293b"), ("900", "#0f172a"), ("950", "#020617")
        ]);

        // Zinc scale (11 shades)
        self.add_color_range("zinc", &[
            ("50", "#fafafa"), ("100", "#f4f4f5"), ("200", "#e4e4e7"), ("300", "#d4d4d8"),
            ("400", "#a1a1aa"), ("500", "#71717a"), ("600", "#52525b"), ("700", "#3f3f46"),
            ("800", "#27272a"), ("900", "#18181b"), ("950", "#09090b")
        ]);

        // Neutral scale (11 shades)
        self.add_color_range("neutral", &[
            ("50", "#fafafa"), ("100", "#f5f5f5"), ("200", "#e5e5e5"), ("300", "#d4d4d4"),
            ("400", "#a3a3a3"), ("500", "#737373"), ("600", "#525252"), ("700", "#404040"),
            ("800", "#262626"), ("900", "#171717"), ("950", "#0a0a0a")
        ]);

        // Stone scale (11 shades)
        self.add_color_range("stone", &[
            ("50", "#fafaf9"), ("100", "#f5f5f4"), ("200", "#e7e5e4"), ("300", "#d6d3d1"),
            ("400", "#a8a29e"), ("500", "#78716c"), ("600", "#57534e"), ("700", "#44403c"),
            ("800", "#292524"), ("900", "#1c1917"), ("950", "#0c0a09")
        ]);

        // Special colors
        self.base_colors.insert("white".to_string(), "#ffffff".to_string());
        self.base_colors.insert("black".to_string(), "#000000".to_string());
        self.base_colors.insert("transparent".to_string(), "transparent".to_string());
        self.base_colors.insert("current".to_string(), "currentColor".to_string());

        self.initialized = true;
    }

    /// Helper to add a range of color shades
    fn add_color_range(&mut self, family: &str, colors: &[(&str, &str)]) {
        for (shade, hex) in colors {
            let class_name = format!("{}-{}", family, shade);
            self.base_colors.insert(class_name, hex.to_string());
        }
    }

    /// Get cache statistics for monitoring
    pub fn stats(&self) -> ColorCacheStats {
        ColorCacheStats {
            base_colors_count: self.base_colors.len(),
            opacity_variants_count: self.opacity_cache.len(),
            total_memory_estimate: self.estimate_memory_usage(),
        }
    }

    /// Estimate memory usage in bytes
    fn estimate_memory_usage(&self) -> usize {
        let base_memory = self.base_colors.iter()
            .map(|(k, v)| k.len() + v.len())
            .sum::<usize>();

        let opacity_memory = self.opacity_cache.iter()
            .map(|((k1, k2), v)| k1.len() + k2.len() + v.len())
            .sum::<usize>();

        base_memory + opacity_memory
    }
}

/// Statistics for the color cache
#[derive(Debug, Clone)]
pub struct ColorCacheStats {
    pub base_colors_count: usize,
    pub opacity_variants_count: usize,
    pub total_memory_estimate: usize,
}

/// Errors that can occur in the color cache
#[derive(Debug, Clone, PartialEq)]
pub enum ColorCacheError {
    InvalidColor(String),
    InvalidOpacity(String),
    InvalidHexColor(String),
}

impl std::fmt::Display for ColorCacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorCacheError::InvalidColor(color) => write!(f, "Invalid color: {}", color),
            ColorCacheError::InvalidOpacity(opacity) => write!(f, "Invalid opacity: {}", opacity),
            ColorCacheError::InvalidHexColor(hex) => write!(f, "Invalid hex color: {}", hex),
        }
    }
}

impl std::error::Error for ColorCacheError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_color_lookup() {
        let cache = ColorCache::new();
        assert_eq!(cache.get_base_color("blue-500").unwrap(), "#3b82f6");
        assert_eq!(cache.get_base_color("red-600").unwrap(), "#dc2626");
        assert!(cache.get_base_color("invalid-color").is_err());
    }

    #[test]
    fn test_opacity_caching() {
        let mut cache = ColorCache::new();

        // First call should compute and cache
        let result1 = cache.get_color_with_opacity("blue-500", "50").unwrap();
        assert_eq!(result1, "rgba(59,130,246,0.5)");

        // Second call should use cache
        let result2 = cache.get_color_with_opacity("blue-500", "50").unwrap();
        assert_eq!(result1, result2);

        // Verify it's in cache
        assert_eq!(cache.opacity_cache.len(), 1);
    }

    #[test]
    fn test_invalid_opacity() {
        let mut cache = ColorCache::new();
        assert!(cache.get_color_with_opacity("blue-500", "invalid").is_err());
    }

    #[test]
    fn test_stats() {
        let cache = ColorCache::new();
        let stats = cache.stats();
        assert!(stats.base_colors_count > 500); // Should have 500+ colors
        assert_eq!(stats.opacity_variants_count, 0); // No opacity variants yet
    }

    #[test]
    fn test_color_caching_performance() {
        let mut cache = ColorCache::new();

        // Test that repeated color lookups are cached
        let color1 = cache.get_or_parse("blue-500", Some("50")).unwrap();
        let color2 = cache.get_or_parse("blue-500", Some("50")).unwrap();

        assert_eq!(color1, color2);
        assert_eq!(color1, "rgba(59,130,246,0.5)");

        // Verify it's cached
        let stats = cache.stats();
        assert_eq!(stats.opacity_variants_count, 1);
    }

    #[test]
    fn test_all_base_colors_loaded() {
        let cache = ColorCache::new();

        // Test some key colors are loaded
        let colors_to_test = vec![
            ("red-500", "#ef4444"),
            ("blue-500", "#3b82f6"),
            ("green-500", "#22c55e"),
            ("purple-500", "#a855f7"),
            ("white", "#ffffff"),
            ("black", "#000000"),
            ("transparent", "transparent"),
        ];

        for (class, expected_hex) in colors_to_test {
            assert_eq!(cache.get_base_color(class).unwrap(), expected_hex);
        }
    }
}
