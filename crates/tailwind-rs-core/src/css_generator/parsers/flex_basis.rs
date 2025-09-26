//! Flex Basis Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS flex-basis utilities,
//! such as `basis-64`, `basis-1/2`, `basis-full`, `basis-auto`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FlexBasisParser {
    spacing_map: HashMap<String, String>,
    fraction_map: HashMap<String, String>,
    container_map: HashMap<String, String>,
}

impl FlexBasisParser {
    pub fn new() -> Self {
        let mut spacing_map = HashMap::new();
        // Standard spacing values
        spacing_map.insert("0".to_string(), "0".to_string());
        spacing_map.insert("px".to_string(), "1px".to_string());
        spacing_map.insert("0.5".to_string(), "0.125rem".to_string());
        spacing_map.insert("1".to_string(), "0.25rem".to_string());
        spacing_map.insert("1.5".to_string(), "0.375rem".to_string());
        spacing_map.insert("2".to_string(), "0.5rem".to_string());
        spacing_map.insert("2.5".to_string(), "0.625rem".to_string());
        spacing_map.insert("3".to_string(), "0.75rem".to_string());
        spacing_map.insert("3.5".to_string(), "0.875rem".to_string());
        spacing_map.insert("4".to_string(), "1rem".to_string());
        spacing_map.insert("5".to_string(), "1.25rem".to_string());
        spacing_map.insert("6".to_string(), "1.5rem".to_string());
        spacing_map.insert("7".to_string(), "1.75rem".to_string());
        spacing_map.insert("8".to_string(), "2rem".to_string());
        spacing_map.insert("9".to_string(), "2.25rem".to_string());
        spacing_map.insert("10".to_string(), "2.5rem".to_string());
        spacing_map.insert("11".to_string(), "2.75rem".to_string());
        spacing_map.insert("12".to_string(), "3rem".to_string());
        spacing_map.insert("14".to_string(), "3.5rem".to_string());
        spacing_map.insert("16".to_string(), "4rem".to_string());
        spacing_map.insert("20".to_string(), "5rem".to_string());
        spacing_map.insert("24".to_string(), "6rem".to_string());
        spacing_map.insert("28".to_string(), "7rem".to_string());
        spacing_map.insert("32".to_string(), "8rem".to_string());
        spacing_map.insert("36".to_string(), "9rem".to_string());
        spacing_map.insert("40".to_string(), "10rem".to_string());
        spacing_map.insert("44".to_string(), "11rem".to_string());
        spacing_map.insert("48".to_string(), "12rem".to_string());
        spacing_map.insert("52".to_string(), "13rem".to_string());
        spacing_map.insert("56".to_string(), "14rem".to_string());
        spacing_map.insert("60".to_string(), "15rem".to_string());
        spacing_map.insert("64".to_string(), "16rem".to_string());
        spacing_map.insert("72".to_string(), "18rem".to_string());
        spacing_map.insert("80".to_string(), "20rem".to_string());
        spacing_map.insert("96".to_string(), "24rem".to_string());
        spacing_map.insert("auto".to_string(), "auto".to_string());
        spacing_map.insert("full".to_string(), "100%".to_string());

        let mut fraction_map = HashMap::new();
        fraction_map.insert("1/2".to_string(), "50%".to_string());
        fraction_map.insert("1/3".to_string(), "33.333333%".to_string());
        fraction_map.insert("2/3".to_string(), "66.666667%".to_string());
        fraction_map.insert("1/4".to_string(), "25%".to_string());
        fraction_map.insert("2/4".to_string(), "50%".to_string());
        fraction_map.insert("3/4".to_string(), "75%".to_string());
        fraction_map.insert("1/5".to_string(), "20%".to_string());
        fraction_map.insert("2/5".to_string(), "40%".to_string());
        fraction_map.insert("3/5".to_string(), "60%".to_string());
        fraction_map.insert("4/5".to_string(), "80%".to_string());
        fraction_map.insert("1/6".to_string(), "16.666667%".to_string());
        fraction_map.insert("2/6".to_string(), "33.333333%".to_string());
        fraction_map.insert("3/6".to_string(), "50%".to_string());
        fraction_map.insert("4/6".to_string(), "66.666667%".to_string());
        fraction_map.insert("5/6".to_string(), "83.333333%".to_string());
        fraction_map.insert("1/12".to_string(), "8.333333%".to_string());
        fraction_map.insert("2/12".to_string(), "16.666667%".to_string());
        fraction_map.insert("3/12".to_string(), "25%".to_string());
        fraction_map.insert("4/12".to_string(), "33.333333%".to_string());
        fraction_map.insert("5/12".to_string(), "41.666667%".to_string());
        fraction_map.insert("6/12".to_string(), "50%".to_string());
        fraction_map.insert("7/12".to_string(), "58.333333%".to_string());
        fraction_map.insert("8/12".to_string(), "66.666667%".to_string());
        fraction_map.insert("9/12".to_string(), "75%".to_string());
        fraction_map.insert("10/12".to_string(), "83.333333%".to_string());
        fraction_map.insert("11/12".to_string(), "91.666667%".to_string());

        let mut container_map = HashMap::new();
        container_map.insert("3xs".to_string(), "var(--container-3xs)".to_string()); // 16rem
        container_map.insert("2xs".to_string(), "var(--container-2xs)".to_string()); // 18rem
        container_map.insert("xs".to_string(), "var(--container-xs)".to_string());  // 20rem
        container_map.insert("sm".to_string(), "var(--container-sm)".to_string());  // 24rem
        container_map.insert("md".to_string(), "var(--container-md)".to_string());  // 28rem
        container_map.insert("lg".to_string(), "var(--container-lg)".to_string());  // 32rem
        container_map.insert("xl".to_string(), "var(--container-xl)".to_string());  // 36rem
        container_map.insert("2xl".to_string(), "var(--container-2xl)".to_string()); // 42rem
        container_map.insert("3xl".to_string(), "var(--container-3xl)".to_string()); // 48rem
        container_map.insert("4xl".to_string(), "var(--container-4xl)".to_string()); // 56rem
        container_map.insert("5xl".to_string(), "var(--container-5xl)".to_string()); // 64rem
        container_map.insert("6xl".to_string(), "var(--container-6xl)".to_string()); // 72rem
        container_map.insert("7xl".to_string(), "var(--container-7xl)".to_string()); // 80rem

        Self { spacing_map, fraction_map, container_map }
    }

