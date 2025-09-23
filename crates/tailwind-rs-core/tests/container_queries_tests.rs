use tailwind_rs_core::utilities::container_queries::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;

#[cfg(test)]
mod container_queries_unit_tests {
    use super::*;

    #[test]
    fn test_container_query_inline_size() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        assert_eq!(query.to_css_query(), "@container (inline-size > 768px)");
        assert_eq!(query.to_class_name(), "@container/inline-size:md");
    }

    #[test]
    fn test_container_query_block_size() {
        let query = ContainerQuery::block_size(ContainerSize::Lg);
        assert_eq!(query.to_css_query(), "@container (block-size > 1024px)");
        assert_eq!(query.to_class_name(), "@container/block-size:lg");
    }

    #[test]
    fn test_container_query_width() {
        let query = ContainerQuery::width(ContainerSize::Sm);
        assert_eq!(query.to_css_query(), "@container (width > 640px)");
        assert_eq!(query.to_class_name(), "@container/width:sm");
    }

    #[test]
    fn test_container_query_height() {
        let query = ContainerQuery::height(ContainerSize::Xl);
        assert_eq!(query.to_css_query(), "@container (height > 1280px)");
        assert_eq!(query.to_class_name(), "@container/height:xl");
    }

    #[test]
    fn test_container_query_aspect_ratio() {
        let query = ContainerQuery::aspect_ratio(ContainerAspectRatio::Widescreen);
        assert_eq!(query.to_css_query(), "@container (aspect-ratio > 16/9)");
        assert_eq!(query.to_class_name(), "@container/aspect-ratio:widescreen");
    }

    #[test]
    fn test_container_query_orientation() {
        let query = ContainerQuery::orientation(ContainerOrientation::Landscape);
        assert_eq!(query.to_css_query(), "@container (orientation: landscape)");
        assert_eq!(query.to_class_name(), "@container/orientation:landscape");
    }

    #[test]
    fn test_container_size_xs() {
        let size = ContainerSize::Xs;
        assert_eq!(size.to_css_value(), "320px");
        assert_eq!(size.to_class_name(), "xs");
    }

    #[test]
    fn test_container_size_sm() {
        let size = ContainerSize::Sm;
        assert_eq!(size.to_css_value(), "640px");
        assert_eq!(size.to_class_name(), "sm");
    }

    #[test]
    fn test_container_size_md() {
        let size = ContainerSize::Md;
        assert_eq!(size.to_css_value(), "768px");
        assert_eq!(size.to_class_name(), "md");
    }

    #[test]
    fn test_container_size_lg() {
        let size = ContainerSize::Lg;
        assert_eq!(size.to_css_value(), "1024px");
        assert_eq!(size.to_class_name(), "lg");
    }

    #[test]
    fn test_container_size_xl() {
        let size = ContainerSize::Xl;
        assert_eq!(size.to_css_value(), "1280px");
        assert_eq!(size.to_class_name(), "xl");
    }

    #[test]
    fn test_container_size_xl2() {
        let size = ContainerSize::Xl2;
        assert_eq!(size.to_css_value(), "1536px");
        assert_eq!(size.to_class_name(), "2xl");
    }

    #[test]
    fn test_container_size_custom() {
        let size = ContainerSize::Custom("500px".to_string());
        assert_eq!(size.to_css_value(), "500px");
        assert_eq!(size.to_class_name(), "500px");
    }

    #[test]
    fn test_container_aspect_ratio_square() {
        let ratio = ContainerAspectRatio::Square;
        assert_eq!(ratio.to_css_value(), "1/1");
        assert_eq!(ratio.to_class_name(), "square");
    }

    #[test]
    fn test_container_aspect_ratio_video() {
        let ratio = ContainerAspectRatio::Video;
        assert_eq!(ratio.to_css_value(), "4/3");
        assert_eq!(ratio.to_class_name(), "video");
    }

    #[test]
    fn test_container_aspect_ratio_widescreen() {
        let ratio = ContainerAspectRatio::Widescreen;
        assert_eq!(ratio.to_css_value(), "16/9");
        assert_eq!(ratio.to_class_name(), "widescreen");
    }

    #[test]
    fn test_container_aspect_ratio_ultrawide() {
        let ratio = ContainerAspectRatio::Ultrawide;
        assert_eq!(ratio.to_css_value(), "21/9");
        assert_eq!(ratio.to_class_name(), "ultrawide");
    }

    #[test]
    fn test_container_aspect_ratio_custom() {
        let ratio = ContainerAspectRatio::Custom("3/2".to_string());
        assert_eq!(ratio.to_css_value(), "3/2");
        assert_eq!(ratio.to_class_name(), "3/2");
    }

    #[test]
    fn test_container_orientation_landscape() {
        let orientation = ContainerOrientation::Landscape;
        assert_eq!(orientation.to_css_value(), "landscape");
        assert_eq!(orientation.to_class_name(), "landscape");
    }

    #[test]
    fn test_container_orientation_portrait() {
        let orientation = ContainerOrientation::Portrait;
        assert_eq!(orientation.to_css_value(), "portrait");
        assert_eq!(orientation.to_class_name(), "portrait");
    }

    #[test]
    fn test_container_query_display() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        assert_eq!(format!("{}", query), "@container (inline-size > 768px)");
    }

    #[test]
    fn test_container_size_display() {
        let size = ContainerSize::Lg;
        assert_eq!(format!("{}", size), "1024px");
    }

    #[test]
    fn test_container_aspect_ratio_display() {
        let ratio = ContainerAspectRatio::Widescreen;
        assert_eq!(format!("{}", ratio), "16/9");
    }

    #[test]
    fn test_container_orientation_display() {
        let orientation = ContainerOrientation::Landscape;
        assert_eq!(format!("{}", orientation), "landscape");
    }

    #[test]
    fn test_container_query_serialization() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        let serialized = serde_json::to_string(&query).unwrap();
        let deserialized: ContainerQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(query, deserialized);
    }

    #[test]
    fn test_container_size_serialization() {
        let size = ContainerSize::Lg;
        let serialized = serde_json::to_string(&size).unwrap();
        let deserialized: ContainerSize = serde_json::from_str(&serialized).unwrap();
        assert_eq!(size, deserialized);
    }

    #[test]
    fn test_container_aspect_ratio_serialization() {
        let ratio = ContainerAspectRatio::Widescreen;
        let serialized = serde_json::to_string(&ratio).unwrap();
        let deserialized: ContainerAspectRatio = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ratio, deserialized);
    }

    #[test]
    fn test_container_orientation_serialization() {
        let orientation = ContainerOrientation::Landscape;
        let serialized = serde_json::to_string(&orientation).unwrap();
        let deserialized: ContainerOrientation = serde_json::from_str(&serialized).unwrap();
        assert_eq!(orientation, deserialized);
    }

    #[test]
    fn test_container_query_clone() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        let cloned = query.clone();
        assert_eq!(query, cloned);
    }

    #[test]
    fn test_container_query_partial_eq() {
        let query1 = ContainerQuery::inline_size(ContainerSize::Md);
        let query2 = ContainerQuery::inline_size(ContainerSize::Md);
        let query3 = ContainerQuery::inline_size(ContainerSize::Lg);
        
        assert_eq!(query1, query2);
        assert_ne!(query1, query3);
    }

    #[test]
    fn test_container_size_clone() {
        let size = ContainerSize::Lg;
        let cloned = size.clone();
        assert_eq!(size, cloned);
    }

    #[test]
    fn test_container_size_partial_eq() {
        let size1 = ContainerSize::Md;
        let size2 = ContainerSize::Md;
        let size3 = ContainerSize::Lg;
        
        assert_eq!(size1, size2);
        assert_ne!(size1, size3);
    }

    #[test]
    fn test_container_aspect_ratio_clone() {
        let ratio = ContainerAspectRatio::Widescreen;
        let cloned = ratio.clone();
        assert_eq!(ratio, cloned);
    }

    #[test]
    fn test_container_aspect_ratio_partial_eq() {
        let ratio1 = ContainerAspectRatio::Widescreen;
        let ratio2 = ContainerAspectRatio::Widescreen;
        let ratio3 = ContainerAspectRatio::Square;
        
        assert_eq!(ratio1, ratio2);
        assert_ne!(ratio1, ratio3);
    }

    #[test]
    fn test_container_orientation_clone() {
        let orientation = ContainerOrientation::Landscape;
        let cloned = orientation.clone();
        assert_eq!(orientation, cloned);
    }

    #[test]
    fn test_container_orientation_partial_eq() {
        let orientation1 = ContainerOrientation::Landscape;
        let orientation2 = ContainerOrientation::Landscape;
        let orientation3 = ContainerOrientation::Portrait;
        
        assert_eq!(orientation1, orientation2);
        assert_ne!(orientation1, orientation3);
    }
}

