use tailwind_rs_core::utilities::logical_properties::*;
use tailwind_rs_core::utilities::spacing::SpacingValue;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod logical_properties_unit_tests {
    use super::*;

    #[test]
    fn test_logical_direction_inline_start() {
        let direction = LogicalDirection::InlineStart;
        assert_eq!(direction.to_string(), "inline-start");
    }

    #[test]
    fn test_logical_direction_inline_end() {
        let direction = LogicalDirection::InlineEnd;
        assert_eq!(direction.to_string(), "inline-end");
    }

    #[test]
    fn test_logical_direction_block_start() {
        let direction = LogicalDirection::BlockStart;
        assert_eq!(direction.to_string(), "block-start");
    }

    #[test]
    fn test_logical_direction_block_end() {
        let direction = LogicalDirection::BlockEnd;
        assert_eq!(direction.to_string(), "block-end");
    }

    #[test]
    fn test_logical_direction_serialization() {
        let direction = LogicalDirection::InlineStart;
        let serialized = serde_json::to_string(&direction).unwrap();
        let deserialized: LogicalDirection = serde_json::from_str(&serialized).unwrap();
        assert_eq!(direction, deserialized);
    }

    #[test]
    fn test_logical_direction_clone() {
        let direction = LogicalDirection::InlineStart;
        let cloned = direction.clone();
        assert_eq!(direction, cloned);
    }

    #[test]
    fn test_logical_direction_partial_eq() {
        let direction1 = LogicalDirection::InlineStart;
        let direction2 = LogicalDirection::InlineStart;
        let direction3 = LogicalDirection::InlineEnd;

        assert_eq!(direction1, direction2);
        assert_ne!(direction1, direction3);
    }

    #[test]
    fn test_logical_direction_hash() {
        let direction1 = LogicalDirection::InlineStart;
        let direction2 = LogicalDirection::InlineStart;
        let direction3 = LogicalDirection::InlineEnd;

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        direction1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        direction2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        direction3.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_logical_direction_debug() {
        let direction = LogicalDirection::InlineStart;
        let debug = format!("{:?}", direction);
        assert!(debug.contains("InlineStart"));
    }

    #[test]
    fn test_logical_direction_copy() {
        let direction = LogicalDirection::InlineStart;
        let copied = direction;
        assert_eq!(direction, copied);
    }

    #[test]
    fn test_logical_direction_all_variants() {
        let directions = vec![
            LogicalDirection::InlineStart,
            LogicalDirection::InlineEnd,
            LogicalDirection::BlockStart,
            LogicalDirection::BlockEnd,
        ];

        let strings: Vec<String> = directions.iter().map(|d| d.to_string()).collect();
        assert!(strings.contains(&"inline-start".to_string()));
        assert!(strings.contains(&"inline-end".to_string()));
        assert!(strings.contains(&"block-start".to_string()));
        assert!(strings.contains(&"block-end".to_string()));
    }
}

#[cfg(test)]
mod logical_properties_integration_tests {
    use super::*;

    #[test]
    fn test_logical_properties_margin_inline_start() {
        let builder = ClassBuilder::new().margin_inline_start(SpacingValue::Integer(4));

        let class_set = builder.build();
        assert!(class_set.classes.contains("ms-4"));
    }

    #[test]
    fn test_logical_properties_margin_inline_end() {
        let builder = ClassBuilder::new().margin_inline_end(SpacingValue::Integer(4));

        let class_set = builder.build();
        assert!(class_set.classes.contains("me-4"));
    }

    #[test]
    fn test_logical_properties_margin_block_start() {
        let builder = ClassBuilder::new().margin_block_start(SpacingValue::Integer(4));

        let class_set = builder.build();
        assert!(class_set.classes.contains("mt-4"));
    }

    #[test]
    fn test_logical_properties_margin_block_end() {
        let builder = ClassBuilder::new().margin_block_end(SpacingValue::Integer(4));

        let class_set = builder.build();
        assert!(class_set.classes.contains("mb-4"));
    }

    #[test]
    fn test_logical_properties_padding_inline_start() {
        let builder = ClassBuilder::new().padding_inline_start(SpacingValue::Integer(2));

        let class_set = builder.build();
        assert!(class_set.classes.contains("ps-2"));
    }

    #[test]
    fn test_logical_properties_padding_inline_end() {
        let builder = ClassBuilder::new().padding_inline_end(SpacingValue::Integer(2));

        let class_set = builder.build();
        assert!(class_set.classes.contains("pe-2"));
    }

    #[test]
    fn test_logical_properties_padding_block_start() {
        let builder = ClassBuilder::new().padding_block_start(SpacingValue::Integer(2));

        let class_set = builder.build();
        assert!(class_set.classes.contains("pt-2"));
    }

    #[test]
    fn test_logical_properties_padding_block_end() {
        let builder = ClassBuilder::new().padding_block_end(SpacingValue::Integer(2));

        let class_set = builder.build();
        assert!(class_set.classes.contains("pb-2"));
    }

    #[test]
    fn test_logical_properties_border_inline_start() {
        let builder = ClassBuilder::new().border_inline_start(SpacingValue::Integer(1));

        let class_set = builder.build();
        assert!(class_set.classes.contains("border-s-1"));
    }

    #[test]
    fn test_logical_properties_border_inline_end() {
        let builder = ClassBuilder::new().border_inline_end(SpacingValue::Integer(1));

        let class_set = builder.build();
        assert!(class_set.classes.contains("border-e-1"));
    }

    #[test]
    fn test_logical_properties_border_block_start() {
        let builder = ClassBuilder::new().border_block_start(SpacingValue::Integer(1));

        let class_set = builder.build();
        assert!(class_set.classes.contains("border-t-1"));
    }

    #[test]
    fn test_logical_properties_border_block_end() {
        let builder = ClassBuilder::new().border_block_end(SpacingValue::Integer(1));

        let class_set = builder.build();
        assert!(class_set.classes.contains("border-b-1"));
    }

    #[test]
    fn test_logical_properties_inset_inline_start() {
        let builder = ClassBuilder::new().inset_inline_start(SpacingValue::Integer(4));

        let class_set = builder.build();
        assert!(class_set.classes.contains("start-4"));
    }

    #[test]
    fn test_logical_properties_inset_inline_end() {
        let builder = ClassBuilder::new().inset_inline_end(SpacingValue::Integer(4));

        let class_set = builder.build();
        assert!(class_set.classes.contains("end-4"));
    }

    #[test]
    fn test_logical_properties_inset_block_start() {
        let builder = ClassBuilder::new().inset_block_start(SpacingValue::Integer(2));

        let class_set = builder.build();
        assert!(class_set.classes.contains("top-2"));
    }

    #[test]
    fn test_logical_properties_inset_block_end() {
        let builder = ClassBuilder::new().inset_block_end(SpacingValue::Integer(2));

        let class_set = builder.build();
        assert!(class_set.classes.contains("bottom-2"));
    }

    #[test]
    fn test_logical_properties_convenience_methods() {
        let builder = ClassBuilder::new()
            .margin_inline_start_4()
            .margin_inline_end_4()
            .padding_inline_start_2()
            .padding_inline_end_2()
            .border_inline_start_1()
            .border_inline_end_1();

        let class_set = builder.build();
        assert!(class_set.classes.contains("ms-4"));
        assert!(class_set.classes.contains("me-4"));
        assert!(class_set.classes.contains("ps-2"));
        assert!(class_set.classes.contains("pe-2"));
        assert!(class_set.classes.contains("border-s-1"));
        assert!(class_set.classes.contains("border-e-1"));
    }

    #[test]
    fn test_logical_properties_with_other_utilities() {
        let builder = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .class("text-blue-500")
            .class("font-bold");

        let class_set = builder.build();
        assert!(class_set.classes.contains("ms-4"));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_logical_properties_responsive() {
        let builder = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .responsive(Breakpoint::Md, "ms-8");

        let class_set = builder.build();
        assert!(class_set.classes.contains("ms-4"));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set
            .responsive
            .get(&Breakpoint::Md)
            .unwrap()
            .contains("ms-8"));
    }

    #[test]
    fn test_logical_properties_conditional() {
        let builder = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .conditional("hover", "ms-8");

        let class_set = builder.build();
        assert!(class_set.classes.contains("ms-4"));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set.conditional.get("hover").unwrap().contains("ms-8"));
    }

    #[test]
    fn test_logical_properties_custom_variant() {
        let builder = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .custom_variant("dark", "ms-8");

        let class_set = builder.build();
        assert!(class_set.classes.contains("ms-4"));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set.conditional.get("dark").unwrap().contains("ms-8"));
    }

    #[test]
    fn test_logical_properties_multiple_properties() {
        let builder = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .margin_inline_end(SpacingValue::Integer(4))
            .padding_inline_start(SpacingValue::Integer(2))
            .padding_inline_end(SpacingValue::Integer(2));

        let class_set = builder.build();
        assert!(class_set.classes.contains("ms-4"));
        assert!(class_set.classes.contains("me-4"));
        assert!(class_set.classes.contains("ps-2"));
        assert!(class_set.classes.contains("pe-2"));
    }

    #[test]
    fn test_logical_properties_build_string() {
        let classes = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .class("text-blue-500")
            .build_string();

        assert!(classes.contains("ms-4"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_logical_properties_css_classes() {
        let class_set = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .class("font-bold")
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("ms-4"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_logical_properties_comprehensive_usage() {
        let class_set = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .margin_inline_end(SpacingValue::Integer(4))
            .padding_inline_start(SpacingValue::Integer(2))
            .padding_inline_end(SpacingValue::Integer(2))
            .border_inline_start(SpacingValue::Integer(1))
            .border_inline_end(SpacingValue::Integer(1))
            .inset_inline_start(SpacingValue::Integer(4))
            .inset_inline_end(SpacingValue::Integer(4))
            .inset_block_start(SpacingValue::Integer(2))
            .inset_block_end(SpacingValue::Integer(2))
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("ms-4"));
        assert!(css_classes.contains("me-4"));
        assert!(css_classes.contains("ps-2"));
        assert!(css_classes.contains("pe-2"));
        assert!(css_classes.contains("border-s-1"));
        assert!(css_classes.contains("border-e-1"));
        assert!(css_classes.contains("start-4"));
        assert!(css_classes.contains("end-4"));
        assert!(css_classes.contains("top-2"));
        assert!(css_classes.contains("bottom-2"));
    }

    #[test]
    fn test_logical_properties_all_variants() {
        let class_set = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .margin_inline_end(SpacingValue::Integer(4))
            .margin_block_start(SpacingValue::Integer(4))
            .margin_block_end(SpacingValue::Integer(4))
            .padding_inline_start(SpacingValue::Integer(2))
            .padding_inline_end(SpacingValue::Integer(2))
            .padding_block_start(SpacingValue::Integer(2))
            .padding_block_end(SpacingValue::Integer(2))
            .border_inline_start(SpacingValue::Integer(1))
            .border_inline_end(SpacingValue::Integer(1))
            .border_block_start(SpacingValue::Integer(1))
            .border_block_end(SpacingValue::Integer(1))
            .inset_inline_start(SpacingValue::Integer(4))
            .inset_inline_end(SpacingValue::Integer(4))
            .inset_block_start(SpacingValue::Integer(2))
            .inset_block_end(SpacingValue::Integer(2))
            .build();

        let css_classes = class_set.to_css_classes();

        // Test that all logical property utilities are present
        assert!(css_classes.contains("ms-4"));
        assert!(css_classes.contains("me-4"));
        assert!(css_classes.contains("mt-4"));
        assert!(css_classes.contains("mb-4"));
        assert!(css_classes.contains("ps-2"));
        assert!(css_classes.contains("pe-2"));
        assert!(css_classes.contains("pt-2"));
        assert!(css_classes.contains("pb-2"));
        assert!(css_classes.contains("border-s-1"));
        assert!(css_classes.contains("border-e-1"));
        assert!(css_classes.contains("border-t-1"));
        assert!(css_classes.contains("border-b-1"));
        assert!(css_classes.contains("start-4"));
        assert!(css_classes.contains("end-4"));
        assert!(css_classes.contains("top-2"));
        assert!(css_classes.contains("bottom-2"));
    }
}
