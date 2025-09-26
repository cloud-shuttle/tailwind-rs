use tailwind_rs_core::utilities::mask::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod mask_utilities_baseline_tests {
    use super::*;

    #[test]
    fn test_mask_utilities_css_output_baseline() {
        let builder = ClassBuilder::new().mask_alpha().mask_luminance();

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain both mask classes
        assert!(classes.contains("mask-alpha"));
        assert!(classes.contains("mask-luminance"));
    }

    #[test]
    fn test_mask_utilities_class_generation_baseline() {
        let builder = ClassBuilder::new().mask_alpha().mask_repeat_round();

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain both mask classes
        assert!(classes.contains("mask-alpha"));
        assert!(classes.contains("mask-repeat-round"));
    }

    #[test]
    fn test_mask_type_alpha_baseline() {
        let mask_type = MaskType::Alpha;
        let string_value = mask_type.to_string();

        // Baseline string output
        assert_eq!(string_value, "alpha");
    }

    #[test]
    fn test_mask_type_luminance_baseline() {
        let mask_type = MaskType::Luminance;
        let string_value = mask_type.to_string();

        // Baseline string output
        assert_eq!(string_value, "luminance");
    }

    #[test]
    fn test_mask_type_none_baseline() {
        let mask_type = MaskType::None;
        let string_value = mask_type.to_string();

        // Baseline string output
        assert_eq!(string_value, "none");
    }

    #[test]
    fn test_mask_mode_alpha_baseline() {
        let mask_mode = MaskMode::Alpha;
        let string_value = mask_mode.to_string();

        // Baseline string output
        assert_eq!(string_value, "alpha");
    }

    #[test]
    fn test_mask_mode_luminance_baseline() {
        let mask_mode = MaskMode::Luminance;
        let string_value = mask_mode.to_string();

        // Baseline string output
        assert_eq!(string_value, "luminance");
    }

    #[test]
    fn test_mask_mode_match_source_baseline() {
        let mask_mode = MaskMode::MatchSource;
        let string_value = mask_mode.to_string();

        // Baseline string output
        assert_eq!(string_value, "match-source");
    }

    #[test]
    fn test_mask_composite_add_baseline() {
        let mask_composite = MaskComposite::Add;
        let string_value = mask_composite.to_string();

        // Baseline string output
        assert_eq!(string_value, "add");
    }

    #[test]
    fn test_mask_composite_subtract_baseline() {
        let mask_composite = MaskComposite::Subtract;
        let string_value = mask_composite.to_string();

        // Baseline string output
        assert_eq!(string_value, "subtract");
    }

    #[test]
    fn test_mask_composite_intersect_baseline() {
        let mask_composite = MaskComposite::Intersect;
        let string_value = mask_composite.to_string();

        // Baseline string output
        assert_eq!(string_value, "intersect");
    }

    #[test]
    fn test_mask_composite_exclude_baseline() {
        let mask_composite = MaskComposite::Exclude;
        let string_value = mask_composite.to_string();

        // Baseline string output
        assert_eq!(string_value, "exclude");
    }

    #[test]
    fn test_mask_repeat_no_repeat_baseline() {
        let mask_repeat = MaskRepeat::NoRepeat;
        let string_value = mask_repeat.to_string();

        // Baseline string output
        assert_eq!(string_value, "no-repeat");
    }

    #[test]
    fn test_mask_repeat_repeat_baseline() {
        let mask_repeat = MaskRepeat::Repeat;
        let string_value = mask_repeat.to_string();

        // Baseline string output
        assert_eq!(string_value, "repeat");
    }

    #[test]
    fn test_mask_repeat_repeat_x_baseline() {
        let mask_repeat = MaskRepeat::RepeatX;
        let string_value = mask_repeat.to_string();

        // Baseline string output
        assert_eq!(string_value, "repeat-x");
    }

    #[test]
    fn test_mask_repeat_repeat_y_baseline() {
        let mask_repeat = MaskRepeat::RepeatY;
        let string_value = mask_repeat.to_string();

        // Baseline string output
        assert_eq!(string_value, "repeat-y");
    }

    #[test]
    fn test_mask_repeat_round_baseline() {
        let mask_repeat = MaskRepeat::Round;
        let string_value = mask_repeat.to_string();

        // Baseline string output
        assert_eq!(string_value, "round");
    }

    #[test]
    fn test_mask_repeat_space_baseline() {
        let mask_repeat = MaskRepeat::Space;
        let string_value = mask_repeat.to_string();

        // Baseline string output
        assert_eq!(string_value, "space");
    }

    #[test]
    fn test_mask_size_auto_baseline() {
        let mask_size = MaskSize::Auto;
        let string_value = mask_size.to_string();

        // Baseline string output
        assert_eq!(string_value, "auto");
    }

    #[test]
    fn test_mask_size_cover_baseline() {
        let mask_size = MaskSize::Cover;
        let string_value = mask_size.to_string();

        // Baseline string output
        assert_eq!(string_value, "cover");
    }

    #[test]
    fn test_mask_size_contain_baseline() {
        let mask_size = MaskSize::Contain;
        let string_value = mask_size.to_string();

        // Baseline string output
        assert_eq!(string_value, "contain");
    }

    #[test]
    fn test_mask_position_center_baseline() {
        let mask_position = MaskPosition::Center;
        let string_value = mask_position.to_string();

        // Baseline string output
        assert_eq!(string_value, "center");
    }

    #[test]
    fn test_mask_position_top_baseline() {
        let mask_position = MaskPosition::Top;
        let string_value = mask_position.to_string();

        // Baseline string output
        assert_eq!(string_value, "top");
    }

    #[test]
    fn test_mask_position_bottom_baseline() {
        let mask_position = MaskPosition::Bottom;
        let string_value = mask_position.to_string();

        // Baseline string output
        assert_eq!(string_value, "bottom");
    }

    #[test]
    fn test_mask_position_left_baseline() {
        let mask_position = MaskPosition::Left;
        let string_value = mask_position.to_string();

        // Baseline string output
        assert_eq!(string_value, "left");
    }

    #[test]
    fn test_mask_position_right_baseline() {
        let mask_position = MaskPosition::Right;
        let string_value = mask_position.to_string();

        // Baseline string output
        assert_eq!(string_value, "right");
    }

    #[test]
    fn test_mask_position_top_left_baseline() {
        let mask_position = MaskPosition::TopLeft;
        let string_value = mask_position.to_string();

        // Baseline string output
        assert_eq!(string_value, "top left");
    }

    #[test]
    fn test_mask_position_top_right_baseline() {
        let mask_position = MaskPosition::TopRight;
        let string_value = mask_position.to_string();

        // Baseline string output
        assert_eq!(string_value, "top right");
    }

    #[test]
    fn test_mask_position_bottom_left_baseline() {
        let mask_position = MaskPosition::BottomLeft;
        let string_value = mask_position.to_string();

        // Baseline string output
        assert_eq!(string_value, "bottom left");
    }

    #[test]
    fn test_mask_position_bottom_right_baseline() {
        let mask_position = MaskPosition::BottomRight;
        let string_value = mask_position.to_string();

        // Baseline string output
        assert_eq!(string_value, "bottom right");
    }

    #[test]
    fn test_mask_clip_border_box_baseline() {
        let mask_clip = MaskClip::BorderBox;
        let string_value = mask_clip.to_string();

        // Baseline string output
        assert_eq!(string_value, "border-box");
    }

    #[test]
    fn test_mask_clip_padding_box_baseline() {
        let mask_clip = MaskClip::PaddingBox;
        let string_value = mask_clip.to_string();

        // Baseline string output
        assert_eq!(string_value, "padding-box");
    }

    #[test]
    fn test_mask_clip_content_box_baseline() {
        let mask_clip = MaskClip::ContentBox;
        let string_value = mask_clip.to_string();

        // Baseline string output
        assert_eq!(string_value, "content-box");
    }

    #[test]
    fn test_mask_clip_text_baseline() {
        let mask_clip = MaskClip::Text;
        let string_value = mask_clip.to_string();

        // Baseline string output
        assert_eq!(string_value, "text");
    }

    #[test]
    fn test_mask_origin_border_box_baseline() {
        let mask_origin = MaskOrigin::BorderBox;
        let string_value = mask_origin.to_string();

        // Baseline string output
        assert_eq!(string_value, "border-box");
    }

    #[test]
    fn test_mask_origin_padding_box_baseline() {
        let mask_origin = MaskOrigin::PaddingBox;
        let string_value = mask_origin.to_string();

        // Baseline string output
        assert_eq!(string_value, "padding-box");
    }

    #[test]
    fn test_mask_origin_content_box_baseline() {
        let mask_origin = MaskOrigin::ContentBox;
        let string_value = mask_origin.to_string();

        // Baseline string output
        assert_eq!(string_value, "content-box");
    }

    #[test]
    fn test_mask_utilities_serialization_baseline() {
        let mask_type = MaskType::Alpha;
        let serialized = serde_json::to_string(&mask_type).unwrap();

        // Baseline: Should serialize to JSON
        assert!(serialized.contains("Alpha"));

        // Should deserialize back to the same value
        let deserialized: MaskType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mask_type, deserialized);
    }

    #[test]
    fn test_mask_utilities_equality_baseline() {
        let mask1 = MaskType::Alpha;
        let mask2 = MaskType::Alpha;
        let mask3 = MaskType::Luminance;

        // Baseline: Same variants should be equal
        assert_eq!(mask1, mask2);
        assert_ne!(mask1, mask3);
    }

    #[test]
    fn test_mask_utilities_clone_baseline() {
        let mask_type = MaskType::Luminance;
        let cloned = mask_type.clone();

        // Baseline: Cloned mask should be equal to original
        assert_eq!(mask_type, cloned);
    }

    #[test]
    fn test_mask_utilities_complex_builder_baseline() {
        let class_set = ClassBuilder::new()
            .mask_alpha()
            .mask_repeat_round()
            .mask_size_cover()
            .mask_center()
            .mask_clip_border()
            .mask_origin_padding()
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "mask-luminance")
            .conditional("hover", "mask-none")
            .build();

        let classes = class_set.to_css_classes();

        // Baseline: Should contain expected classes
        assert!(classes.contains("mask-alpha"));
        assert!(classes.contains("mask-repeat-round"));
        assert!(classes.contains("mask-size-cover"));
        assert!(classes.contains("mask-center"));
        assert!(classes.contains("mask-clip-border"));
        assert!(classes.contains("mask-origin-padding"));
        assert!(classes.contains("text-blue-500"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("md:mask-luminance"));
        assert!(classes.contains("hover:mask-none"));
    }
}
