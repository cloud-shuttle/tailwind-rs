use tailwind_rs_core::utilities::mask::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod mask_utilities_unit_tests {
    use super::*;

    #[test]
    fn test_mask_type_none() {
        let mask_type = MaskType::None;
        assert_eq!(mask_type.to_string(), "none");
    }

    #[test]
    fn test_mask_type_alpha() {
        let mask_type = MaskType::Alpha;
        assert_eq!(mask_type.to_string(), "alpha");
    }

    #[test]
    fn test_mask_type_luminance() {
        let mask_type = MaskType::Luminance;
        assert_eq!(mask_type.to_string(), "luminance");
    }

    #[test]
    fn test_mask_mode_alpha() {
        let mask_mode = MaskMode::Alpha;
        assert_eq!(mask_mode.to_string(), "alpha");
    }

    #[test]
    fn test_mask_mode_luminance() {
        let mask_mode = MaskMode::Luminance;
        assert_eq!(mask_mode.to_string(), "luminance");
    }

    #[test]
    fn test_mask_mode_match_source() {
        let mask_mode = MaskMode::MatchSource;
        assert_eq!(mask_mode.to_string(), "match-source");
    }

    #[test]
    fn test_mask_composite_add() {
        let mask_composite = MaskComposite::Add;
        assert_eq!(mask_composite.to_string(), "add");
    }

    #[test]
    fn test_mask_composite_subtract() {
        let mask_composite = MaskComposite::Subtract;
        assert_eq!(mask_composite.to_string(), "subtract");
    }

    #[test]
    fn test_mask_composite_intersect() {
        let mask_composite = MaskComposite::Intersect;
        assert_eq!(mask_composite.to_string(), "intersect");
    }

    #[test]
    fn test_mask_composite_exclude() {
        let mask_composite = MaskComposite::Exclude;
        assert_eq!(mask_composite.to_string(), "exclude");
    }

    #[test]
    fn test_mask_repeat_no_repeat() {
        let mask_repeat = MaskRepeat::NoRepeat;
        assert_eq!(mask_repeat.to_string(), "no-repeat");
    }

    #[test]
    fn test_mask_repeat_repeat() {
        let mask_repeat = MaskRepeat::Repeat;
        assert_eq!(mask_repeat.to_string(), "repeat");
    }

    #[test]
    fn test_mask_repeat_repeat_x() {
        let mask_repeat = MaskRepeat::RepeatX;
        assert_eq!(mask_repeat.to_string(), "repeat-x");
    }

    #[test]
    fn test_mask_repeat_repeat_y() {
        let mask_repeat = MaskRepeat::RepeatY;
        assert_eq!(mask_repeat.to_string(), "repeat-y");
    }

    #[test]
    fn test_mask_repeat_round() {
        let mask_repeat = MaskRepeat::Round;
        assert_eq!(mask_repeat.to_string(), "round");
    }

    #[test]
    fn test_mask_repeat_space() {
        let mask_repeat = MaskRepeat::Space;
        assert_eq!(mask_repeat.to_string(), "space");
    }

    #[test]
    fn test_mask_size_auto() {
        let mask_size = MaskSize::Auto;
        assert_eq!(mask_size.to_string(), "auto");
    }

    #[test]
    fn test_mask_size_cover() {
        let mask_size = MaskSize::Cover;
        assert_eq!(mask_size.to_string(), "cover");
    }

    #[test]
    fn test_mask_size_contain() {
        let mask_size = MaskSize::Contain;
        assert_eq!(mask_size.to_string(), "contain");
    }

    #[test]
    fn test_mask_position_center() {
        let mask_position = MaskPosition::Center;
        assert_eq!(mask_position.to_string(), "center");
    }

    #[test]
    fn test_mask_position_top() {
        let mask_position = MaskPosition::Top;
        assert_eq!(mask_position.to_string(), "top");
    }

    #[test]
    fn test_mask_position_bottom() {
        let mask_position = MaskPosition::Bottom;
        assert_eq!(mask_position.to_string(), "bottom");
    }

    #[test]
    fn test_mask_position_left() {
        let mask_position = MaskPosition::Left;
        assert_eq!(mask_position.to_string(), "left");
    }

    #[test]
    fn test_mask_position_right() {
        let mask_position = MaskPosition::Right;
        assert_eq!(mask_position.to_string(), "right");
    }

    #[test]
    fn test_mask_position_top_left() {
        let mask_position = MaskPosition::TopLeft;
        assert_eq!(mask_position.to_string(), "top left");
    }

    #[test]
    fn test_mask_position_top_right() {
        let mask_position = MaskPosition::TopRight;
        assert_eq!(mask_position.to_string(), "top right");
    }

    #[test]
    fn test_mask_position_bottom_left() {
        let mask_position = MaskPosition::BottomLeft;
        assert_eq!(mask_position.to_string(), "bottom left");
    }

    #[test]
    fn test_mask_position_bottom_right() {
        let mask_position = MaskPosition::BottomRight;
        assert_eq!(mask_position.to_string(), "bottom right");
    }

    #[test]
    fn test_mask_clip_border_box() {
        let mask_clip = MaskClip::BorderBox;
        assert_eq!(mask_clip.to_string(), "border-box");
    }

    #[test]
    fn test_mask_clip_padding_box() {
        let mask_clip = MaskClip::PaddingBox;
        assert_eq!(mask_clip.to_string(), "padding-box");
    }

    #[test]
    fn test_mask_clip_content_box() {
        let mask_clip = MaskClip::ContentBox;
        assert_eq!(mask_clip.to_string(), "content-box");
    }

    #[test]
    fn test_mask_clip_text() {
        let mask_clip = MaskClip::Text;
        assert_eq!(mask_clip.to_string(), "text");
    }

    #[test]
    fn test_mask_origin_border_box() {
        let mask_origin = MaskOrigin::BorderBox;
        assert_eq!(mask_origin.to_string(), "border-box");
    }

    #[test]
    fn test_mask_origin_padding_box() {
        let mask_origin = MaskOrigin::PaddingBox;
        assert_eq!(mask_origin.to_string(), "padding-box");
    }

    #[test]
    fn test_mask_origin_content_box() {
        let mask_origin = MaskOrigin::ContentBox;
        assert_eq!(mask_origin.to_string(), "content-box");
    }

    #[test]
    fn test_mask_serialization() {
        let mask_type = MaskType::Alpha;
        let serialized = serde_json::to_string(&mask_type).unwrap();
        let deserialized: MaskType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mask_type, deserialized);
    }

    #[test]
    fn test_mask_clone() {
        let mask_type = MaskType::Luminance;
        let cloned = mask_type.clone();
        assert_eq!(mask_type, cloned);
    }

    #[test]
    fn test_mask_partial_eq() {
        let mask1 = MaskType::Alpha;
        let mask2 = MaskType::Alpha;
        let mask3 = MaskType::Luminance;

        assert_eq!(mask1, mask2);
        assert_ne!(mask1, mask3);
    }
}

