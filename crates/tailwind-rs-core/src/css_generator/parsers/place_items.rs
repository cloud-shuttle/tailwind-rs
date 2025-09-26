//! Place Items Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS place-items utilities,
//! such as `place-items-start`, `place-items-end`, `place-items-center`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PlaceItemsParser {
    place_items_map: HashMap<String, String>,
}

impl PlaceItemsParser {
    pub fn new() -> Self {
        let mut place_items_map = HashMap::new();
        place_items_map.insert("place-items-start".to_string(), "start".to_string());
        place_items_map.insert("place-items-end".to_string(), "end".to_string());
        place_items_map.insert("place-items-end-safe".to_string(), "safe end".to_string());
        place_items_map.insert("place-items-center".to_string(), "center".to_string());
        place_items_map.insert(
            "place-items-center-safe".to_string(),
            "safe center".to_string(),
        );
        place_items_map.insert("place-items-baseline".to_string(), "baseline".to_string());
        place_items_map.insert("place-items-stretch".to_string(), "stretch".to_string());

        Self { place_items_map }
    }

    fn parse_place_items_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(place_items_value) = self.place_items_map.get(class) {
            return Some(vec![CssProperty {
                name: "place-items".to_string(),
                value: place_items_value.clone(),
                important: false,
            }]);
        }
        None
    }
}

impl UtilityParser for PlaceItemsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_place_items_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "place-items-start",
            "place-items-end",
            "place-items-end-safe",
            "place-items-center",
            "place-items-center-safe",
            "place-items-baseline",
            "place-items-stretch",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Grid
    }
}

impl Default for PlaceItemsParser {
    fn default() -> Self {
        Self::new()
    }
}
