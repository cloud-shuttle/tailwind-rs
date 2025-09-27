//! Validation system for tailwind-rs
//!
//! This module provides class validation functionality to ensure that
//! generated Tailwind CSS classes are valid and don't conflict.

use crate::custom_variant::CustomVariantManager;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

pub mod validation_errors;
pub mod validation_rules;
pub mod class_validator;
pub mod conflict_detector;
pub mod deprecated_checker;
pub mod custom_variant_validator;
pub mod validation_engine;

pub use validation_errors::*;
pub use validation_rules::*;
pub use class_validator::*;
pub use conflict_detector::*;
pub use deprecated_checker::*;
pub use custom_variant_validator::*;
pub use validation_engine::*;
