//! ClassSet implementation
//!
//! This module contains the ClassSet struct and its methods.
//!
//! ## Example
//!
//! ```rust
//! use tailwind_rs_core::ClassSet;
//!
//! // Create and use a class set
//! let mut class_set = ClassSet::new();
//! class_set.add_class("bg-blue-500");
//! class_set.add_class("text-white");
//! class_set.add_class("hover:bg-blue-600");
//!
//! // Check if class exists
//! let has_class = class_set.has_class("bg-blue-500"); // true
//!
//! // Get all classes
//! let classes = class_set.get_classes();
//!
//! // Convert to CSS string
//! let css = class_set.to_css_classes();
//! // Result: "bg-blue-500 text-white hover:bg-blue-600"
//! ```

use crate::responsive::Breakpoint;
use std::collections::{HashMap, HashSet};

/// A set of CSS classes with metadata
#[derive(Debug, Clone, PartialEq)]
pub struct ClassSet {
    /// The actual CSS classes
    pub classes: HashSet<String>,
    /// Responsive classes organized by breakpoint
    pub responsive: HashMap<Breakpoint, HashSet<String>>,
    /// Conditional classes
    pub conditional: HashMap<String, HashSet<String>>,
    /// Custom CSS properties
    pub custom: HashMap<String, String>,
}

impl ClassSet {
    /// Create a new empty class set
    pub fn new() -> Self {
        Self {
            classes: HashSet::new(),
            responsive: HashMap::new(),
            conditional: HashMap::new(),
            custom: HashMap::new(),
        }
    }

    /// Add a base class
    pub fn add_class(&mut self, class: impl Into<String>) {
        self.classes.insert(class.into());
    }

    /// Add multiple base classes
    pub fn add_classes(&mut self, classes: impl IntoIterator<Item = String>) {
        for class in classes {
            self.classes.insert(class);
        }
    }

    /// Add a responsive class
    pub fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: impl Into<String>) {
        self.responsive
            .entry(breakpoint)
            .or_default()
            .insert(class.into());
    }

    /// Add a conditional class
    pub fn add_conditional_class(
        &mut self,
        condition: impl Into<String>,
        class: impl Into<String>,
    ) {
        self.conditional
            .entry(condition.into())
            .or_default()
            .insert(class.into());
    }

    /// Add a custom CSS property
    pub fn add_custom(&mut self, property: impl Into<String>, value: impl Into<String>) {
        self.custom.insert(property.into(), value.into());
    }

    /// Remove a base class
    pub fn remove_class(&mut self, class: &str) {
        self.classes.remove(class);
    }

    /// Check if a base class exists
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.contains(class)
    }

    /// Get all base classes as a vector
    pub fn get_classes(&self) -> Vec<String> {
        self.classes.iter().cloned().collect()
    }

    /// Get responsive classes for a specific breakpoint
    pub fn get_responsive_classes(&self, breakpoint: Breakpoint) -> Vec<String> {
        self.responsive
            .get(&breakpoint)
            .map(|classes| classes.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all responsive classes
    pub fn get_all_responsive_classes(&self) -> HashMap<Breakpoint, Vec<String>> {
        self.responsive
            .iter()
            .map(|(breakpoint, classes)| (*breakpoint, classes.iter().cloned().collect()))
            .collect()
    }

    /// Get conditional classes for a specific condition
    pub fn get_conditional_classes(&self, condition: &str) -> Vec<String> {
        self.conditional
            .get(condition)
            .map(|classes| classes.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all conditional classes
    pub fn get_all_conditional_classes(&self) -> HashMap<String, Vec<String>> {
        self.conditional
            .iter()
            .map(|(condition, classes)| (condition.clone(), classes.iter().cloned().collect()))
            .collect()
    }

    /// Get custom CSS properties
    pub fn get_custom_properties(&self) -> HashMap<String, String> {
        self.custom.clone()
    }

    /// Convert to CSS class string
    pub fn to_css_classes(&self) -> String {
        let mut result = Vec::new();

        // Add base classes
        let mut base_classes: Vec<String> = self.classes.iter().cloned().collect();
        base_classes.sort();
        result.extend(base_classes);

        // Add responsive classes
        let mut responsive_classes: Vec<(Breakpoint, String)> = self
            .responsive
            .iter()
            .flat_map(|(breakpoint, classes)| {
                classes
                    .iter()
                    .map(|class| (*breakpoint, format!("{}{}", breakpoint.prefix(), class)))
            })
            .collect();
        responsive_classes.sort_by(|a, b| a.0.min_width().cmp(&b.0.min_width()));
        result.extend(responsive_classes.into_iter().map(|(_, class)| class));

        // Add custom variant classes (Tailwind v4.1.13 @custom-variant support)
        let mut custom_variant_classes: Vec<String> = self
            .conditional
            .iter()
            .flat_map(|(variant, classes)| {
                let variant = variant.clone();
                classes
                    .iter()
                    .map(move |class| format!("{}:{}", variant, class))
            })
            .collect();
        custom_variant_classes.sort();
        result.extend(custom_variant_classes);

        result.join(" ")
    }

    /// Convert to CSS custom properties string
    pub fn to_css_custom_properties(&self) -> String {
        if self.custom.is_empty() {
            return String::new();
        }

        let properties: Vec<String> = self
            .custom
            .iter()
            .map(|(property, value)| format!("--{}: {}", property, value))
            .collect();

        format!("style=\"{}\"", properties.join("; "))
    }

    /// Merge another class set into this one
    pub fn merge(&mut self, other: ClassSet) {
        self.classes.extend(other.classes);

        for (breakpoint, classes) in other.responsive {
            self.responsive
                .entry(breakpoint)
                .or_default()
                .extend(classes);
        }

        for (condition, classes) in other.conditional {
            self.conditional
                .entry(condition)
                .or_default()
                .extend(classes);
        }

        self.custom.extend(other.custom);
    }

    /// Check if the class set is empty
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
            && self.responsive.is_empty()
            && self.conditional.is_empty()
            && self.custom.is_empty()
    }

    /// Get the total number of classes
    pub fn len(&self) -> usize {
        self.classes.len()
            + self
                .responsive
                .values()
                .map(|classes| classes.len())
                .sum::<usize>()
            + self
                .conditional
                .values()
                .map(|classes| classes.len())
                .sum::<usize>()
    }
}

impl Default for ClassSet {
    fn default() -> Self {
        Self::new()
    }
}
