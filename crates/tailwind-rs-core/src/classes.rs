//! Class management system for tailwind-rs

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

/// Builder for creating class sets
#[derive(Debug, Clone)]
pub struct ClassBuilder {
    class_set: ClassSet,
}

impl ClassBuilder {
    /// Create a new class builder
    pub fn new() -> Self {
        Self {
            class_set: ClassSet::new(),
        }
    }

    /// Add a base class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class_set.add_class(class);
        self
    }

    /// Add multiple base classes
    pub fn classes(mut self, classes: impl IntoIterator<Item = String>) -> Self {
        self.class_set.add_classes(classes);
        self
    }

    /// Add a responsive class
    pub fn responsive(mut self, breakpoint: Breakpoint, class: impl Into<String>) -> Self {
        self.class_set.add_responsive_class(breakpoint, class);
        self
    }

    /// Add a conditional class
    pub fn conditional(mut self, condition: impl Into<String>, class: impl Into<String>) -> Self {
        self.class_set.add_conditional_class(condition, class);
        self
    }

    /// Add a custom CSS property
    pub fn custom(mut self, property: impl Into<String>, value: impl Into<String>) -> Self {
        self.class_set.add_custom(property, value);
        self
    }

    /// Add a custom variant class (Tailwind v4.1.13 @custom-variant support)
    pub fn custom_variant(mut self, variant: impl Into<String>, class: impl Into<String>) -> Self {
        let variant = variant.into();
        let class = class.into();
        
        // Add the variant as a conditional class
        self.class_set.add_conditional_class(variant, class);
        self
    }

    /// Add an ARIA variant class
    pub fn aria(self, aria_attr: impl Into<String>, class: impl Into<String>) -> Self {
        let variant = format!("aria-{}", aria_attr.into());
        self.custom_variant(variant, class)
    }

    /// Add a data variant class
    pub fn data(self, data_attr: impl Into<String>, value: Option<String>, class: impl Into<String>) -> Self {
        let variant = if let Some(val) = value {
            format!("data-{}={}", data_attr.into(), val)
        } else {
            format!("data-{}", data_attr.into())
        };
        self.custom_variant(variant, class)
    }

    /// Add a supports variant class
    pub fn supports(self, feature: impl Into<String>, class: impl Into<String>) -> Self {
        let variant = format!("supports-{}", feature.into());
        self.custom_variant(variant, class)
    }

    /// Build the class set
    pub fn build(self) -> ClassSet {
        self.class_set
    }

    /// Build the class set and convert to CSS string
    pub fn build_string(self) -> String {
        self.class_set.to_css_classes()
    }
}

impl Default for ClassBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for class management
#[allow(clippy::module_inception)]
pub mod classes {
    use super::*;

    /// Create a new class set with base classes
    pub fn new(classes: impl IntoIterator<Item = String>) -> ClassSet {
        let mut class_set = ClassSet::new();
        class_set.add_classes(classes);
        class_set
    }

    /// Create a responsive class set
    pub fn responsive(
        base: impl IntoIterator<Item = String>,
        responsive: impl IntoIterator<Item = (Breakpoint, String)>,
    ) -> ClassSet {
        let mut class_set = ClassSet::new();
        class_set.add_classes(base);

        for (breakpoint, class) in responsive {
            class_set.add_responsive_class(breakpoint, class);
        }

        class_set
    }

    /// Create a conditional class set
    pub fn conditional(
        base: impl IntoIterator<Item = String>,
        conditional: impl IntoIterator<Item = (String, String)>,
    ) -> ClassSet {
        let mut class_set = ClassSet::new();
        class_set.add_classes(base);

        for (condition, class) in conditional {
            class_set.add_conditional_class(condition, class);
        }

        class_set
    }