    fn parse_basis_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("basis-") {
            // Try spacing values first
            if let Some(basis_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "flex-basis".to_string(), value: basis_value.clone(), important: false }]);
            }
            // Try fraction values
            if let Some(basis_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "flex-basis".to_string(), value: basis_value.clone(), important: false }]);
            }
            // Try container values
            if let Some(basis_value) = self.container_map.get(value) {
                return Some(vec![CssProperty { name: "flex-basis".to_string(), value: basis_value.clone(), important: false }]);
            }
            // Handle arbitrary values like basis-[30vw]
            if value.starts_with('[') && value.ends_with(']') {
                let inner_value = value[1..value.len() - 1].to_string();
                return Some(vec![CssProperty { name: "flex-basis".to_string(), value: inner_value, important: false }]);
            }
            // Handle custom properties like basis-(--my-basis)
            if value.starts_with('(') && value.ends_with(')') {
                let inner_value = value[1..value.len() - 1].to_string();
                return Some(vec![CssProperty { name: "flex-basis".to_string(), value: format!("var({})", inner_value), important: false }]);
            }
        }
        None
    }
}

impl UtilityParser for FlexBasisParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_basis_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "basis-0", "basis-px", "basis-0.5", "basis-1", "basis-1.5", "basis-2", "basis-2.5", "basis-3", "basis-3.5", "basis-4",
            "basis-5", "basis-6", "basis-7", "basis-8", "basis-9", "basis-10", "basis-11", "basis-12", "basis-14", "basis-16",
            "basis-20", "basis-24", "basis-28", "basis-32", "basis-36", "basis-40", "basis-44", "basis-48", "basis-52", "basis-56",
            "basis-60", "basis-64", "basis-72", "basis-80", "basis-96", "basis-auto", "basis-full",
            "basis-1/2", "basis-1/3", "basis-2/3", "basis-1/4", "basis-2/4", "basis-3/4", "basis-1/5", "basis-2/5", "basis-3/5", "basis-4/5",
            "basis-1/6", "basis-2/6", "basis-3/6", "basis-4/6", "basis-5/6", "basis-1/12", "basis-2/12", "basis-3/12", "basis-4/12", "basis-5/12",
            "basis-6/12", "basis-7/12", "basis-8/12", "basis-9/12", "basis-10/12", "basis-11/12",
            "basis-3xs", "basis-2xs", "basis-xs", "basis-sm", "basis-md", "basis-lg", "basis-xl", "basis-2xl", "basis-3xl", "basis-4xl",
            "basis-5xl", "basis-6xl", "basis-7xl",
            "basis-[*]", "basis-(*)"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Flexbox }
}

impl Default for FlexBasisParser {
    fn default() -> Self { Self::new() }
}
