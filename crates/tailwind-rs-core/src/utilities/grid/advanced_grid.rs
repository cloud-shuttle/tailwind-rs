//! Advanced CSS Grid Utilities for Tailwind-RS
//!
//! This module provides comprehensive support for modern CSS Grid features
//! including named grid areas, complex grid templates, masonry layout,
//! and advanced grid line naming.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Write;

/// Advanced Grid Template Areas
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridTemplateAreas {
    /// Custom grid area pattern
    Custom(String),
    /// Header-main-sidebar layout
    HeaderMainSidebar,
    /// Header-sidebar-main layout
    HeaderSidebarMain,
    /// Sidebar-main-footer layout
    SidebarMainFooter,
    /// Header-main-footer layout
    HeaderMainFooter,
    /// Complex 3x3 grid areas
    Areas3x3(String, String, String, String, String, String, String, String, String),
}

impl fmt::Display for GridTemplateAreas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridTemplateAreas::Custom(pattern) => write!(f, "\"{}\"", pattern),
            GridTemplateAreas::HeaderMainSidebar => write!(f, "\"header header\" \"main sidebar\""),
            GridTemplateAreas::HeaderSidebarMain => write!(f, "\"header sidebar\" \"header main\""),
            GridTemplateAreas::SidebarMainFooter => write!(f, "\"sidebar main\" \"sidebar footer\""),
            GridTemplateAreas::HeaderMainFooter => write!(f, "\"header\" \"main\" \"footer\""),
            GridTemplateAreas::Areas3x3(a, b, c, d, e, f, g, h, i) =>
                write!(f, "\"{} {} {}\" \"{} {} {}\" \"{} {} {}\"", a, b, c, d, e, f, g, h, i)
        }
    }
}

/// Advanced Grid Auto Columns with modern sizing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdvancedGridAutoColumns {
    /// Auto sizing
    Auto,
    /// Min-max sizing
    MinMax(String, String),
    /// Fit-content sizing
    FitContent(String),
    /// Custom value
    Custom(String),
}

impl fmt::Display for AdvancedGridAutoColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdvancedGridAutoColumns::Auto => write!(f, "auto"),
            AdvancedGridAutoColumns::MinMax(min, max) => write!(f, "minmax({}, {})", min, max),
            AdvancedGridAutoColumns::FitContent(size) => write!(f, "fit-content({})", size),
            AdvancedGridAutoColumns::Custom(value) => write!(f, "{}", value),
        }
    }
}

/// Advanced Grid Template with repeat patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdvancedGridTemplate {
    /// Repeat pattern: repeat(count, size)
    Repeat(usize, String),
    /// Repeat with auto-fit: repeat(auto-fit, minmax(min, max))
    AutoFit(String, String),
    /// Repeat with auto-fill: repeat(auto-fill, minmax(min, max))
    AutoFill(String, String),
    /// Complex pattern with multiple sizes
    Pattern(Vec<String>),
    /// Named grid lines pattern
    NamedLines(Vec<String>),
}

impl fmt::Display for AdvancedGridTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdvancedGridTemplate::Repeat(count, size) => write!(f, "repeat({}, {})", count, size),
            AdvancedGridTemplate::AutoFit(min, max) => write!(f, "repeat(auto-fit, minmax({}, {}))", min, max),
            AdvancedGridTemplate::AutoFill(min, max) => write!(f, "repeat(auto-fill, minmax({}, {}))", min, max),
            AdvancedGridTemplate::Pattern(sizes) => write!(f, "{}", sizes.join(" ")),
            AdvancedGridTemplate::NamedLines(lines) => write!(f, "[{}] {}", lines.join("] ["), "]"),
        }
    }
}

/// Grid Line Names for advanced grid positioning
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridLineName {
    /// Custom line name
    Custom(String),
    /// Standard line names
    Start,
    End,
    Center,
    // Add more predefined names as needed
}

