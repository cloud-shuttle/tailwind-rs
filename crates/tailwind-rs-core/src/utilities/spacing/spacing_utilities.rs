//! Spacing utility traits and implementations
//!
//! This module contains all the utility traits for padding, margin, gap, and space-between utilities.

use crate::classes::ClassBuilder;
use super::spacing_values::SpacingValue;

/// Trait for adding padding utilities to a class builder
pub trait PaddingUtilities {
    /// Add padding to all sides
    fn padding(self, value: SpacingValue) -> Self;

    /// Add horizontal padding (left and right)
    fn padding_x(self, value: SpacingValue) -> Self;

    /// Add vertical padding (top and bottom)
    fn padding_y(self, value: SpacingValue) -> Self;

    /// Add top padding
    fn padding_top(self, value: SpacingValue) -> Self;

    /// Add right padding
    fn padding_right(self, value: SpacingValue) -> Self;

    /// Add bottom padding
    fn padding_bottom(self, value: SpacingValue) -> Self;

    /// Add left padding
    fn padding_left(self, value: SpacingValue) -> Self;

    /// Add padding to start (left in LTR, right in RTL)
    fn padding_start(self, value: SpacingValue) -> Self;

    /// Add padding to end (right in LTR, left in RTL)
    fn padding_end(self, value: SpacingValue) -> Self;
}

impl PaddingUtilities for ClassBuilder {
    fn padding(self, value: SpacingValue) -> Self {
        self.class(format!("p-{}", value.to_class_name()))
    }

    fn padding_x(self, value: SpacingValue) -> Self {
        self.class(format!("px-{}", value.to_class_name()))
    }

    fn padding_y(self, value: SpacingValue) -> Self {
        self.class(format!("py-{}", value.to_class_name()))
    }

    fn padding_top(self, value: SpacingValue) -> Self {
        self.class(format!("pt-{}", value.to_class_name()))
    }

    fn padding_right(self, value: SpacingValue) -> Self {
        self.class(format!("pr-{}", value.to_class_name()))
    }

    fn padding_bottom(self, value: SpacingValue) -> Self {
        self.class(format!("pb-{}", value.to_class_name()))
    }

    fn padding_left(self, value: SpacingValue) -> Self {
        self.class(format!("pl-{}", value.to_class_name()))
    }

    fn padding_start(self, value: SpacingValue) -> Self {
        self.class(format!("ps-{}", value.to_class_name()))
    }

    fn padding_end(self, value: SpacingValue) -> Self {
        self.class(format!("pe-{}", value.to_class_name()))
    }
}

/// Trait for adding margin utilities to a class builder
pub trait MarginUtilities {
    /// Add margin to all sides
    fn margin(self, value: SpacingValue) -> Self;

    /// Add horizontal margin (left and right)
    fn margin_x(self, value: SpacingValue) -> Self;

    /// Add vertical margin (top and bottom)
    fn margin_y(self, value: SpacingValue) -> Self;

    /// Add top margin
    fn margin_top(self, value: SpacingValue) -> Self;

    /// Add right margin
    fn margin_right(self, value: SpacingValue) -> Self;

    /// Add bottom margin
    fn margin_bottom(self, value: SpacingValue) -> Self;

    /// Add left margin
    fn margin_left(self, value: SpacingValue) -> Self;

    /// Add margin to start (left in LTR, right in RTL)
    fn margin_start(self, value: SpacingValue) -> Self;

    /// Add margin to end (right in LTR, left in RTL)
    fn margin_end(self, value: SpacingValue) -> Self;

    /// Add negative margin to all sides
    fn margin_negative(self, value: SpacingValue) -> Self;

    /// Add negative horizontal margin
    fn margin_x_negative(self, value: SpacingValue) -> Self;

    /// Add negative vertical margin
    fn margin_y_negative(self, value: SpacingValue) -> Self;

    /// Add negative top margin
    fn margin_top_negative(self, value: SpacingValue) -> Self;

    /// Add negative right margin
    fn margin_right_negative(self, value: SpacingValue) -> Self;

    /// Add negative bottom margin
    fn margin_bottom_negative(self, value: SpacingValue) -> Self;

    /// Add negative left margin
    fn margin_left_negative(self, value: SpacingValue) -> Self;
}

impl MarginUtilities for ClassBuilder {
    fn margin(self, value: SpacingValue) -> Self {
        self.class(format!("m-{}", value.to_class_name()))
    }

    fn margin_x(self, value: SpacingValue) -> Self {
        self.class(format!("mx-{}", value.to_class_name()))
    }

    fn margin_y(self, value: SpacingValue) -> Self {
        self.class(format!("my-{}", value.to_class_name()))
    }

    fn margin_top(self, value: SpacingValue) -> Self {
        self.class(format!("mt-{}", value.to_class_name()))
    }

    fn margin_right(self, value: SpacingValue) -> Self {
        self.class(format!("mr-{}", value.to_class_name()))
    }

    fn margin_bottom(self, value: SpacingValue) -> Self {
        self.class(format!("mb-{}", value.to_class_name()))
    }

    fn margin_left(self, value: SpacingValue) -> Self {
        self.class(format!("ml-{}", value.to_class_name()))
    }

