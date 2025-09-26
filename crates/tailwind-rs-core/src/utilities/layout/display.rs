//! Display utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Display values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Display {
    /// Block display
    Block,
    /// Inline block display
    InlineBlock,
    /// Inline display
    Inline,
    /// Flex display
    Flex,
    /// Inline flex display
    InlineFlex,
    /// Grid display
    Grid,
    /// Inline grid display
    InlineGrid,
    /// Table display
    Table,
    /// Inline table display
    InlineTable,
    /// Table cell display
    TableCell,
    /// Table row display
    TableRow,
    /// Table column display
    TableColumn,
    /// Table column group display
    TableColumnGroup,
    /// Table footer group display
    TableFooterGroup,
    /// Table header group display
    TableHeaderGroup,
    /// Table row group display
    TableRowGroup,
    /// Flow root display
    FlowRoot,
    /// Contents display
    Contents,
    /// List item display
    ListItem,
    /// Hidden display
    Hidden,
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Display::Block => write!(f, "block"),
            Display::InlineBlock => write!(f, "inline-block"),
            Display::Inline => write!(f, "inline"),
            Display::Flex => write!(f, "flex"),
            Display::InlineFlex => write!(f, "inline-flex"),
            Display::Grid => write!(f, "grid"),
            Display::InlineGrid => write!(f, "inline-grid"),
            Display::Table => write!(f, "table"),
            Display::InlineTable => write!(f, "inline-table"),
            Display::TableCell => write!(f, "table-cell"),
            Display::TableRow => write!(f, "table-row"),
            Display::TableColumn => write!(f, "table-column"),
            Display::TableColumnGroup => write!(f, "table-column-group"),
            Display::TableFooterGroup => write!(f, "table-footer-group"),
            Display::TableHeaderGroup => write!(f, "table-header-group"),
            Display::TableRowGroup => write!(f, "table-row-group"),
            Display::FlowRoot => write!(f, "flow-root"),
            Display::Contents => write!(f, "contents"),
            Display::ListItem => write!(f, "list-item"),
            Display::Hidden => write!(f, "hidden"),
        }
    }
}

/// Trait for adding display utilities to a class builder
pub trait DisplayUtilities {
    fn display(self, display: Display) -> Self;
}

impl DisplayUtilities for ClassBuilder {
    fn display(self, display: Display) -> Self {
        self.class(display.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_utilities() {
        let classes = ClassBuilder::new().display(Display::Flex).build();

        assert!(classes.to_css_classes().contains("flex"));
    }
}