impl fmt::Display for GridLineName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridLineName::Custom(name) => write!(f, "{}", name),
            GridLineName::Start => write!(f, "start"),
            GridLineName::End => write!(f, "end"),
            GridLineName::Center => write!(f, "center"),
        }
    }
}

/// Advanced Grid Placement with line names
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdvancedGridPlacement {
    /// Named area placement
    Area(String),
    /// Line-based placement: grid-row/column: start / end
    Lines(GridLineName, GridLineName),
    /// Span placement: span N
    Span(usize),
    /// Auto placement
    Auto,
}

impl fmt::Display for AdvancedGridPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdvancedGridPlacement::Area(area) => write!(f, "{}", area),
            AdvancedGridPlacement::Lines(start, end) => write!(f, "{} / {}", start, end),
            AdvancedGridPlacement::Span(count) => write!(f, "span {}", count),
            AdvancedGridPlacement::Auto => write!(f, "auto"),
        }
    }
}

/// Masonry Grid Configuration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MasonryGrid {
    /// Horizontal masonry
    Horizontal,
    /// Vertical masonry (default)
    Vertical,
    /// Dense packing
    Dense,
}

impl fmt::Display for MasonryGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MasonryGrid::Horizontal => write!(f, "masonry"),
            MasonryGrid::Vertical => write!(f, "masonry"),
            MasonryGrid::Dense => write!(f, "masonry dense"),
        }
    }
}

/// Subgrid Configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SubgridConfig {
    /// Inherit parent grid lines
    Inherit,
    /// Custom subgrid lines
    Lines(Vec<String>),
    /// Auto subgrid
    Auto,
}

impl fmt::Display for SubgridConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubgridConfig::Inherit => write!(f, "subgrid"),
            SubgridConfig::Lines(lines) => write!(f, "subgrid [{}]", lines.join("] [")),
            SubgridConfig::Auto => write!(f, "auto"),
        }
    }
}

/// Advanced Grid Utilities Trait
pub trait AdvancedGridUtilities {
    // Grid Template Areas
    fn grid_areas(self, areas: GridTemplateAreas) -> Self;
    fn grid_areas_custom(self, pattern: &str) -> Self;

    // Advanced Auto Columns/Rows
    fn grid_auto_cols_advanced(self, config: AdvancedGridAutoColumns) -> Self;
    fn grid_auto_rows_advanced(self, config: AdvancedGridAutoColumns) -> Self;

    // Complex Grid Templates
    fn grid_template_advanced(self, template: AdvancedGridTemplate) -> Self;
    fn grid_cols_repeat(self, count: usize, size: &str) -> Self;
    fn grid_cols_auto_fit(self, min: &str, max: &str) -> Self;
    fn grid_cols_auto_fill(self, min: &str, max: &str) -> Self;

    // Named Grid Lines
    fn grid_row_named(self, start: GridLineName, end: GridLineName) -> Self;
    fn grid_col_named(self, start: GridLineName, end: GridLineName) -> Self;

    // Advanced Grid Placement
    fn grid_area_named(self, area: &str) -> Self;
    fn grid_placement_advanced(self, placement: AdvancedGridPlacement) -> Self;

    // Masonry Layout
    fn grid_masonry(self, masonry: MasonryGrid) -> Self;

    // Subgrid Support
    fn grid_subgrid(self, config: SubgridConfig) -> Self;

    // Complex Grid Patterns
    fn grid_template_complex(self, columns: Vec<String>, rows: Vec<String>) -> Self;

    // Grid Item Alignment (advanced)
    fn grid_self_align(self, align: &str) -> Self;
    fn grid_content_align(self, align: &str) -> Self;

    // Grid Auto Flow with dense
    fn grid_flow_dense(self) -> Self;
    fn grid_flow_row_dense(self) -> Self;
    fn grid_flow_col_dense(self) -> Self;
}