#[cfg(test)]
mod mask_utilities_integration_tests {
    use super::*;

    #[test]
    fn test_mask_utilities_with_class_builder() {
        let builder = ClassBuilder::new().mask_alpha();

        let class_set = builder.build();
        assert!(class_set.classes.contains("mask-alpha"));
    }

    #[test]
    fn test_mask_utilities_with_other_utilities() {
        let builder = ClassBuilder::new()
            .mask_alpha()
            .class("text-blue-500")
            .class("font-bold");

        let class_set = builder.build();
        assert!(class_set.classes.contains("mask-alpha"));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_mask_utilities_responsive() {
        let builder = ClassBuilder::new()
            .mask_alpha()
            .responsive(Breakpoint::Md, "mask-luminance");

        let class_set = builder.build();
        assert!(class_set.classes.contains("mask-alpha"));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set
            .responsive
            .get(&Breakpoint::Md)
            .unwrap()
            .contains("mask-luminance"));
    }

    #[test]
    fn test_mask_utilities_conditional() {
        let builder = ClassBuilder::new()
            .mask_alpha()
            .conditional("hover", "mask-luminance");

        let class_set = builder.build();
        assert!(class_set.classes.contains("mask-alpha"));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set
            .conditional
            .get("hover")
            .unwrap()
            .contains("mask-luminance"));
    }

    #[test]
    fn test_mask_utilities_custom_variant() {
        let builder = ClassBuilder::new()
            .mask_alpha()
            .custom_variant("dark", "mask-luminance");

        let class_set = builder.build();
        assert!(class_set.classes.contains("mask-alpha"));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set
            .conditional
            .get("dark")
            .unwrap()
            .contains("mask-luminance"));
    }

    #[test]
    fn test_mask_utilities_multiple_masks() {
        let builder = ClassBuilder::new().mask_alpha().mask_luminance();

        let class_set = builder.build();
        assert!(class_set.classes.contains("mask-alpha"));
        assert!(class_set.classes.contains("mask-luminance"));
    }

    #[test]
    fn test_mask_utilities_build_string() {
        let classes = ClassBuilder::new()
            .mask_alpha()
            .class("text-blue-500")
            .build_string();

        assert!(classes.contains("mask-alpha"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_mask_utilities_css_classes() {
        let class_set = ClassBuilder::new()
            .mask_luminance()
            .class("font-bold")
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("mask-luminance"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_mask_utilities_comprehensive_usage() {
        let class_set = ClassBuilder::new()
            .mask_alpha()
            .mask_repeat_round()
            .mask_size_cover()
            .mask_center()
            .mask_clip_border()
            .mask_origin_padding()
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("mask-alpha"));
        assert!(css_classes.contains("mask-repeat-round"));
        assert!(css_classes.contains("mask-size-cover"));
        assert!(css_classes.contains("mask-center"));
        assert!(css_classes.contains("mask-clip-border"));
        assert!(css_classes.contains("mask-origin-padding"));
    }

    #[test]
    fn test_mask_utilities_all_variants() {
        let class_set = ClassBuilder::new()
            .mask_none()
            .mask_alpha()
            .mask_luminance()
            .mask_repeat_none()
            .mask_repeat()
            .mask_repeat_x()
            .mask_repeat_y()
            .mask_repeat_round()
            .mask_repeat_space()
            .mask_size_auto()
            .mask_size_cover()
            .mask_size_contain()
            .mask_center()
            .mask_top()
            .mask_bottom()
            .mask_left()
            .mask_right()
            .mask_top_left()
            .mask_top_right()
            .mask_bottom_left()
            .mask_bottom_right()
            .mask_clip_border()
            .mask_clip_padding()
            .mask_clip_content()
            .mask_clip_text()
            .mask_origin_border()
            .mask_origin_padding()
            .mask_origin_content()
            .build();

        let css_classes = class_set.to_css_classes();

        // Test that all mask utilities are present
        assert!(css_classes.contains("mask-none"));
        assert!(css_classes.contains("mask-alpha"));
        assert!(css_classes.contains("mask-luminance"));
        assert!(css_classes.contains("mask-repeat-none"));
        assert!(css_classes.contains("mask-repeat"));
        assert!(css_classes.contains("mask-repeat-x"));
        assert!(css_classes.contains("mask-repeat-y"));
        assert!(css_classes.contains("mask-repeat-round"));
        assert!(css_classes.contains("mask-repeat-space"));
        assert!(css_classes.contains("mask-size-auto"));
        assert!(css_classes.contains("mask-size-cover"));
        assert!(css_classes.contains("mask-size-contain"));
        assert!(css_classes.contains("mask-center"));
        assert!(css_classes.contains("mask-top"));
        assert!(css_classes.contains("mask-bottom"));
        assert!(css_classes.contains("mask-left"));
        assert!(css_classes.contains("mask-right"));
        assert!(css_classes.contains("mask-top-left"));
        assert!(css_classes.contains("mask-top-right"));
        assert!(css_classes.contains("mask-bottom-left"));
        assert!(css_classes.contains("mask-bottom-right"));
        assert!(css_classes.contains("mask-clip-border"));
        assert!(css_classes.contains("mask-clip-padding"));
        assert!(css_classes.contains("mask-clip-content"));
        assert!(css_classes.contains("mask-clip-text"));
        assert!(css_classes.contains("mask-origin-border"));
        assert!(css_classes.contains("mask-origin-padding"));
        assert!(css_classes.contains("mask-origin-content"));
    }
}
