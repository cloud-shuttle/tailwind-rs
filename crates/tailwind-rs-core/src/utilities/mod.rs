//! Utility class implementations for tailwind-rs
//!
//! This module contains all the utility class implementations organized by category.
//! Each utility category follows a consistent pattern with traits and implementations.

pub mod spacing;
pub mod sizing;
pub mod typography;
pub mod colors;
pub mod color_functions;
pub mod container_queries;
pub mod layout;
pub mod flexbox;
pub mod grid;
pub mod backgrounds;
pub mod borders;
pub mod effects;
pub mod filters;
pub mod transitions;
pub mod transforms;
pub mod animations;
pub mod advanced_animations;
pub mod performance_optimization;
pub mod interactivity;

// Re-export all utility traits for easy access
pub use spacing::*;
pub use sizing::*;
pub use typography::*;
pub use colors::*;
pub use color_functions::*;
pub use container_queries::*;
pub use layout::*;
pub use flexbox::*;
pub use grid::*;
pub use backgrounds::*;
pub use borders::*;
pub use effects::*;
pub use filters::*;
pub use transitions::*;
pub use transforms::*;
pub use animations::*;
pub use advanced_animations::*;
pub use performance_optimization::*;
pub use interactivity::*;