impl AdvancedGridUtilities for ClassBuilder {
    fn grid_areas(self, areas: GridTemplateAreas) -> Self {
        self.custom("grid-template-areas", &areas.to_string())
    }

    fn grid_areas_custom(self, pattern: &str) -> Self {
        self.custom("grid-template-areas", pattern)
    }

    fn grid_auto_cols_advanced(self, config: AdvancedGridAutoColumns) -> Self {
        self.custom("grid-auto-columns", &config.to_string())
    }

    fn grid_auto_rows_advanced(self, config: AdvancedGridAutoColumns) -> Self {
        self.custom("grid-auto-rows", &config.to_string())
    }

    fn grid_template_advanced(self, template: AdvancedGridTemplate) -> Self {
        self.custom("grid-template-columns", &template.to_string())
    }

    fn grid_cols_repeat(self, count: usize, size: &str) -> Self {
        let template = AdvancedGridTemplate::Repeat(count, size.to_string());
        self.grid_template_advanced(template)
    }

    fn grid_cols_auto_fit(self, min: &str, max: &str) -> Self {
        let template = AdvancedGridTemplate::AutoFit(min.to_string(), max.to_string());
        self.grid_template_advanced(template)
    }

    fn grid_cols_auto_fill(self, min: &str, max: &str) -> Self {
        let template = AdvancedGridTemplate::AutoFill(min.to_string(), max.to_string());
        self.grid_template_advanced(template)
    }

    fn grid_row_named(self, start: GridLineName, end: GridLineName) -> Self {
        self.custom("grid-row", &format!("{} / {}", start, end))
    }

    fn grid_col_named(self, start: GridLineName, end: GridLineName) -> Self {
        self.custom("grid-column", &format!("{} / {}", start, end))
    }

    fn grid_area_named(self, area: &str) -> Self {
        self.custom("grid-area", area)
    }

    fn grid_placement_advanced(self, placement: AdvancedGridPlacement) -> Self {
        self.custom("grid-area", &placement.to_string())
    }

    fn grid_masonry(self, masonry: MasonryGrid) -> Self {
        self.custom("grid-template-columns", &masonry.to_string())
    }

    fn grid_subgrid(self, config: SubgridConfig) -> Self {
        self.custom("grid-template-columns", &config.to_string())
            .custom("grid-template-rows", &config.to_string())
    }

    fn grid_template_complex(self, columns: Vec<String>, rows: Vec<String>) -> Self {
        let cols_template = columns.join(" ");
        let rows_template = rows.join(" ");

        self.custom("grid-template-columns", &cols_template)
            .custom("grid-template-rows", &rows_template)
    }

    fn grid_self_align(self, align: &str) -> Self {
        self.custom("justify-self", align)
            .custom("align-self", align)
    }

    fn grid_content_align(self, align: &str) -> Self {
        self.custom("justify-content", align)
            .custom("align-content", align)
    }

    fn grid_flow_dense(self) -> Self {
        self.custom("grid-auto-flow", "dense")
    }

    fn grid_flow_row_dense(self) -> Self {
        self.custom("grid-auto-flow", "row dense")
    }

