//! Grid utilities for tailwind-rs
//!
//! This module provides utilities for CSS Grid layout including grid template columns,
//! grid template rows, grid column span, grid row span, grid auto flow, and gap utilities.

pub mod template_columns;
pub mod template_rows;
pub mod column_span;
pub mod row_span;
pub mod auto_flow;
pub mod auto_columns;
pub mod auto_rows;
pub mod gap;
pub mod placement;

// Re-export all the main types and traits
pub use template_columns::{GridTemplateColumns, GridTemplateColumnsUtilities};
pub use template_rows::{GridTemplateRows, GridTemplateRowsUtilities};
pub use column_span::{GridColumnSpan, GridColumnSpanUtilities};
pub use row_span::{GridRowSpan, GridRowSpanUtilities};
pub use auto_flow::{GridAutoFlow, GridAutoFlowUtilities};
pub use auto_columns::{GridAutoColumns, GridAutoColumnsUtilities};
pub use auto_rows::{GridAutoRows, GridAutoRowsUtilities};
pub use gap::{GridGap, GridGapUtilities};
pub use placement::{GridColumnStart, GridColumnEnd, GridRowStart, GridRowEnd, GridPlacementUtilities};
