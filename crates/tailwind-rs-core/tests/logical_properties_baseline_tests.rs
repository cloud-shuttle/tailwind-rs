use tailwind_rs_core::utilities::logical_properties::*;
use tailwind_rs_core::utilities::spacing::SpacingValue;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod logical_properties_baseline_tests {
    use super::*;

    #[test]
    fn test_logical_properties_css_output_baseline() {
        let builder = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .margin_inline_end(SpacingValue::Integer(4));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain both logical property classes
        assert!(classes.contains("ms-4"));
        assert!(classes.contains("me-4"));
    }

    #[test]
    fn test_logical_properties_class_generation_baseline() {
        let builder = ClassBuilder::new()
            .padding_inline_start(SpacingValue::Integer(2))
            .padding_inline_end(SpacingValue::Integer(2));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain both logical property classes
        assert!(classes.contains("ps-2"));
        assert!(classes.contains("pe-2"));
    }

    #[test]
    fn test_logical_direction_inline_start_baseline() {
        let direction = LogicalDirection::InlineStart;
        let string_value = direction.to_string();

        // Baseline string output
        assert_eq!(string_value, "inline-start");
    }

    #[test]
    fn test_logical_direction_inline_end_baseline() {
        let direction = LogicalDirection::InlineEnd;
        let string_value = direction.to_string();

        // Baseline string output
        assert_eq!(string_value, "inline-end");
    }

    #[test]
    fn test_logical_direction_block_start_baseline() {
        let direction = LogicalDirection::BlockStart;
        let string_value = direction.to_string();

        // Baseline string output
        assert_eq!(string_value, "block-start");
    }

    #[test]
    fn test_logical_direction_block_end_baseline() {
        let direction = LogicalDirection::BlockEnd;
        let string_value = direction.to_string();

        // Baseline string output
        assert_eq!(string_value, "block-end");
    }

    #[test]
    fn test_logical_properties_margin_inline_start_baseline() {
        let builder = ClassBuilder::new().margin_inline_start(SpacingValue::Integer(4));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain margin inline start class
        assert!(classes.contains("ms-4"));
    }

    #[test]
    fn test_logical_properties_margin_inline_end_baseline() {
        let builder = ClassBuilder::new().margin_inline_end(SpacingValue::Integer(4));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain margin inline end class
        assert!(classes.contains("me-4"));
    }

    #[test]
    fn test_logical_properties_margin_block_start_baseline() {
        let builder = ClassBuilder::new().margin_block_start(SpacingValue::Integer(4));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain margin block start class
        assert!(classes.contains("mt-4"));
    }

    #[test]
    fn test_logical_properties_margin_block_end_baseline() {
        let builder = ClassBuilder::new().margin_block_end(SpacingValue::Integer(4));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain margin block end class
        assert!(classes.contains("mb-4"));
    }

    #[test]
    fn test_logical_properties_padding_inline_start_baseline() {
        let builder = ClassBuilder::new().padding_inline_start(SpacingValue::Integer(2));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain padding inline start class
        assert!(classes.contains("ps-2"));
    }

    #[test]
    fn test_logical_properties_padding_inline_end_baseline() {
        let builder = ClassBuilder::new().padding_inline_end(SpacingValue::Integer(2));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain padding inline end class
        assert!(classes.contains("pe-2"));
    }

    #[test]
    fn test_logical_properties_padding_block_start_baseline() {
        let builder = ClassBuilder::new().padding_block_start(SpacingValue::Integer(2));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain padding block start class
        assert!(classes.contains("pt-2"));
    }

    #[test]
    fn test_logical_properties_padding_block_end_baseline() {
        let builder = ClassBuilder::new().padding_block_end(SpacingValue::Integer(2));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain padding block end class
        assert!(classes.contains("pb-2"));
    }

    #[test]
    fn test_logical_properties_border_inline_start_baseline() {
        let builder = ClassBuilder::new().border_inline_start(SpacingValue::Integer(1));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain border inline start class
        assert!(classes.contains("border-s-1"));
    }

    #[test]
    fn test_logical_properties_border_inline_end_baseline() {
        let builder = ClassBuilder::new().border_inline_end(SpacingValue::Integer(1));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain border inline end class
        assert!(classes.contains("border-e-1"));
    }

    #[test]
    fn test_logical_properties_border_block_start_baseline() {
        let builder = ClassBuilder::new().border_block_start(SpacingValue::Integer(1));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain border block start class
        assert!(classes.contains("border-t-1"));
    }

    #[test]
    fn test_logical_properties_border_block_end_baseline() {
        let builder = ClassBuilder::new().border_block_end(SpacingValue::Integer(1));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain border block end class
        assert!(classes.contains("border-b-1"));
    }

    #[test]
    fn test_logical_properties_inset_inline_start_baseline() {
        let builder = ClassBuilder::new().inset_inline_start(SpacingValue::Integer(4));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain inset inline start class
        assert!(classes.contains("start-4"));
    }

    #[test]
    fn test_logical_properties_inset_inline_end_baseline() {
        let builder = ClassBuilder::new().inset_inline_end(SpacingValue::Integer(4));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain inset inline end class
        assert!(classes.contains("end-4"));
    }

    #[test]
    fn test_logical_properties_inset_block_start_baseline() {
        let builder = ClassBuilder::new().inset_block_start(SpacingValue::Integer(2));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain inset block start class
        assert!(classes.contains("top-2"));
    }

    #[test]
    fn test_logical_properties_inset_block_end_baseline() {
        let builder = ClassBuilder::new().inset_block_end(SpacingValue::Integer(2));

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain inset block end class
        assert!(classes.contains("bottom-2"));
    }

    #[test]
    fn test_logical_properties_serialization_baseline() {
        let direction = LogicalDirection::InlineStart;
        let serialized = serde_json::to_string(&direction).unwrap();

        // Baseline: Should serialize to JSON
        assert!(serialized.contains("InlineStart"));

        // Should deserialize back to the same value
        let deserialized: LogicalDirection = serde_json::from_str(&serialized).unwrap();
        assert_eq!(direction, deserialized);
    }

    #[test]
    fn test_logical_properties_equality_baseline() {
        let direction1 = LogicalDirection::InlineStart;
        let direction2 = LogicalDirection::InlineStart;
        let direction3 = LogicalDirection::InlineEnd;

        // Baseline: Same variants should be equal
        assert_eq!(direction1, direction2);
        assert_ne!(direction1, direction3);
    }

    #[test]
    fn test_logical_properties_clone_baseline() {
        let direction = LogicalDirection::InlineStart;
        let cloned = direction.clone();

        // Baseline: Cloned direction should be equal to original
        assert_eq!(direction, cloned);
    }

    #[test]
    fn test_logical_properties_complex_builder_baseline() {
        let class_set = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .margin_inline_end(SpacingValue::Integer(4))
            .padding_inline_start(SpacingValue::Integer(2))
            .padding_inline_end(SpacingValue::Integer(2))
            .border_inline_start(SpacingValue::Integer(1))
            .border_inline_end(SpacingValue::Integer(1))
            .inset_inline_start(SpacingValue::Integer(4))
            .inset_inline_end(SpacingValue::Integer(4))
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "ms-8")
            .conditional("hover", "me-8")
            .build();

        let classes = class_set.to_css_classes();

        // Baseline: Should contain expected classes
        assert!(classes.contains("ms-4"));
        assert!(classes.contains("me-4"));
        assert!(classes.contains("ps-2"));
        assert!(classes.contains("pe-2"));
        assert!(classes.contains("border-s-1"));
        assert!(classes.contains("border-e-1"));
        assert!(classes.contains("start-4"));
        assert!(classes.contains("end-4"));
        assert!(classes.contains("text-blue-500"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("md:ms-8"));
        assert!(classes.contains("hover:me-8"));
    }
}
