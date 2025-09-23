use tailwind_rs_core::utilities::grid::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;

#[cfg(test)]
mod css_grid_subgrid_unit_tests {
    use super::*;

    #[test]
    fn test_grid_template_columns_subgrid() {
        let columns = GridTemplateColumns::Subgrid;
        assert_eq!(columns.to_string(), "grid-cols-subgrid");
    }

    #[test]
    fn test_grid_template_rows_subgrid() {
        let rows = GridTemplateRows::Subgrid;
        assert_eq!(rows.to_string(), "grid-rows-subgrid");
    }

    #[test]
    fn test_grid_template_columns_none() {
        let columns = GridTemplateColumns::None;
        assert_eq!(columns.to_string(), "grid-cols-none");
    }

    #[test]
    fn test_grid_template_rows_none() {
        let rows = GridTemplateRows::None;
        assert_eq!(rows.to_string(), "grid-rows-none");
    }

    #[test]
    fn test_grid_template_columns_cols1() {
        let columns = GridTemplateColumns::Cols1;
        assert_eq!(columns.to_string(), "grid-cols-1");
    }

    #[test]
    fn test_grid_template_columns_cols2() {
        let columns = GridTemplateColumns::Cols2;
        assert_eq!(columns.to_string(), "grid-cols-2");
    }

    #[test]
    fn test_grid_template_columns_cols3() {
        let columns = GridTemplateColumns::Cols3;
        assert_eq!(columns.to_string(), "grid-cols-3");
    }

    #[test]
    fn test_grid_template_columns_cols4() {
        let columns = GridTemplateColumns::Cols4;
        assert_eq!(columns.to_string(), "grid-cols-4");
    }

    #[test]
    fn test_grid_template_columns_cols5() {
        let columns = GridTemplateColumns::Cols5;
        assert_eq!(columns.to_string(), "grid-cols-5");
    }

    #[test]
    fn test_grid_template_columns_cols6() {
        let columns = GridTemplateColumns::Cols6;
        assert_eq!(columns.to_string(), "grid-cols-6");
    }

    #[test]
    fn test_grid_template_columns_cols7() {
        let columns = GridTemplateColumns::Cols7;
        assert_eq!(columns.to_string(), "grid-cols-7");
    }

    #[test]
    fn test_grid_template_columns_cols8() {
        let columns = GridTemplateColumns::Cols8;
        assert_eq!(columns.to_string(), "grid-cols-8");
    }

    #[test]
    fn test_grid_template_columns_cols9() {
        let columns = GridTemplateColumns::Cols9;
        assert_eq!(columns.to_string(), "grid-cols-9");
    }

    #[test]
    fn test_grid_template_columns_cols10() {
        let columns = GridTemplateColumns::Cols10;
        assert_eq!(columns.to_string(), "grid-cols-10");
    }

    #[test]
    fn test_grid_template_columns_cols11() {
        let columns = GridTemplateColumns::Cols11;
        assert_eq!(columns.to_string(), "grid-cols-11");
    }

    #[test]
    fn test_grid_template_columns_cols12() {
        let columns = GridTemplateColumns::Cols12;
        assert_eq!(columns.to_string(), "grid-cols-12");
    }

    #[test]
    fn test_grid_template_rows_auto() {
        let rows = GridTemplateRows::Auto;
        assert_eq!(rows.to_string(), "grid-rows-auto");
    }

    #[test]
    fn test_grid_template_rows_one() {
        let rows = GridTemplateRows::One;
        assert_eq!(rows.to_string(), "grid-rows-1");
    }

    #[test]
    fn test_grid_template_rows_two() {
        let rows = GridTemplateRows::Two;
        assert_eq!(rows.to_string(), "grid-rows-2");
    }

    #[test]
    fn test_grid_template_rows_three() {
        let rows = GridTemplateRows::Three;
        assert_eq!(rows.to_string(), "grid-rows-3");
    }

    #[test]
    fn test_grid_template_rows_four() {
        let rows = GridTemplateRows::Four;
        assert_eq!(rows.to_string(), "grid-rows-4");
    }

    #[test]
    fn test_grid_template_rows_five() {
        let rows = GridTemplateRows::Five;
        assert_eq!(rows.to_string(), "grid-rows-5");
    }

    #[test]
    fn test_grid_template_rows_six() {
        let rows = GridTemplateRows::Six;
        assert_eq!(rows.to_string(), "grid-rows-6");
    }

    #[test]
    fn test_grid_template_columns_serialization() {
        let columns = GridTemplateColumns::Subgrid;
        let serialized = serde_json::to_string(&columns).unwrap();
        let deserialized: GridTemplateColumns = serde_json::from_str(&serialized).unwrap();
        assert_eq!(columns, deserialized);
    }

    #[test]
    fn test_grid_template_rows_serialization() {
        let rows = GridTemplateRows::Subgrid;
        let serialized = serde_json::to_string(&rows).unwrap();
        let deserialized: GridTemplateRows = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rows, deserialized);
    }

    #[test]
    fn test_grid_template_columns_clone() {
        let columns = GridTemplateColumns::Subgrid;
        let cloned = columns.clone();
        assert_eq!(columns, cloned);
    }

    #[test]
    fn test_grid_template_rows_clone() {
        let rows = GridTemplateRows::Subgrid;
        let cloned = rows.clone();
        assert_eq!(rows, cloned);
    }

    #[test]
    fn test_grid_template_columns_partial_eq() {
        let columns1 = GridTemplateColumns::Subgrid;
        let columns2 = GridTemplateColumns::Subgrid;
        let columns3 = GridTemplateColumns::Cols3;
        
        assert_eq!(columns1, columns2);
        assert_ne!(columns1, columns3);
    }

    #[test]
    fn test_grid_template_rows_partial_eq() {
        let rows1 = GridTemplateRows::Subgrid;
        let rows2 = GridTemplateRows::Subgrid;
        let rows3 = GridTemplateRows::Three;
        
        assert_eq!(rows1, rows2);
        assert_ne!(rows1, rows3);
    }
}

