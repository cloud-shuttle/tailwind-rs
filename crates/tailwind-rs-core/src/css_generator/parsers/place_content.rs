//! Place Content Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS place-content utilities,
//! such as `place-content-center`, `place-content-start`, `place-content-end`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PlaceContentParser {
    place_content_map: HashMap<String, String>,
}

impl PlaceContentParser {
    pub fn new() -> Self {
        let mut place_content_map = HashMap::new();
        place_content_map.insert("place-content-center".to_string(), "center".to_string());
        place_content_map.insert(
            "place-content-center-safe".to_string(),
            "safe center".to_string(),
        );
        place_content_map.insert("place-content-start".to_string(), "start".to_string());
        place_content_map.insert("place-content-end".to_string(), "end".to_string());
        place_content_map.insert("place-content-end-safe".to_string(), "safe end".to_string());
        place_content_map.insert(
            "place-content-between".to_string(),
            "space-between".to_string(),
        );
        place_content_map.insert(
            "place-content-around".to_string(),
            "space-around".to_string(),
        );
        place_content_map.insert(
            "place-content-evenly".to_string(),
            "space-evenly".to_string(),
        );
        place_content_map.insert("place-content-baseline".to_string(), "baseline".to_string());
        place_content_map.insert("place-content-stretch".to_string(), "stretch".to_string());

        Self { place_content_map }
    }

    fn parse_place_content_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(place_content_value) = self.place_content_map.get(class) {
            return Some(vec![CssProperty {
                name: "place-content".to_string(),
                value: place_content_value.clone(),
                important: false,
            }]);
        }
        None
    }
}

impl UtilityParser for PlaceContentParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_place_content_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "place-content-center",
            "place-content-center-safe",
            "place-content-start",
            "place-content-end",
            "place-content-end-safe",
            "place-content-between",
            "place-content-around",
            "place-content-evenly",
            "place-content-baseline",
            "place-content-stretch",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Grid
    }
}

impl Default for PlaceContentParser {
    fn default() -> Self {
        Self::new()
    }
}
