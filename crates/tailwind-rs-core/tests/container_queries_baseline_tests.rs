use tailwind_rs_core::utilities::container_queries::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;

#[cfg(test)]
mod container_queries_baseline_tests {
    use super::*;

    #[test]
    fn test_container_queries_css_output_baseline() {
        let builder = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .class("@container/block-size:lg:text-xl");
        
        let class_set = builder.build();
        let classes = class_set.to_css_classes();
        
        // Baseline: Should contain both container query classes
        assert!(classes.contains("@container/inline-size:md:text-lg"));
        assert!(classes.contains("@container/block-size:lg:text-xl"));
    }

    #[test]
    fn test_container_queries_class_generation_baseline() {
        let builder = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .class("@container/aspect-ratio:widescreen:grid-cols-2");
        
        let class_set = builder.build();
        let classes = class_set.to_css_classes();
        
        // Baseline: Should contain both container query classes
        assert!(classes.contains("@container/inline-size:md:text-lg"));
        assert!(classes.contains("@container/aspect-ratio:widescreen:grid-cols-2"));
    }

    #[test]
    fn test_container_query_inline_size_baseline() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        let css_query = query.to_css_query();
        
        // Baseline CSS query output
        assert_eq!(css_query, "@container (inline-size > 768px)");
    }

    #[test]
    fn test_container_query_block_size_baseline() {
        let query = ContainerQuery::block_size(ContainerSize::Lg);
        let css_query = query.to_css_query();
        
        // Baseline CSS query output
        assert_eq!(css_query, "@container (block-size > 1024px)");
    }

    #[test]
    fn test_container_query_width_baseline() {
        let query = ContainerQuery::width(ContainerSize::Sm);
        let css_query = query.to_css_query();
        
        // Baseline CSS query output
        assert_eq!(css_query, "@container (width > 640px)");
    }

    #[test]
    fn test_container_query_height_baseline() {
        let query = ContainerQuery::height(ContainerSize::Xl);
        let css_query = query.to_css_query();
        
        // Baseline CSS query output
        assert_eq!(css_query, "@container (height > 1280px)");
    }

    #[test]
    fn test_container_query_aspect_ratio_baseline() {
        let query = ContainerQuery::aspect_ratio(ContainerAspectRatio::Widescreen);
        let css_query = query.to_css_query();
        
        // Baseline CSS query output
        assert_eq!(css_query, "@container (aspect-ratio > 16/9)");
    }

    #[test]
    fn test_container_query_orientation_baseline() {
        let query = ContainerQuery::orientation(ContainerOrientation::Landscape);
        let css_query = query.to_css_query();
        
        // Baseline CSS query output
        assert_eq!(css_query, "@container (orientation: landscape)");
    }

    #[test]
    fn test_container_size_xs_baseline() {
        let size = ContainerSize::Xs;
        let css_value = size.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "320px");
    }

    #[test]
    fn test_container_size_sm_baseline() {
        let size = ContainerSize::Sm;
        let css_value = size.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "640px");
    }

    #[test]
    fn test_container_size_md_baseline() {
        let size = ContainerSize::Md;
        let css_value = size.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "768px");
    }

    #[test]
    fn test_container_size_lg_baseline() {
        let size = ContainerSize::Lg;
        let css_value = size.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "1024px");
    }

    #[test]
    fn test_container_size_xl_baseline() {
        let size = ContainerSize::Xl;
        let css_value = size.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "1280px");
    }

    #[test]
    fn test_container_size_xl2_baseline() {
        let size = ContainerSize::Xl2;
        let css_value = size.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "1536px");
    }

    #[test]
    fn test_container_size_custom_baseline() {
        let size = ContainerSize::Custom("500px".to_string());
        let css_value = size.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "500px");
    }

    #[test]
    fn test_container_aspect_ratio_square_baseline() {
        let ratio = ContainerAspectRatio::Square;
        let css_value = ratio.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "1/1");
    }

    #[test]
    fn test_container_aspect_ratio_video_baseline() {
        let ratio = ContainerAspectRatio::Video;
        let css_value = ratio.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "4/3");
    }

    #[test]
    fn test_container_aspect_ratio_widescreen_baseline() {
        let ratio = ContainerAspectRatio::Widescreen;
        let css_value = ratio.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "16/9");
    }

    #[test]
    fn test_container_aspect_ratio_ultrawide_baseline() {
        let ratio = ContainerAspectRatio::Ultrawide;
        let css_value = ratio.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "21/9");
    }

    #[test]
    fn test_container_aspect_ratio_custom_baseline() {
        let ratio = ContainerAspectRatio::Custom("3/2".to_string());
        let css_value = ratio.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "3/2");
    }

    #[test]
    fn test_container_orientation_landscape_baseline() {
        let orientation = ContainerOrientation::Landscape;
        let css_value = orientation.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "landscape");
    }

    #[test]
    fn test_container_orientation_portrait_baseline() {
        let orientation = ContainerOrientation::Portrait;
        let css_value = orientation.to_css_value();
        
        // Baseline CSS value output
        assert_eq!(css_value, "portrait");
    }

    #[test]
    fn test_container_query_class_name_baseline() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        let class_name = query.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "@container/inline-size:md");
    }

    #[test]
    fn test_container_size_class_name_baseline() {
        let size = ContainerSize::Lg;
        let class_name = size.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "lg");
    }

    #[test]
    fn test_container_aspect_ratio_class_name_baseline() {
        let ratio = ContainerAspectRatio::Widescreen;
        let class_name = ratio.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "widescreen");
    }

    #[test]
    fn test_container_orientation_class_name_baseline() {
        let orientation = ContainerOrientation::Landscape;
        let class_name = orientation.to_class_name();
        
        // Baseline class name output
        assert_eq!(class_name, "landscape");
    }

    #[test]
    fn test_container_query_display_baseline() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        let display = format!("{}", query);
        
        // Baseline display output
        assert_eq!(display, "@container (inline-size > 768px)");
    }

    #[test]
    fn test_container_size_display_baseline() {
        let size = ContainerSize::Lg;
        let display = format!("{}", size);
        
        // Baseline display output
        assert_eq!(display, "1024px");
    }

    #[test]
    fn test_container_aspect_ratio_display_baseline() {
        let ratio = ContainerAspectRatio::Widescreen;
        let display = format!("{}", ratio);
        
        // Baseline display output
        assert_eq!(display, "16/9");
    }

    #[test]
    fn test_container_orientation_display_baseline() {
        let orientation = ContainerOrientation::Landscape;
        let display = format!("{}", orientation);
        
        // Baseline display output
        assert_eq!(display, "landscape");
    }

    #[test]
    fn test_container_queries_serialization_baseline() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        let serialized = serde_json::to_string(&query).unwrap();
        
        // Baseline: Should serialize to JSON
        assert!(serialized.contains("InlineSize"));
        assert!(serialized.contains("Md"));
        
        // Should deserialize back to the same value
        let deserialized: ContainerQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(query, deserialized);
    }

    #[test]
    fn test_container_queries_equality_baseline() {
        let query1 = ContainerQuery::inline_size(ContainerSize::Md);
        let query2 = ContainerQuery::inline_size(ContainerSize::Md);
        let query3 = ContainerQuery::inline_size(ContainerSize::Lg);
        
        // Baseline: Same variants should be equal
        assert_eq!(query1, query2);
        assert_ne!(query1, query3);
    }

    #[test]
    fn test_container_queries_clone_baseline() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        let cloned = query.clone();
        
        // Baseline: Cloned query should be equal to original
        assert_eq!(query, cloned);
    }

    #[test]
    fn test_container_queries_complex_builder_baseline() {
        let class_set = ClassBuilder::new()
            .class("@container/inline-size:md:text-lg")
            .class("@container/block-size:lg:text-xl")
            .class("@container/aspect-ratio:widescreen:grid-cols-2")
            .class("@container/orientation:landscape:flex-row")
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "@container/block-size:lg:text-xl")
            .conditional("hover", "@container/inline-size:md:text-lg")
            .build();
        
        let classes = class_set.to_css_classes();
        
        // Baseline: Should contain expected classes
        assert!(classes.contains("@container/inline-size:md:text-lg"));
        assert!(classes.contains("@container/block-size:lg:text-xl"));
        assert!(classes.contains("@container/aspect-ratio:widescreen:grid-cols-2"));
        assert!(classes.contains("@container/orientation:landscape:flex-row"));
        assert!(classes.contains("text-blue-500"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("md:@container/block-size:lg:text-xl"));
        assert!(classes.contains("hover:@container/inline-size:md:text-lg"));
    }
}
