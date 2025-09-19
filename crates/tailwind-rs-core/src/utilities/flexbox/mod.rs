//! Flexbox utilities for tailwind-rs
//!
//! This module provides utilities for flex direction, wrap, justify-content, align-items,
//! align-content, align-self, flex-grow, flex-shrink, flex-basis, flex, and order.

pub mod direction;
pub mod wrap;
pub mod justify_content;
pub mod align_items;
pub mod align_content;
pub mod align_self;
pub mod flex_grow;
pub mod flex_shrink;
pub mod flex_basis;
pub mod flex;
pub mod order;

// Re-export all the main types and traits
pub use direction::{FlexDirection, FlexDirectionUtilities};
pub use wrap::{FlexWrap, FlexWrapUtilities};
pub use justify_content::{JustifyContent, JustifyContentUtilities};
pub use align_items::{AlignItems, AlignItemsUtilities};
pub use align_content::{AlignContent, AlignContentUtilities};
pub use align_self::{AlignSelf, AlignSelfUtilities};
pub use flex_grow::{FlexGrow, FlexGrowUtilities};
pub use flex_shrink::{FlexShrink, FlexShrinkUtilities};
pub use flex_basis::{FlexBasis, FlexBasisUtilities};
pub use flex::{Flex, FlexUtilities};
pub use order::{Order, OrderUtilities};