    /// Merge multiple class sets
    pub fn merge(class_sets: impl IntoIterator<Item = ClassSet>) -> ClassSet {
        let mut result = ClassSet::new();

        for class_set in class_sets {
            result.merge(class_set);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_set_creation() {
        let class_set = ClassSet::new();
        assert!(class_set.is_empty());
        assert_eq!(class_set.len(), 0);
    }

    #[test]
    fn test_class_set_add_remove() {
        let mut class_set = ClassSet::new();

        class_set.add_class("bg-blue-500");
        assert!(class_set.has_class("bg-blue-500"));
        assert_eq!(class_set.len(), 1);

        class_set.add_class("text-white");
        assert!(class_set.has_class("text-white"));
        assert_eq!(class_set.len(), 2);

        class_set.remove_class("bg-blue-500");
        assert!(!class_set.has_class("bg-blue-500"));
        assert!(class_set.has_class("text-white"));
        assert_eq!(class_set.len(), 1);
    }

    #[test]
    fn test_class_set_responsive() {
        let mut class_set = ClassSet::new();

        class_set.add_responsive_class(Breakpoint::Sm, "text-sm");
        class_set.add_responsive_class(Breakpoint::Md, "text-md");

        let sm_classes = class_set.get_responsive_classes(Breakpoint::Sm);
        assert_eq!(sm_classes, vec!["text-sm"]);

        let md_classes = class_set.get_responsive_classes(Breakpoint::Md);
        assert_eq!(md_classes, vec!["text-md"]);

        let lg_classes = class_set.get_responsive_classes(Breakpoint::Lg);
        assert!(lg_classes.is_empty());
    }

    #[test]
    fn test_class_set_conditional() {
        let mut class_set = ClassSet::new();

        class_set.add_conditional_class("hover", "hover:bg-blue-600");
        class_set.add_conditional_class("focus", "focus:ring-2");

        let hover_classes = class_set.get_conditional_classes("hover");
        assert_eq!(hover_classes, vec!["hover:bg-blue-600"]);

        let focus_classes = class_set.get_conditional_classes("focus");
        assert_eq!(focus_classes, vec!["focus:ring-2"]);
    }

    #[test]
    fn test_class_set_custom() {
        let mut class_set = ClassSet::new();

        class_set.add_custom("primary-color", "#3b82f6");
        class_set.add_custom("spacing", "1rem");

        let custom_properties = class_set.get_custom_properties();
        assert_eq!(
            custom_properties.get("primary-color"),
            Some(&"#3b82f6".to_string())
        );
        assert_eq!(custom_properties.get("spacing"), Some(&"1rem".to_string()));
    }

    #[test]
    fn test_class_set_to_css() {
        let mut class_set = ClassSet::new();
        class_set.add_class("bg-blue-500");
        class_set.add_class("text-white");
        class_set.add_responsive_class(Breakpoint::Sm, "text-sm");
        class_set.add_responsive_class(Breakpoint::Md, "text-md");

        let css = class_set.to_css_classes();
        assert!(css.contains("bg-blue-500"));
        assert!(css.contains("text-white"));
        // Note: responsive classes are sorted by breakpoint
        assert!(css.contains("sm:text-sm"));
        assert!(css.contains("md:text-md"));
    }

    #[test]
    fn test_class_set_merge() {
        let mut class_set1 = ClassSet::new();
        class_set1.add_class("bg-blue-500");
        class_set1.add_responsive_class(Breakpoint::Sm, "text-sm");

        let mut class_set2 = ClassSet::new();
        class_set2.add_class("text-white");
        class_set2.add_responsive_class(Breakpoint::Md, "text-md");

        class_set1.merge(class_set2);

        assert!(class_set1.has_class("bg-blue-500"));
        assert!(class_set1.has_class("text-white"));
        assert_eq!(
            class_set1.get_responsive_classes(Breakpoint::Sm),
            vec!["text-sm"]
        );
        assert_eq!(
            class_set1.get_responsive_classes(Breakpoint::Md),
            vec!["text-md"]
        );
    }

    #[test]
    fn test_class_builder() {
        let class_set = ClassBuilder::new()
            .class("bg-blue-500")
            .class("text-white")
            .responsive(Breakpoint::Sm, "text-sm")
            .conditional("hover", "hover:bg-blue-600")
            .custom("primary-color", "#3b82f6")
            .build();

        assert!(class_set.has_class("bg-blue-500"));
        assert!(class_set.has_class("text-white"));
        assert_eq!(
            class_set.get_responsive_classes(Breakpoint::Sm),
            vec!["text-sm"]
        );
        assert_eq!(
            class_set.get_conditional_classes("hover"),
            vec!["hover:bg-blue-600"]
        );
        assert_eq!(
            class_set.get_custom_properties().get("primary-color"),
            Some(&"#3b82f6".to_string())
        );
    }

    #[test]
    fn test_classes_utility_functions() {
        let class_set = classes::new(vec!["bg-blue-500".to_string(), "text-white".to_string()]);
        assert!(class_set.has_class("bg-blue-500"));
        assert!(class_set.has_class("text-white"));

        let responsive_class_set = classes::responsive(
            vec!["bg-blue-500".to_string()],
            vec![(Breakpoint::Sm, "text-sm".to_string())],
        );
        assert!(responsive_class_set.has_class("bg-blue-500"));
        assert_eq!(
            responsive_class_set.get_responsive_classes(Breakpoint::Sm),
            vec!["text-sm"]
        );

        let conditional_class_set = classes::conditional(
            vec!["bg-blue-500".to_string()],
            vec![("hover".to_string(), "hover:bg-blue-600".to_string())],
        );
        assert!(conditional_class_set.has_class("bg-blue-500"));
        assert_eq!(
            conditional_class_set.get_conditional_classes("hover"),
            vec!["hover:bg-blue-600"]
        );
    }
}
