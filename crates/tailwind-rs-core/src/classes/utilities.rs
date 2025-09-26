//! Utility functions for class management
//!
//! This module contains utility functions for working with class sets.

use super::ClassSet;
use crate::responsive::Breakpoint;

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
