//! Place Self Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS place-self utilities,
//! such as `place-self-auto`, `place-self-start`, `place-self-end`, etc.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PlaceSelfParser {
    place_self_map: HashMap<String, String>,
}

impl PlaceSelfParser {
    pub fn new() -> Self {
        let mut place_self_map = HashMap::new();
        place_self_map.insert("place-self-auto".to_string(), "auto".to_string());
        place_self_map.insert("place-self-start".to_string(), "start".to_string());
        place_self_map.insert("place-self-end".to_string(), "end".to_string());
        place_self_map.insert("place-self-end-safe".to_string(), "safe end".to_string());
        place_self_map.insert("place-self-center".to_string(), "center".to_string());
        place_self_map.insert(
            "place-self-center-safe".to_string(),
            "safe center".to_string(),
        );
        place_self_map.insert("place-self-stretch".to_string(), "stretch".to_string());

        Self { place_self_map }
    }

    fn parse_place_self_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(place_self_value) = self.place_self_map.get(class) {
            return Some(vec![CssProperty {
                name: "place-self".to_string(),
                value: place_self_value.clone(),
                important: false,
            }]);
        }
        None
    }
}

impl UtilityParser for PlaceSelfParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_place_self_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "place-self-auto",
            "place-self-start",
            "place-self-end",
            "place-self-end-safe",
            "place-self-center",
            "place-self-center-safe",
            "place-self-stretch",
        ]
    }

    fn get_priority(&self) -> u32 {
        70
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Grid
    }
}

impl Default for PlaceSelfParser {
    fn default() -> Self {
        Self::new()
    }
}
