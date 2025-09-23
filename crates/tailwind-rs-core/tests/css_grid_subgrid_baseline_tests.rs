use tailwind_rs_core::utilities::grid::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;

#[cfg(test)]
mod css_grid_subgrid_baseline_tests {
    use super::*;

    #[test]
    fn test_css_grid_subgrid_css_output_baseline() {
        let builder = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .grid_template_rows(GridTemplateRows::Subgrid);
        
        let class_set = builder.build();
        let classes = class_set.to_css_classes();
        
        // Baseline: Should contain both grid subgrid classes
        assert!(classes.contains("grid-cols-subgrid"));
        assert!(classes.contains("grid-rows-subgrid"));
    }

    #[test]
    fn test_css_grid_subgrid_class_generation_baseline() {
        let builder = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .grid_template_rows(GridTemplateRows::Subgrid);
        
        let class_set = builder.build();
        let classes = class_set.to_css_classes();
        
        // Baseline: Should contain both grid subgrid classes
        assert!(classes.contains("grid-cols-subgrid"));
        assert!(classes.contains("grid-rows-subgrid"));
    }

    #[test]
    fn test_grid_template_columns_subgrid_baseline() {
        let columns = GridTemplateColumns::Subgrid;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-subgrid");
    }

    #[test]
    fn test_grid_template_columns_none_baseline() {
        let columns = GridTemplateColumns::None;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-none");
    }

    #[test]
    fn test_grid_template_columns_cols1_baseline() {
        let columns = GridTemplateColumns::Cols1;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-1");
    }

    #[test]
    fn test_grid_template_columns_cols2_baseline() {
        let columns = GridTemplateColumns::Cols2;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-2");
    }

    #[test]
    fn test_grid_template_columns_cols3_baseline() {
        let columns = GridTemplateColumns::Cols3;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-3");
    }

    #[test]
    fn test_grid_template_columns_cols4_baseline() {
        let columns = GridTemplateColumns::Cols4;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-4");
    }

    #[test]
    fn test_grid_template_columns_cols5_baseline() {
        let columns = GridTemplateColumns::Cols5;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-5");
    }

    #[test]
    fn test_grid_template_columns_cols6_baseline() {
        let columns = GridTemplateColumns::Cols6;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-6");
    }

    #[test]
    fn test_grid_template_columns_cols7_baseline() {
        let columns = GridTemplateColumns::Cols7;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-7");
    }

    #[test]
    fn test_grid_template_columns_cols8_baseline() {
        let columns = GridTemplateColumns::Cols8;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-8");
    }

    #[test]
    fn test_grid_template_columns_cols9_baseline() {
        let columns = GridTemplateColumns::Cols9;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-9");
    }

    #[test]
    fn test_grid_template_columns_cols10_baseline() {
        let columns = GridTemplateColumns::Cols10;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-10");
    }

    #[test]
    fn test_grid_template_columns_cols11_baseline() {
        let columns = GridTemplateColumns::Cols11;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-11");
    }

    #[test]
    fn test_grid_template_columns_cols12_baseline() {
        let columns = GridTemplateColumns::Cols12;
        let string_value = columns.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-cols-12");
    }

    #[test]
    fn test_grid_template_rows_subgrid_baseline() {
        let rows = GridTemplateRows::Subgrid;
        let string_value = rows.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-rows-subgrid");
    }

    #[test]
    fn test_grid_template_rows_none_baseline() {
        let rows = GridTemplateRows::None;
        let string_value = rows.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-rows-none");
    }

    #[test]
    fn test_grid_template_rows_auto_baseline() {
        let rows = GridTemplateRows::Auto;
        let string_value = rows.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-rows-auto");
    }

    #[test]
    fn test_grid_template_rows_one_baseline() {
        let rows = GridTemplateRows::One;
        let string_value = rows.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-rows-1");
    }

    #[test]
    fn test_grid_template_rows_two_baseline() {
        let rows = GridTemplateRows::Two;
        let string_value = rows.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-rows-2");
    }

    #[test]
    fn test_grid_template_rows_three_baseline() {
        let rows = GridTemplateRows::Three;
        let string_value = rows.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-rows-3");
    }

    #[test]
    fn test_grid_template_rows_four_baseline() {
        let rows = GridTemplateRows::Four;
        let string_value = rows.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-rows-4");
    }

    #[test]
    fn test_grid_template_rows_five_baseline() {
        let rows = GridTemplateRows::Five;
        let string_value = rows.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-rows-5");
    }

    #[test]
    fn test_grid_template_rows_six_baseline() {
        let rows = GridTemplateRows::Six;
        let string_value = rows.to_string();
        
        // Baseline string output
        assert_eq!(string_value, "grid-rows-6");
    }

    #[test]
    fn test_css_grid_subgrid_serialization_baseline() {
        let columns = GridTemplateColumns::Subgrid;
        let serialized = serde_json::to_string(&columns).unwrap();
        
        // Baseline: Should serialize to JSON
        assert!(serialized.contains("Subgrid"));
        
        // Should deserialize back to the same value
        let deserialized: GridTemplateColumns = serde_json::from_str(&serialized).unwrap();
        assert_eq!(columns, deserialized);
    }

    #[test]
    fn test_css_grid_subgrid_equality_baseline() {
        let columns1 = GridTemplateColumns::Subgrid;
        let columns2 = GridTemplateColumns::Subgrid;
        let columns3 = GridTemplateColumns::Cols3;
        
        // Baseline: Same variants should be equal
        assert_eq!(columns1, columns2);
        assert_ne!(columns1, columns3);
    }

    #[test]
    fn test_css_grid_subgrid_clone_baseline() {
        let columns = GridTemplateColumns::Subgrid;
        let cloned = columns.clone();
        
        // Baseline: Cloned columns should be equal to original
        assert_eq!(columns, cloned);
    }

    #[test]
    fn test_css_grid_subgrid_complex_builder_baseline() {
        let class_set = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .grid_template_rows(GridTemplateRows::Subgrid)
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "grid-cols-3")
            .conditional("hover", "grid-rows-2")
            .build();
        
        let classes = class_set.to_css_classes();
        
        // Baseline: Should contain expected classes
        assert!(classes.contains("grid-cols-subgrid"));
        assert!(classes.contains("grid-rows-subgrid"));
        assert!(classes.contains("text-blue-500"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("md:grid-cols-3"));
        assert!(classes.contains("hover:grid-rows-2"));
    }
}
