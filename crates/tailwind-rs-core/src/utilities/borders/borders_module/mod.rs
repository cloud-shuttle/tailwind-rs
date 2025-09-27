//! Border utilities for tailwind-rs
//!
//! This module provides utilities for border width, border style, border radius,
//! border color, outline, and divide utilities.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod border_width;
pub mod border_style;
pub mod border_radius;
pub mod border_color;
pub mod outline;
pub mod divide;
pub mod ring;
pub mod border_utilities;

pub use border_width::*;
pub use border_style::*;
pub use border_radius::*;
pub use border_color::*;
pub use outline::*;
pub use divide::*;
pub use ring::*;
pub use border_utilities::*;
