//! Flexbox utilities for tailwind-rs
//!
//! This module provides utilities for flex direction, wrap, justify-content, align-items,
//! align-content, align-self, flex-grow, flex-shrink, flex-basis, flex, and order.

pub mod align_content;
pub mod align_items;
pub mod align_self;
pub mod direction;
pub mod flex;
pub mod flex_basis;
pub mod flex_grow;
pub mod flex_shrink;
pub mod justify_content;
pub mod order;
pub mod wrap;

// Re-export all the main types and traits
pub use align_content::{AlignContent, AlignContentUtilities};
pub use align_items::{AlignItems, AlignItemsUtilities};
pub use align_self::{AlignSelf, AlignSelfUtilities};
pub use direction::{FlexDirection, FlexDirectionUtilities};
pub use flex::{Flex, FlexUtilities};
pub use flex_basis::{FlexBasis, FlexBasisUtilities};
pub use flex_grow::{FlexGrow, FlexGrowUtilities};
pub use flex_shrink::{FlexShrink, FlexShrinkUtilities};
pub use justify_content::{JustifyContent, JustifyContentUtilities};
pub use order::{Order, OrderUtilities};
pub use wrap::{FlexWrap, FlexWrapUtilities};