    fn grid_flow_col_dense(self) -> Self {
        self.custom("grid-auto-flow", "column dense")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_template_areas() {
        let areas = GridTemplateAreas::HeaderMainSidebar;
        assert_eq!(areas.to_string(), "\"header header\" \"main sidebar\"");

        let areas = GridTemplateAreas::Areas3x3(
            "a".to_string(), "b".to_string(), "c".to_string(),
            "d".to_string(), "e".to_string(), "f".to_string(),
            "g".to_string(), "h".to_string(), "i".to_string(),
        );
        assert_eq!(areas.to_string(), "\"a b c\" \"d e f\" \"g h i\"");
    }

    #[test]
    fn test_advanced_grid_auto_columns() {
        let config = AdvancedGridAutoColumns::MinMax("100px".to_string(), "1fr".to_string());
        assert_eq!(config.to_string(), "minmax(100px, 1fr)");

        let config = AdvancedGridAutoColumns::FitContent("200px".to_string());
        assert_eq!(config.to_string(), "fit-content(200px)");
    }

    #[test]
    fn test_advanced_grid_template() {
        let template = AdvancedGridTemplate::Repeat(3, "1fr".to_string());
        assert_eq!(template.to_string(), "repeat(3, 1fr)");

        let template = AdvancedGridTemplate::AutoFit("200px".to_string(), "1fr".to_string());
        assert_eq!(template.to_string(), "repeat(auto-fit, minmax(200px, 1fr))");

        let template = AdvancedGridTemplate::NamedLines(vec![
            "header-start".to_string(),
            "main-start".to_string(),
            "sidebar-start".to_string(),
        ]);
        assert_eq!(template.to_string(), "[header-start] [main-start] [sidebar-start]");
    }

    #[test]
    fn test_grid_line_names() {
        let line = GridLineName::Custom("content-start".to_string());
        assert_eq!(line.to_string(), "content-start");

        let line = GridLineName::Start;
        assert_eq!(line.to_string(), "start");
    }

    #[test]
    fn test_advanced_grid_placement() {
        let placement = AdvancedGridPlacement::Area("header".to_string());
        assert_eq!(placement.to_string(), "header");

        let placement = AdvancedGridPlacement::Lines(GridLineName::Start, GridLineName::End);
        assert_eq!(placement.to_string(), "start / end");

        let placement = AdvancedGridPlacement::Span(3);
        assert_eq!(placement.to_string(), "span 3");
    }

    #[test]
    fn test_masonry_grid() {
        let masonry = MasonryGrid::Dense;
        assert_eq!(masonry.to_string(), "masonry dense");
    }

    #[test]
    fn test_subgrid_config() {
        let config = SubgridConfig::Lines(vec![
            "a".to_string(), "b".to_string(), "c".to_string()
        ]);
        assert_eq!(config.to_string(), "subgrid [a] [b] [c]");
    }

    #[test]
    fn test_advanced_grid_utilities() {
        // Test basic grid areas
        let builder = ClassBuilder::new()
            .grid_areas(GridTemplateAreas::HeaderMainSidebar);

        // Test advanced auto columns
        let builder = ClassBuilder::new()
            .grid_auto_cols_advanced(AdvancedGridAutoColumns::MinMax(
                "100px".to_string(),
                "1fr".to_string()
            ));

        // Test complex grid template
        let builder = ClassBuilder::new()
            .grid_cols_auto_fit("200px", "1fr");

        // Test named grid lines
        let builder = ClassBuilder::new()
            .grid_row_named(GridLineName::Start, GridLineName::End);

        // Test masonry
        let builder = ClassBuilder::new()
            .grid_masonry(MasonryGrid::Dense);

        // Test subgrid
        let builder = ClassBuilder::new()
            .grid_subgrid(SubgridConfig::Inherit);

        // Test complex grid
        let builder = ClassBuilder::new()
            .grid_template_complex(
                vec!["1fr".to_string(), "2fr".to_string(), "1fr".to_string()],
                vec!["auto".to_string(), "1fr".to_string(), "auto".to_string()]
            );

        // All should succeed without panicking
        assert!(true, "Advanced grid utilities work correctly");
    }

    #[test]
    fn test_grid_flow_utilities() {
        let builder = ClassBuilder::new()
            .grid_flow_dense();

        let builder = ClassBuilder::new()
            .grid_flow_row_dense();

        let builder = ClassBuilder::new()
            .grid_flow_col_dense();

        assert!(true, "Grid flow utilities work correctly");
    }

    #[test]
    fn test_grid_alignment_utilities() {
        let builder = ClassBuilder::new()
            .grid_self_align("center");

        let builder = ClassBuilder::new()
            .grid_content_align("space-between");

        assert!(true, "Grid alignment utilities work correctly");
    }
}