#[cfg(test)]
mod container_queries_integration_tests {
    use super::*;

    #[test]
    fn test_container_queries_with_class_builder() {
        let builder = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("@container/inline-size:md:text-lg"));
    }

    #[test]
    fn test_container_queries_with_other_utilities() {
        let builder = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .class("text-blue-500")
            .class("font-bold");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("@container/inline-size:md:text-lg"));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_container_queries_responsive() {
        let builder = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .responsive(Breakpoint::Md, "@container/block-size:lg:text-xl");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("@container/inline-size:md:text-lg"));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set.responsive.get(&Breakpoint::Md).unwrap().contains("@container/block-size:lg:text-xl"));
    }

    #[test]
    fn test_container_queries_conditional() {
        let builder = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .conditional("hover", "@container/block-size:lg:text-xl");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("@container/inline-size:md:text-lg"));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set.conditional.get("hover").unwrap().contains("@container/block-size:lg:text-xl"));
    }

    #[test]
    fn test_container_queries_custom_variant() {
        let builder = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .custom_variant("dark", "@container/block-size:lg:text-xl");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("@container/inline-size:md:text-lg"));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set.conditional.get("dark").unwrap().contains("@container/block-size:lg:text-xl"));
    }

    #[test]
    fn test_container_queries_multiple_queries() {
        let builder = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .class("@container/block-size:lg:text-xl");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("@container/inline-size:md:text-lg"));
        assert!(class_set.classes.contains("@container/block-size:lg:text-xl"));
    }

    #[test]
    fn test_container_queries_build_string() {
        let classes = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .class("text-blue-500")
            .build_string();
        
        assert!(classes.contains("@container/inline-size:md:text-lg"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_container_queries_css_classes() {
        let class_set = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .class("font-bold")
            .build();
        
        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("@container/inline-size:md:text-lg"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_container_queries_comprehensive_usage() {
        let class_set = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .class("@container/block-size:lg:text-xl")
            .class("@container/aspect-ratio:widescreen:grid-cols-2")
            .class("@container/orientation:landscape:flex-row")
            .build();
        
        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("@container/inline-size:md:text-lg"));
        assert!(css_classes.contains("@container/block-size:lg:text-xl"));
        assert!(css_classes.contains("@container/aspect-ratio:widescreen:grid-cols-2"));
        assert!(css_classes.contains("@container/orientation:landscape:flex-row"));
    }

    #[test]
    fn test_container_queries_all_variants() {
        let class_set = ClassBuilder::new()
            .class("@container/inline-size:xs:text-sm")
            .class("@container/inline-size:sm:text-base")
            .class("@container/inline-size:md:text-lg")
            .class("@container/inline-size:lg:text-xl")
            .class("@container/inline-size:xl:text-2xl")
            .class("@container/inline-size:2xl:text-3xl")
            .class("@container/block-size:xs:h-8")
            .class("@container/block-size:sm:h-10")
            .class("@container/block-size:md:h-12")
            .class("@container/block-size:lg:h-14")
            .class("@container/block-size:xl:h-16")
            .class("@container/block-size:2xl:h-20")
            .class("@container/width:xs:w-8")
            .class("@container/width:sm:w-10")
            .class("@container/width:md:w-12")
            .class("@container/width:lg:w-14")
            .class("@container/width:xl:w-16")
            .class("@container/width:2xl:w-20")
            .class("@container/height:xs:h-8")
            .class("@container/height:sm:h-10")
            .class("@container/height:md:h-12")
            .class("@container/height:lg:h-14")
            .class("@container/height:xl:h-16")
            .class("@container/height:2xl:h-20")
            .class("@container/aspect-ratio:square:aspect-square")
            .class("@container/aspect-ratio:video:aspect-video")
            .class("@container/aspect-ratio:widescreen:aspect-widescreen")
            .class("@container/aspect-ratio:ultrawide:aspect-ultrawide")
            .class("@container/orientation:landscape:flex-row")
            .class("@container/orientation:portrait:flex-col")
            .build();
        
        let css_classes = class_set.to_css_classes();
        
        // Test that all container query utilities are present
        assert!(css_classes.contains("@container/inline-size:xs:text-sm"));
        assert!(css_classes.contains("@container/inline-size:sm:text-base"));
        assert!(css_classes.contains("@container/inline-size:md:text-lg"));
        assert!(css_classes.contains("@container/inline-size:lg:text-xl"));
        assert!(css_classes.contains("@container/inline-size:xl:text-2xl"));
        assert!(css_classes.contains("@container/inline-size:2xl:text-3xl"));
        assert!(css_classes.contains("@container/block-size:xs:h-8"));
        assert!(css_classes.contains("@container/block-size:sm:h-10"));
        assert!(css_classes.contains("@container/block-size:md:h-12"));
        assert!(css_classes.contains("@container/block-size:lg:h-14"));
        assert!(css_classes.contains("@container/block-size:xl:h-16"));
        assert!(css_classes.contains("@container/block-size:2xl:h-20"));
        assert!(css_classes.contains("@container/width:xs:w-8"));
        assert!(css_classes.contains("@container/width:sm:w-10"));
        assert!(css_classes.contains("@container/width:md:w-12"));
        assert!(css_classes.contains("@container/width:lg:w-14"));
        assert!(css_classes.contains("@container/width:xl:w-16"));
        assert!(css_classes.contains("@container/width:2xl:w-20"));
        assert!(css_classes.contains("@container/height:xs:h-8"));
        assert!(css_classes.contains("@container/height:sm:h-10"));
        assert!(css_classes.contains("@container/height:md:h-12"));
        assert!(css_classes.contains("@container/height:lg:h-14"));
        assert!(css_classes.contains("@container/height:xl:h-16"));
        assert!(css_classes.contains("@container/height:2xl:h-20"));
        assert!(css_classes.contains("@container/aspect-ratio:square:aspect-square"));
        assert!(css_classes.contains("@container/aspect-ratio:video:aspect-video"));
        assert!(css_classes.contains("@container/aspect-ratio:widescreen:aspect-widescreen"));
        assert!(css_classes.contains("@container/aspect-ratio:ultrawide:aspect-ultrawide"));
        assert!(css_classes.contains("@container/orientation:landscape:flex-row"));
        assert!(css_classes.contains("@container/orientation:portrait:flex-col"));
    }
}
