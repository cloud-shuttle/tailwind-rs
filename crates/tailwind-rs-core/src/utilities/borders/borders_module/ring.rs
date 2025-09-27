//! Ring utilities
//!
//! This module provides utilities for ring values.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Ring values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ring {
    /// No ring
    None,
    /// Thin ring
    Thin,
    /// Default ring
    Default,
    /// Medium ring
    Medium,
    /// Thick ring
    Thick,
}

impl Ring {
    /// Get the CSS value for the ring
    pub fn css_value(&self) -> &'static str {
        match self {
            Ring::None => "none",
            Ring::Thin => "0 0 0 1px var(--ring-color)",
            Ring::Default => "0 0 0 2px var(--ring-color)",
            Ring::Medium => "0 0 0 4px var(--ring-color)",
            Ring::Thick => "0 0 0 8px var(--ring-color)",
        }
    }

    /// Get the Tailwind class for the ring
    pub fn class_name(&self) -> &'static str {
        match self {
            Ring::None => "ring-0",
            Ring::Thin => "ring-1",
            Ring::Default => "ring-2",
            Ring::Medium => "ring-4",
            Ring::Thick => "ring-8",
        }
    }
}

impl fmt::Display for Ring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_value())
    }
}

impl ClassBuilder {
    /// Set ring
    pub fn ring(&mut self, ring: Ring) -> &mut Self {
        *self = self.clone().class(ring.class_name());
        self
    }

    /// Set ring to none
    pub fn ring_0(&mut self) -> &mut Self {
        self.ring(Ring::None)
    }

    /// Set ring to thin
    pub fn ring_1(&mut self) -> &mut Self {
        self.ring(Ring::Thin)
    }

    /// Set ring to default
    pub fn ring_2(&mut self) -> &mut Self {
        self.ring(Ring::Default)
    }

    /// Set ring to medium
    pub fn ring_4(&mut self) -> &mut Self {
        self.ring(Ring::Medium)
    }

    /// Set ring to thick
    pub fn ring_8(&mut self) -> &mut Self {
        self.ring(Ring::Thick)
    }
}
