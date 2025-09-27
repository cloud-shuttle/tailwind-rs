//! Grid Parser Methods for CssGenerator
//!
//! This module contains grid-related parsing methods for grid template columns, grid column, grid template rows, grid row, grid auto flow, grid auto columns, grid auto rows, gap, justify content, justify items, justify self, align content, align items, align self, place content, place items, and place self.

// Removed unused imports
use super::types::CssProperty;
use super::parsers::{
    GridTemplateColumnsParser, GridColumnParser, GridTemplateRowsParser, GridRowParser,
    GridAutoFlowParser, GridAutoColumnsParser, GridAutoRowsParser, GapParser, 
    JustifyContentParser, JustifyItemsParser, JustifySelfParser, AlignContentParser,
    AlignItemsParser, AlignSelfParser, PlaceContentParser, PlaceItemsParser, 
    PlaceSelfParser, UtilityParser
};

/// Grid parser methods for CssGenerator
pub trait GridParsers {
    /// Parse grid template columns classes
    fn parse_grid_template_columns_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse grid column classes
    fn parse_grid_column_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse grid template rows classes
    fn parse_grid_template_rows_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse grid row classes
    fn parse_grid_row_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse grid auto flow classes
    fn parse_grid_auto_flow_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse grid auto columns classes
    fn parse_grid_auto_columns_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse grid auto rows classes
    fn parse_grid_auto_rows_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse gap classes
    fn parse_gap_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse justify content classes
    fn parse_justify_content_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse justify items classes
    fn parse_justify_items_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse justify self classes
    fn parse_justify_self_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse align content classes
    fn parse_align_content_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse align items classes
    fn parse_align_items_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse align self classes
    fn parse_align_self_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse place content classes
    fn parse_place_content_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse place items classes
    fn parse_place_items_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    
    /// Parse place self classes
    fn parse_place_self_class(&self, class: &str) -> Option<Vec<CssProperty>>;
}

impl GridParsers for super::CssGenerator {
    fn parse_grid_template_columns_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridTemplateColumnsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_column_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridColumnParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_template_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridTemplateRowsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_row_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridRowParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_auto_flow_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridAutoFlowParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_auto_columns_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridAutoColumnsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_grid_auto_rows_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GridAutoRowsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_gap_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = GapParser::new();
        parser.parse_class(class)
    }
    
    fn parse_justify_content_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = JustifyContentParser::new();
        parser.parse_class(class)
    }
    
    fn parse_justify_items_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = JustifyItemsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_justify_self_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = JustifySelfParser::new();
        parser.parse_class(class)
    }
    
    fn parse_align_content_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = AlignContentParser::new();
        parser.parse_class(class)
    }
    
    fn parse_align_items_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = AlignItemsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_align_self_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = AlignSelfParser::new();
        parser.parse_class(class)
    }
    
    fn parse_place_content_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = PlaceContentParser::new();
        parser.parse_class(class)
    }
    
    fn parse_place_items_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = PlaceItemsParser::new();
        parser.parse_class(class)
    }
    
    fn parse_place_self_class(&self, class: &str) -> Option<Vec<CssProperty>> { 
        use super::parsers::UtilityParser;
        let parser = PlaceSelfParser::new();
        parser.parse_class(class)
    }
}
