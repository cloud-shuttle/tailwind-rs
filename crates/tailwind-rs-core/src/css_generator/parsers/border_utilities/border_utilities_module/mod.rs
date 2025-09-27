//! Border Utilities Parser
//!
//! This module provides parsing logic for border-related Tailwind CSS utilities,
//! including border-radius, border-width, border-color, border-style, and border utilities.

use crate::css_generator::parsers::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

pub mod border_radius;
pub mod border_width;
pub mod border_color;
pub mod border_style;
pub mod border_utilities;
pub mod border_collapse;
pub mod border_separate;
pub mod border_spacing;
pub mod divide_width;
pub mod divide_color;
pub mod divide_style;
pub mod divide_opacity;
pub mod outline;
pub mod ring;
pub mod ring_width;
pub mod ring_color;
pub mod ring_opacity;
pub mod ring_offset;
pub mod ring_offset_width;
pub mod ring_offset_color;

pub use border_radius::BorderRadiusParser;
pub use border_width::BorderWidthParser;
pub use border_color::BorderColorParser;
pub use border_style::BorderStyleParser;
pub use border_utilities::BorderUtilitiesParser;
pub use border_collapse::BorderCollapseParser;
pub use border_separate::BorderSeparateParser;
pub use border_spacing::BorderSpacingParser;
pub use divide_width::DivideWidthParser;
pub use divide_color::DivideColorParser;
pub use divide_style::DivideStyleParser;
pub use divide_opacity::DivideOpacityParser;
pub use outline::OutlineParser;
pub use ring::RingParser;
pub use ring_width::RingWidthParser;
pub use ring_color::RingColorParser;
pub use ring_opacity::RingOpacityParser;
pub use ring_offset::RingOffsetParser;
pub use ring_offset_width::RingOffsetWidthParser;
pub use ring_offset_color::RingOffsetColorParser;
