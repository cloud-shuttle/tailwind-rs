//! Performance Optimization Module
//! 
//! This module contains the modularized performance optimization utilities.
//! Split from the original 823-line performance_optimization.rs file.

pub mod core_structures;
pub mod bundle_analysis;
pub mod performance_monitoring;

// Re-export the main components
pub use core_structures::*;
pub use bundle_analysis::*;
pub use performance_monitoring::*;
