//! Background Display Traits Module
//!
//! Provides Display trait implementations for all background utility enums.

use std::fmt;
use super::{
    attachment::BackgroundAttachment,
    clip::BackgroundClip,
    origin::BackgroundOrigin,
    position::BackgroundPosition,
    repeat::BackgroundRepeat,
    size::BackgroundSize,
    image::BackgroundImage,
    gradient::{GradientDirection, GradientStop},
};

impl fmt::Display for BackgroundAttachment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundClip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundRepeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GradientDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GradientStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::attachment::BackgroundAttachment;
    use super::super::clip::BackgroundClip;
    use super::super::position::BackgroundPosition;
    use super::super::gradient::{GradientDirection, GradientStop};

    #[test]
    fn attachment_display() {
        let attachment = BackgroundAttachment::Fixed;
        assert_eq!(format!("{}", attachment), "fixed");
    }

    #[test]
    fn clip_display() {
        let clip = BackgroundClip::Border;
        assert_eq!(format!("{}", clip), "border");
    }

    #[test]
    fn position_display() {
        let position = BackgroundPosition::Center;
        assert_eq!(format!("{}", position), "center");

        let position = BackgroundPosition::LeftBottom;
        assert_eq!(format!("{}", position), "left-bottom");
    }

    #[test]
    fn gradient_direction_display() {
        let direction = GradientDirection::ToRight;
        assert_eq!(format!("{}", direction), "to-r");

        let direction = GradientDirection::ToTopLeft;
        assert_eq!(format!("{}", direction), "to-tl");
    }

    #[test]
    fn gradient_stop_display() {
        let stop = GradientStop::From;
        assert_eq!(format!("{}", stop), "from");

        let stop = GradientStop::Via;
        assert_eq!(format!("{}", stop), "via");

        let stop = GradientStop::To;
        assert_eq!(format!("{}", stop), "to");
    }
}
