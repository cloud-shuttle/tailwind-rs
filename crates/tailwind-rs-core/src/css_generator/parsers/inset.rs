//! Inset Utilities Parser
//!
//! This module provides parsing logic for Tailwind CSS inset utilities,
//! such as `top-0`, `right-4`, `bottom-auto`, `left-1/2`, `inset-0`, etc.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InsetParser {
    spacing_map: HashMap<String, String>,
    fraction_map: HashMap<String, String>,
}

impl InsetParser {
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

        Self { spacing_map, fraction_map }
    }

    fn parse_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("inset-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "inset".to_string(), value: spacing_value.clone(), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "inset".to_string(), value: fraction_value.clone(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("-inset-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "inset".to_string(), value: format!("-{}", spacing_value), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "inset".to_string(), value: format!("-{}", fraction_value), important: false }]);
            }
        }
        if class == "inset-auto" {
            return Some(vec![CssProperty { name: "inset".to_string(), value: "auto".to_string(), important: false }]);
        }
        None
    }

    fn parse_inset_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("inset-x-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "left".to_string(), value: spacing_value.clone(), important: false }, CssProperty { name: "right".to_string(), value: spacing_value.clone(), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "left".to_string(), value: fraction_value.clone(), important: false }, CssProperty { name: "right".to_string(), value: fraction_value.clone(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("-inset-x-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "left".to_string(), value: format!("-{}", spacing_value), important: false }, CssProperty { name: "right".to_string(), value: format!("-{}", spacing_value), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "left".to_string(), value: format!("-{}", fraction_value), important: false }, CssProperty { name: "right".to_string(), value: format!("-{}", fraction_value), important: false }]);
            }
        }
        None
    }

    fn parse_inset_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("inset-y-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "top".to_string(), value: spacing_value.clone(), important: false }, CssProperty { name: "bottom".to_string(), value: spacing_value.clone(), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "top".to_string(), value: fraction_value.clone(), important: false }, CssProperty { name: "bottom".to_string(), value: fraction_value.clone(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("-inset-y-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "top".to_string(), value: format!("-{}", spacing_value), important: false }, CssProperty { name: "bottom".to_string(), value: format!("-{}", spacing_value), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "top".to_string(), value: format!("-{}", fraction_value), important: false }, CssProperty { name: "bottom".to_string(), value: format!("-{}", fraction_value), important: false }]);
            }
        }
        None
    }

    fn parse_top_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("top-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "top".to_string(), value: spacing_value.clone(), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "top".to_string(), value: fraction_value.clone(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("-top-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "top".to_string(), value: format!("-{}", spacing_value), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "top".to_string(), value: format!("-{}", fraction_value), important: false }]);
            }
        }
        if class == "top-auto" {
            return Some(vec![CssProperty { name: "top".to_string(), value: "auto".to_string(), important: false }]);
        }
        None
    }

    fn parse_right_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("right-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "right".to_string(), value: spacing_value.clone(), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "right".to_string(), value: fraction_value.clone(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("-right-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "right".to_string(), value: format!("-{}", spacing_value), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "right".to_string(), value: format!("-{}", fraction_value), important: false }]);
            }
        }
        if class == "right-auto" {
            return Some(vec![CssProperty { name: "right".to_string(), value: "auto".to_string(), important: false }]);
        }
        None
    }

    fn parse_bottom_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("bottom-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "bottom".to_string(), value: spacing_value.clone(), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "bottom".to_string(), value: fraction_value.clone(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("-bottom-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "bottom".to_string(), value: format!("-{}", spacing_value), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "bottom".to_string(), value: format!("-{}", fraction_value), important: false }]);
            }
        }
        if class == "bottom-auto" {
            return Some(vec![CssProperty { name: "bottom".to_string(), value: "auto".to_string(), important: false }]);
        }
        None
    }

    fn parse_left_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("left-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "left".to_string(), value: spacing_value.clone(), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "left".to_string(), value: fraction_value.clone(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("-left-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "left".to_string(), value: format!("-{}", spacing_value), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "left".to_string(), value: format!("-{}", fraction_value), important: false }]);
            }
        }
        if class == "left-auto" {
            return Some(vec![CssProperty { name: "left".to_string(), value: "auto".to_string(), important: false }]);
        }
        None
    }

    fn parse_start_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("start-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "inset-inline-start".to_string(), value: spacing_value.clone(), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "inset-inline-start".to_string(), value: fraction_value.clone(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("-start-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "inset-inline-start".to_string(), value: format!("-{}", spacing_value), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "inset-inline-start".to_string(), value: format!("-{}", fraction_value), important: false }]);
            }
        }
        if class == "start-auto" {
            return Some(vec![CssProperty { name: "inset-inline-start".to_string(), value: "auto".to_string(), important: false }]);
        }
        None
    }

    fn parse_end_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("end-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "inset-inline-end".to_string(), value: spacing_value.clone(), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "inset-inline-end".to_string(), value: fraction_value.clone(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("-end-") {
            if let Some(spacing_value) = self.spacing_map.get(value) {
                return Some(vec![CssProperty { name: "inset-inline-end".to_string(), value: format!("-{}", spacing_value), important: false }]);
            }
            if let Some(fraction_value) = self.fraction_map.get(value) {
                return Some(vec![CssProperty { name: "inset-inline-end".to_string(), value: format!("-{}", fraction_value), important: false }]);
            }
        }
        if class == "end-auto" {
            return Some(vec![CssProperty { name: "inset-inline-end".to_string(), value: "auto".to_string(), important: false }]);
        }
        None
    }

    fn parse_arbitrary_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value) = class.strip_prefix("inset-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "inset".to_string(), value: value.to_string(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("top-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "top".to_string(), value: value.to_string(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("right-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "right".to_string(), value: value.to_string(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("bottom-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "bottom".to_string(), value: value.to_string(), important: false }]);
            }
        }
        if let Some(value) = class.strip_prefix("left-[") {
            if let Some(value) = value.strip_suffix("]") {
                return Some(vec![CssProperty { name: "left".to_string(), value: value.to_string(), important: false }]);
            }
        }
        None
    }

    fn parse_custom_property_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(prop) = class.strip_prefix("inset-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "inset".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("top-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "top".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("right-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "right".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("bottom-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "bottom".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("left-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "left".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("start-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "inset-inline-start".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        if let Some(prop) = class.strip_prefix("end-(") {
            if let Some(prop) = prop.strip_suffix(")") {
                return Some(vec![CssProperty { name: "inset-inline-end".to_string(), value: format!("var({})", prop), important: false }]);
            }
        }
        None
    }
}

impl UtilityParser for InsetParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_inset_class(class)
            .or_else(|| self.parse_inset_x_class(class))
            .or_else(|| self.parse_inset_y_class(class))
            .or_else(|| self.parse_top_class(class))
            .or_else(|| self.parse_right_class(class))
            .or_else(|| self.parse_bottom_class(class))
            .or_else(|| self.parse_left_class(class))
            .or_else(|| self.parse_start_class(class))
            .or_else(|| self.parse_end_class(class))
            .or_else(|| self.parse_arbitrary_inset_class(class))
            .or_else(|| self.parse_custom_property_inset_class(class))
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "inset-0", "inset-px", "inset-0.5", "inset-1", "inset-1.5", "inset-2", "inset-2.5", "inset-3", "inset-3.5", "inset-4",
            "inset-5", "inset-6", "inset-7", "inset-8", "inset-9", "inset-10", "inset-11", "inset-12", "inset-14", "inset-16",
            "inset-20", "inset-24", "inset-28", "inset-32", "inset-36", "inset-40", "inset-44", "inset-48", "inset-52", "inset-56",
            "inset-60", "inset-64", "inset-72", "inset-80", "inset-96", "inset-auto", "inset-full",
            "inset-x-*", "inset-y-*", "top-*", "right-*", "bottom-*", "left-*", "start-*", "end-*",
            "inset-[*]", "top-[*]", "right-[*]", "bottom-[*]", "left-[*]", "inset-(*)"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Layout }
}

impl Default for InsetParser {
    fn default() -> Self { Self::new() }
}