    fn margin_start(self, value: SpacingValue) -> Self {
        self.class(format!("ms-{}", value.to_class_name()))
    }

    fn margin_end(self, value: SpacingValue) -> Self {
        self.class(format!("me-{}", value.to_class_name()))
    }

    fn margin_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-m-{}", value.to_class_name()))
    }

    fn margin_x_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-mx-{}", value.to_class_name()))
    }

    fn margin_y_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-my-{}", value.to_class_name()))
    }

    fn margin_top_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-mt-{}", value.to_class_name()))
    }

    fn margin_right_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-mr-{}", value.to_class_name()))
    }

    fn margin_bottom_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-mb-{}", value.to_class_name()))
    }

    fn margin_left_negative(self, value: SpacingValue) -> Self {
        self.class(format!("-ml-{}", value.to_class_name()))
    }
}

/// Trait for adding gap utilities to a class builder
pub trait GapUtilities {
    /// Add gap between grid/flex items
    fn gap(self, value: SpacingValue) -> Self;

    /// Add horizontal gap between grid/flex items
    fn gap_x(self, value: SpacingValue) -> Self;

    /// Add vertical gap between grid/flex items
    fn gap_y(self, value: SpacingValue) -> Self;
}

impl GapUtilities for ClassBuilder {
    fn gap(self, value: SpacingValue) -> Self {
        self.class(format!("gap-{}", value.to_class_name()))
    }

    fn gap_x(self, value: SpacingValue) -> Self {
        self.class(format!("gap-x-{}", value.to_class_name()))
    }

    fn gap_y(self, value: SpacingValue) -> Self {
        self.class(format!("gap-y-{}", value.to_class_name()))
    }
}

/// Trait for adding space-between utilities to a class builder
pub trait SpaceBetweenUtilities {
    /// Add horizontal space between child elements
    fn space_x(self, value: SpacingValue) -> Self;

    /// Add vertical space between child elements
    fn space_y(self, value: SpacingValue) -> Self;

    /// Reverse horizontal space between child elements
    fn space_x_reverse(self) -> Self;

    /// Reverse vertical space between child elements
    fn space_y_reverse(self) -> Self;
}

impl SpaceBetweenUtilities for ClassBuilder {
    fn space_x(self, value: SpacingValue) -> Self {
        self.class(format!("space-x-{}", value.to_class_name()))
    }

    fn space_y(self, value: SpacingValue) -> Self {
        self.class(format!("space-y-{}", value.to_class_name()))
    }

    fn space_x_reverse(self) -> Self {
        self.class("space-x-reverse".to_string())
    }

    fn space_y_reverse(self) -> Self {
        self.class("space-y-reverse".to_string())
    }
}

/// Convenience methods for space-between utilities
impl ClassBuilder {
    /// Add horizontal space between child elements with value 2
    pub fn space_x_2(self) -> Self {
        self.space_x(SpacingValue::Integer(2))
    }

    /// Add horizontal space between child elements with value 4
    pub fn space_x_4(self) -> Self {
        self.space_x(SpacingValue::Integer(4))
    }

    /// Add vertical space between child elements with value 2
    pub fn space_y_2(self) -> Self {
        self.space_y(SpacingValue::Integer(2))
    }

    /// Add vertical space between child elements with value 4
    pub fn space_y_4(self) -> Self {
        self.space_y(SpacingValue::Integer(4))
    }
}

/// Trait for adding divide utilities to a class builder
pub trait SpacingDivideUtilities {
    /// Add horizontal divider between child elements
    fn divide_x(self, value: SpacingValue) -> Self;

    /// Add vertical divider between child elements
    fn divide_y(self, value: SpacingValue) -> Self;

    /// Reverse horizontal divider between child elements
    fn divide_x_reverse(self) -> Self;

    /// Reverse vertical divider between child elements
    fn divide_y_reverse(self) -> Self;
}

impl SpacingDivideUtilities for ClassBuilder {
    fn divide_x(self, value: SpacingValue) -> Self {
        self.class(format!("divide-x-{}", value.to_class_name()))
    }

    fn divide_y(self, value: SpacingValue) -> Self {
        self.class(format!("divide-y-{}", value.to_class_name()))
    }

    fn divide_x_reverse(self) -> Self {
        self.class("divide-x-reverse".to_string())
    }

    fn divide_y_reverse(self) -> Self {
        self.class("divide-y-reverse".to_string())
    }
}

/// Convenience methods for divide utilities
impl ClassBuilder {
    /// Add horizontal divider between child elements with value 2
    pub fn divide_x_2(self) -> Self {
        self.divide_x(SpacingValue::Integer(2))
    }

    /// Add horizontal divider between child elements with value 4
    pub fn divide_x_4(self) -> Self {
        self.divide_x(SpacingValue::Integer(4))
    }

    /// Add vertical divider between child elements with value 2
    pub fn divide_y_2(self) -> Self {
        self.divide_y(SpacingValue::Integer(2))
    }

    /// Add vertical divider between child elements with value 4
    pub fn divide_y_4(self) -> Self {
        self.divide_y(SpacingValue::Integer(4))
    }
}
