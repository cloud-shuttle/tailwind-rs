//! Grid auto flow utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Grid auto flow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridAutoFlow {
    /// Row auto flow
    Row,
    /// Column auto flow
    Column,
    /// Dense row auto flow
    Dense,
    /// Dense column auto flow
    DenseColumn,
}

impl fmt::Display for GridAutoFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridAutoFlow::Row => write!(f, "grid-flow-row"),
            GridAutoFlow::Column => write!(f, "grid-flow-col"),
            GridAutoFlow::Dense => write!(f, "grid-flow-dense"),
            GridAutoFlow::DenseColumn => write!(f, "grid-flow-col-dense"),
        }
    }
}

/// Trait for adding grid auto flow utilities to a class builder
pub trait GridAutoFlowUtilities {
    fn grid_auto_flow(self, flow: GridAutoFlow) -> Self;
}

impl GridAutoFlowUtilities for ClassBuilder {
    fn grid_auto_flow(self, flow: GridAutoFlow) -> Self {
        self.class(flow.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_auto_flow_display() {
        assert_eq!(GridAutoFlow::Row.to_string(), "grid-flow-row");
        assert_eq!(GridAutoFlow::Column.to_string(), "grid-flow-col");
        assert_eq!(GridAutoFlow::Dense.to_string(), "grid-flow-dense");
    }

    #[test]
    fn test_grid_auto_flow_utilities() {
        let classes = ClassBuilder::new()
            .grid_auto_flow(GridAutoFlow::Row)
            .build();

        assert!(classes.to_css_classes().contains("grid-flow-row"));
    }
}
