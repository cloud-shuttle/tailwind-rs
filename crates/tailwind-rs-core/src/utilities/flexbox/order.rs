//! Order utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Order values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Order {
    /// Order 1
    Order1,
    /// Order 2
    Order2,
    /// Order 3
    Order3,
    /// Order 4
    Order4,
    /// Order 5
    Order5,
    /// Order 6
    Order6,
    /// Order 7
    Order7,
    /// Order 8
    Order8,
    /// Order 9
    Order9,
    /// Order 10
    Order10,
    /// Order 11
    Order11,
    /// Order 12
    Order12,
    /// Order first
    OrderFirst,
    /// Order last
    OrderLast,
    /// Order none
    OrderNone,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Order::Order1 => write!(f, "order-1"),
            Order::Order2 => write!(f, "order-2"),
            Order::Order3 => write!(f, "order-3"),
            Order::Order4 => write!(f, "order-4"),
            Order::Order5 => write!(f, "order-5"),
            Order::Order6 => write!(f, "order-6"),
            Order::Order7 => write!(f, "order-7"),
            Order::Order8 => write!(f, "order-8"),
            Order::Order9 => write!(f, "order-9"),
            Order::Order10 => write!(f, "order-10"),
            Order::Order11 => write!(f, "order-11"),
            Order::Order12 => write!(f, "order-12"),
            Order::OrderFirst => write!(f, "order-first"),
            Order::OrderLast => write!(f, "order-last"),
            Order::OrderNone => write!(f, "order-none"),
        }
    }
}

/// Trait for adding order utilities to a class builder
pub trait OrderUtilities {
    fn order(self, order: Order) -> Self;
}

impl OrderUtilities for ClassBuilder {
    fn order(self, order: Order) -> Self {
        self.class(order.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_utilities() {
        let classes = ClassBuilder::new().order(Order::Order1).build();

        assert!(classes.to_css_classes().contains("order-1"));
    }
}