#[cfg(test)]
mod css_grid_subgrid_integration_tests {
    use super::*;

    #[test]
    fn test_css_grid_subgrid_with_class_builder() {
        let builder = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("grid-cols-subgrid"));
    }

    #[test]
    fn test_css_grid_subgrid_rows_with_class_builder() {
        let builder = ClassBuilder::new()
            .grid_template_rows(GridTemplateRows::Subgrid);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("grid-rows-subgrid"));
    }

    #[test]
    fn test_css_grid_subgrid_with_other_utilities() {
        let builder = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .grid_template_rows(GridTemplateRows::Subgrid)
            .class("text-blue-500")
            .class("font-bold");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("grid-cols-subgrid"));
        assert!(class_set.classes.contains("grid-rows-subgrid"));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_css_grid_subgrid_responsive() {
        let builder = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .responsive(Breakpoint::Md, "grid-cols-3");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("grid-cols-subgrid"));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set.responsive.get(&Breakpoint::Md).unwrap().contains("grid-cols-3"));
    }

    #[test]
    fn test_css_grid_subgrid_conditional() {
        let builder = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .conditional("hover", "grid-cols-3");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("grid-cols-subgrid"));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set.conditional.get("hover").unwrap().contains("grid-cols-3"));
    }

    #[test]
    fn test_css_grid_subgrid_custom_variant() {
        let builder = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .custom_variant("dark", "grid-cols-3");
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("grid-cols-subgrid"));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set.conditional.get("dark").unwrap().contains("grid-cols-3"));
    }

    #[test]
    fn test_css_grid_subgrid_multiple_grids() {
        let builder = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .grid_template_rows(GridTemplateRows::Subgrid);
        
        let class_set = builder.build();
        assert!(class_set.classes.contains("grid-cols-subgrid"));
        assert!(class_set.classes.contains("grid-rows-subgrid"));
    }

    #[test]
    fn test_css_grid_subgrid_build_string() {
        let classes = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .class("text-blue-500")
            .build_string();
        
        assert!(classes.contains("grid-cols-subgrid"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_css_grid_subgrid_css_classes() {
        let class_set = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .class("font-bold")
            .build();
        
        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("grid-cols-subgrid"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_css_grid_subgrid_comprehensive_usage() {
        let class_set = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .grid_template_rows(GridTemplateRows::Subgrid)
            .build();
        
        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("grid-cols-subgrid"));
        assert!(css_classes.contains("grid-rows-subgrid"));
    }

    #[test]
    fn test_css_grid_subgrid_all_variants() {
        let class_set = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::None)
            .grid_template_columns(GridTemplateColumns::Subgrid)
            .grid_template_columns(GridTemplateColumns::Cols1)
            .grid_template_columns(GridTemplateColumns::Cols2)
            .grid_template_columns(GridTemplateColumns::Cols3)
            .grid_template_columns(GridTemplateColumns::Cols4)
            .grid_template_columns(GridTemplateColumns::Cols5)
            .grid_template_columns(GridTemplateColumns::Cols6)
            .grid_template_columns(GridTemplateColumns::Cols7)
            .grid_template_columns(GridTemplateColumns::Cols8)
            .grid_template_columns(GridTemplateColumns::Cols9)
            .grid_template_columns(GridTemplateColumns::Cols10)
            .grid_template_columns(GridTemplateColumns::Cols11)
            .grid_template_columns(GridTemplateColumns::Cols12)
            .grid_template_rows(GridTemplateRows::None)
            .grid_template_rows(GridTemplateRows::Subgrid)
            .grid_template_rows(GridTemplateRows::Auto)
            .grid_template_rows(GridTemplateRows::One)
            .grid_template_rows(GridTemplateRows::Two)
            .grid_template_rows(GridTemplateRows::Three)
            .grid_template_rows(GridTemplateRows::Four)
            .grid_template_rows(GridTemplateRows::Five)
            .grid_template_rows(GridTemplateRows::Six)
            .build();
        
        let css_classes = class_set.to_css_classes();
        
        // Test that all grid subgrid utilities are present
        assert!(css_classes.contains("grid-cols-none"));
        assert!(css_classes.contains("grid-cols-subgrid"));
        assert!(css_classes.contains("grid-cols-1"));
        assert!(css_classes.contains("grid-cols-2"));
        assert!(css_classes.contains("grid-cols-3"));
        assert!(css_classes.contains("grid-cols-4"));
        assert!(css_classes.contains("grid-cols-5"));
        assert!(css_classes.contains("grid-cols-6"));
        assert!(css_classes.contains("grid-cols-7"));
        assert!(css_classes.contains("grid-cols-8"));
        assert!(css_classes.contains("grid-cols-9"));
        assert!(css_classes.contains("grid-cols-10"));
        assert!(css_classes.contains("grid-cols-11"));
        assert!(css_classes.contains("grid-cols-12"));
        assert!(css_classes.contains("grid-rows-none"));
        assert!(css_classes.contains("grid-rows-subgrid"));
        assert!(css_classes.contains("grid-rows-auto"));
        assert!(css_classes.contains("grid-rows-1"));
        assert!(css_classes.contains("grid-rows-2"));
        assert!(css_classes.contains("grid-rows-3"));
        assert!(css_classes.contains("grid-rows-4"));
        assert!(css_classes.contains("grid-rows-5"));
        assert!(css_classes.contains("grid-rows-6"));
    }
}
