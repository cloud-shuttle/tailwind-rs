//! Class management system for tailwind-rs
//!
//! This module provides the core class management functionality,
//! including ClassSet, ClassBuilder, and utility functions.

pub mod class_builder;
pub mod class_set;
pub mod utilities;

// Re-export main types
pub use class_builder::ClassBuilder;
pub use class_set::ClassSet;
pub use utilities::*;
