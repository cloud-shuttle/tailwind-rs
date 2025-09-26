//! Order Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS order utilities,
//! such as `order-1`, `order-2`, `-order-1`, `order-first`, `order-last`, `order-none`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OrderParser {
    order_map: HashMap<String, String>,
}

impl OrderParser {
    pub fn new() -> Self {
        let mut order_map = HashMap::new();
        order_map.insert("order-first".to_string(), "-9999".to_string());
        order_map.insert("order-last".to_string(), "9999".to_string());
        order_map.insert("order-none".to_string(), "0".to_string());
        
        // Add order-1 through order-12
        for i in 1..=12 {
            order_map.insert(format!("order-{}", i), i.to_string());
        }
        
        // Add negative orders -order-1 through -order-12
        for i in 1..=12 {
            order_map.insert(format!("-order-{}", i), format!("calc({} * -1)", i));
        }
        
        Self { order_map }
    }

    fn parse_order_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(order_value) = self.order_map.get(class) {
            return Some(vec![CssProperty { name: "order".to_string(), value: order_value.clone(), important: false }]);
        }
        None
    }

    fn parse_arbitrary_order_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("order-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "order".to_string(), value: value.to_string(), important: false }]);
            }
        }
        None
    }

    fn parse_custom_property_order_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("order-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "order".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        None
    }
}

impl UtilityParser for OrderParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_order_class(class)
            .or_else(|| self.parse_arbitrary_order_class(class))
            .or_else(|| self.parse_custom_property_order_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "order-1", "order-2", "order-3", "order-4", "order-5", "order-6", "order-7", "order-8", "order-9", "order-10", "order-11", "order-12",
            "-order-1", "-order-2", "-order-3", "-order-4", "-order-5", "-order-6", "-order-7", "-order-8", "-order-9", "-order-10", "-order-11", "-order-12",
            "order-first", "order-last", "order-none",
            "order-[*]", "order-(*)"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Flexbox }
}

impl Default for OrderParser {
    fn default() -> Self { Self::new() }
}
