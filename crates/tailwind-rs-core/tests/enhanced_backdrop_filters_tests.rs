use tailwind_rs_core::utilities::enhanced_backdrop_filters::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod enhanced_backdrop_filters_unit_tests {
    use super::*;

    #[test]
    fn test_enhanced_backdrop_blur_none() {
        let blur = EnhancedBackdropBlur::None;
        assert_eq!(blur.to_string(), "none");
        assert_eq!(blur.to_class_name(), "backdrop-blur-none");
        assert_eq!(blur.to_css_value(), "none");
    }

    #[test]
    fn test_enhanced_backdrop_blur_sm() {
        let blur = EnhancedBackdropBlur::Sm;
        assert_eq!(blur.to_string(), "4px");
        assert_eq!(blur.to_class_name(), "backdrop-blur-sm");
        assert_eq!(blur.to_css_value(), "4px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_default() {
        let blur = EnhancedBackdropBlur::Default;
        assert_eq!(blur.to_string(), "8px");
        assert_eq!(blur.to_class_name(), "backdrop-blur");
        assert_eq!(blur.to_css_value(), "8px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_md() {
        let blur = EnhancedBackdropBlur::Md;
        assert_eq!(blur.to_string(), "12px");
        assert_eq!(blur.to_class_name(), "backdrop-blur-md");
        assert_eq!(blur.to_css_value(), "12px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_lg() {
        let blur = EnhancedBackdropBlur::Lg;
        assert_eq!(blur.to_string(), "16px");
        assert_eq!(blur.to_class_name(), "backdrop-blur-lg");
        assert_eq!(blur.to_css_value(), "16px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_xl() {
        let blur = EnhancedBackdropBlur::Xl;
        assert_eq!(blur.to_string(), "24px");
        assert_eq!(blur.to_class_name(), "backdrop-blur-xl");
        assert_eq!(blur.to_css_value(), "24px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_2xl() {
        let blur = EnhancedBackdropBlur::Xl2;
        assert_eq!(blur.to_string(), "40px");
        assert_eq!(blur.to_class_name(), "backdrop-blur-2xl");
        assert_eq!(blur.to_css_value(), "40px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_3xl() {
        let blur = EnhancedBackdropBlur::Xl3;
        assert_eq!(blur.to_string(), "64px");
        assert_eq!(blur.to_class_name(), "backdrop-blur-3xl");
        assert_eq!(blur.to_css_value(), "64px");
    }

    #[test]
    fn test_enhanced_backdrop_brightness_zero() {
        let brightness = EnhancedBackdropBrightness::Zero;
        assert_eq!(brightness.to_string(), "0");
    }

    #[test]
    fn test_enhanced_backdrop_brightness_half() {
        let brightness = EnhancedBackdropBrightness::Half;
        assert_eq!(brightness.to_string(), "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_brightness_default() {
        let brightness = EnhancedBackdropBrightness::Default;
        assert_eq!(brightness.to_string(), "1");
    }

    #[test]
    fn test_enhanced_backdrop_brightness_one_hundred_fifty() {
        let brightness = EnhancedBackdropBrightness::OneHundredFifty;
        assert_eq!(brightness.to_string(), "1.5");
    }

    #[test]
    fn test_enhanced_backdrop_brightness_two_hundred() {
        let brightness = EnhancedBackdropBrightness::TwoHundred;
        assert_eq!(brightness.to_string(), "2");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_zero() {
        let contrast = EnhancedBackdropContrast::Zero;
        assert_eq!(contrast.to_string(), "0");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_half() {
        let contrast = EnhancedBackdropContrast::Half;
        assert_eq!(contrast.to_string(), "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_default() {
        let contrast = EnhancedBackdropContrast::Default;
        assert_eq!(contrast.to_string(), "1");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_one_hundred_fifty() {
        let contrast = EnhancedBackdropContrast::OneHundredFifty;
        assert_eq!(contrast.to_string(), "1.5");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_two_hundred() {
        let contrast = EnhancedBackdropContrast::TwoHundred;
        assert_eq!(contrast.to_string(), "2");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_none() {
        let grayscale = EnhancedBackdropGrayscale::None;
        assert_eq!(grayscale.to_string(), "0");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_quarter() {
        let grayscale = EnhancedBackdropGrayscale::Quarter;
        assert_eq!(grayscale.to_string(), "0.25");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_half() {
        let grayscale = EnhancedBackdropGrayscale::Half;
        assert_eq!(grayscale.to_string(), "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_three_quarters() {
        let grayscale = EnhancedBackdropGrayscale::ThreeQuarters;
        assert_eq!(grayscale.to_string(), "0.75");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_full() {
        let grayscale = EnhancedBackdropGrayscale::Full;
        assert_eq!(grayscale.to_string(), "1");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_none() {
        let hue_rotate = EnhancedBackdropHueRotate::None;
        assert_eq!(hue_rotate.to_string(), "0deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_fifteen() {
        let hue_rotate = EnhancedBackdropHueRotate::Fifteen;
        assert_eq!(hue_rotate.to_string(), "15deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_thirty() {
        let hue_rotate = EnhancedBackdropHueRotate::Thirty;
        assert_eq!(hue_rotate.to_string(), "30deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_sixty() {
        let hue_rotate = EnhancedBackdropHueRotate::Sixty;
        assert_eq!(hue_rotate.to_string(), "60deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_ninety() {
        let hue_rotate = EnhancedBackdropHueRotate::Ninety;
        assert_eq!(hue_rotate.to_string(), "90deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_one_hundred_eighty() {
        let hue_rotate = EnhancedBackdropHueRotate::OneHundredEighty;
        assert_eq!(hue_rotate.to_string(), "180deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_two_hundred_seventy() {
        let hue_rotate = EnhancedBackdropHueRotate::TwoHundredSeventy;
        assert_eq!(hue_rotate.to_string(), "270deg");
    }

    #[test]
    fn test_enhanced_backdrop_invert_none() {
        let invert = EnhancedBackdropInvert::None;
        assert_eq!(invert.to_string(), "0");
    }

    #[test]
    fn test_enhanced_backdrop_invert_quarter() {
        let invert = EnhancedBackdropInvert::Quarter;
        assert_eq!(invert.to_string(), "0.25");
    }

    #[test]
    fn test_enhanced_backdrop_invert_half() {
        let invert = EnhancedBackdropInvert::Half;
        assert_eq!(invert.to_string(), "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_invert_three_quarters() {
        let invert = EnhancedBackdropInvert::ThreeQuarters;
        assert_eq!(invert.to_string(), "0.75");
    }

    #[test]
    fn test_enhanced_backdrop_invert_full() {
        let invert = EnhancedBackdropInvert::Full;
        assert_eq!(invert.to_string(), "1");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_zero() {
        let opacity = EnhancedBackdropOpacity::Zero;
        assert_eq!(opacity.to_string(), "0");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_five() {
        let opacity = EnhancedBackdropOpacity::Five;
        assert_eq!(opacity.to_string(), "0.05");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_ten() {
        let opacity = EnhancedBackdropOpacity::Ten;
        assert_eq!(opacity.to_string(), "0.1");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_twenty() {
        let opacity = EnhancedBackdropOpacity::Twenty;
        assert_eq!(opacity.to_string(), "0.2");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_twenty_five() {
        let opacity = EnhancedBackdropOpacity::TwentyFive;
        assert_eq!(opacity.to_string(), "0.25");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_thirty() {
        let opacity = EnhancedBackdropOpacity::Thirty;
        assert_eq!(opacity.to_string(), "0.3");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_forty() {
        let opacity = EnhancedBackdropOpacity::Forty;
        assert_eq!(opacity.to_string(), "0.4");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_fifty() {
        let opacity = EnhancedBackdropOpacity::Fifty;
        assert_eq!(opacity.to_string(), "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_sixty() {
        let opacity = EnhancedBackdropOpacity::Sixty;
        assert_eq!(opacity.to_string(), "0.6");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_seventy() {
        let opacity = EnhancedBackdropOpacity::Seventy;
        assert_eq!(opacity.to_string(), "0.7");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_seventy_five() {
        let opacity = EnhancedBackdropOpacity::SeventyFive;
        assert_eq!(opacity.to_string(), "0.75");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_eighty() {
        let opacity = EnhancedBackdropOpacity::Eighty;
        assert_eq!(opacity.to_string(), "0.8");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_ninety() {
        let opacity = EnhancedBackdropOpacity::Ninety;
        assert_eq!(opacity.to_string(), "0.9");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_ninety_five() {
        let opacity = EnhancedBackdropOpacity::NinetyFive;
        assert_eq!(opacity.to_string(), "0.95");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_default() {
        let opacity = EnhancedBackdropOpacity::Default;
        assert_eq!(opacity.to_string(), "1");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_zero() {
        let saturate = EnhancedBackdropSaturate::Zero;
        assert_eq!(saturate.to_string(), "0");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_half() {
        let saturate = EnhancedBackdropSaturate::Half;
        assert_eq!(saturate.to_string(), "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_default() {
        let saturate = EnhancedBackdropSaturate::Default;
        assert_eq!(saturate.to_string(), "1");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_one_hundred_fifty() {
        let saturate = EnhancedBackdropSaturate::OneHundredFifty;
        assert_eq!(saturate.to_string(), "1.5");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_two_hundred() {
        let saturate = EnhancedBackdropSaturate::TwoHundred;
        assert_eq!(saturate.to_string(), "2");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_none() {
        let sepia = EnhancedBackdropSepia::None;
        assert_eq!(sepia.to_string(), "0");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_quarter() {
        let sepia = EnhancedBackdropSepia::Quarter;
        assert_eq!(sepia.to_string(), "0.25");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_half() {
        let sepia = EnhancedBackdropSepia::Half;
        assert_eq!(sepia.to_string(), "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_three_quarters() {
        let sepia = EnhancedBackdropSepia::ThreeQuarters;
        assert_eq!(sepia.to_string(), "0.75");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_full() {
        let sepia = EnhancedBackdropSepia::Full;
        assert_eq!(sepia.to_string(), "1");
    }

    #[test]
    fn test_enhanced_backdrop_blur_serialization() {
        let blur = EnhancedBackdropBlur::Lg;
        let serialized = serde_json::to_string(&blur).unwrap();
        let deserialized: EnhancedBackdropBlur = serde_json::from_str(&serialized).unwrap();
        assert_eq!(blur, deserialized);
    }

    #[test]
    fn test_enhanced_backdrop_blur_clone() {
        let blur = EnhancedBackdropBlur::Xl;
        let cloned = blur.clone();
        assert_eq!(blur, cloned);
    }

    #[test]
    fn test_enhanced_backdrop_blur_partial_eq() {
        let blur1 = EnhancedBackdropBlur::Sm;
        let blur2 = EnhancedBackdropBlur::Sm;
        let blur3 = EnhancedBackdropBlur::Lg;

        assert_eq!(blur1, blur2);
        assert_ne!(blur1, blur3);
    }
}

#[cfg(test)]
mod enhanced_backdrop_filters_integration_tests {
    use super::*;

    #[test]
    fn test_enhanced_backdrop_filters_with_class_builder() {
        let builder = ClassBuilder::new().backdrop_blur_sm();

        let class_set = builder.build();
        assert!(class_set.classes.contains("backdrop-blur-sm"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_with_other_utilities() {
        let builder = ClassBuilder::new()
            .backdrop_blur_sm()
            .class("text-blue-500")
            .class("font-bold");

        let class_set = builder.build();
        assert!(class_set.classes.contains("backdrop-blur-sm"));
        assert!(class_set.classes.contains("text-blue-500"));
        assert!(class_set.classes.contains("font-bold"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_responsive() {
        let builder = ClassBuilder::new()
            .backdrop_blur_sm()
            .responsive(Breakpoint::Md, "backdrop-blur-lg");

        let class_set = builder.build();
        assert!(class_set.classes.contains("backdrop-blur-sm"));
        assert!(class_set.responsive.contains_key(&Breakpoint::Md));
        assert!(class_set
            .responsive
            .get(&Breakpoint::Md)
            .unwrap()
            .contains("backdrop-blur-lg"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_conditional() {
        let builder = ClassBuilder::new()
            .backdrop_blur_sm()
            .conditional("hover", "backdrop-blur-lg");

        let class_set = builder.build();
        assert!(class_set.classes.contains("backdrop-blur-sm"));
        assert!(class_set.conditional.contains_key("hover"));
        assert!(class_set
            .conditional
            .get("hover")
            .unwrap()
            .contains("backdrop-blur-lg"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_custom_variant() {
        let builder = ClassBuilder::new()
            .backdrop_blur_sm()
            .custom_variant("dark", "backdrop-blur-lg");

        let class_set = builder.build();
        assert!(class_set.classes.contains("backdrop-blur-sm"));
        assert!(class_set.conditional.contains_key("dark"));
        assert!(class_set
            .conditional
            .get("dark")
            .unwrap()
            .contains("backdrop-blur-lg"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_multiple_filters() {
        let builder = ClassBuilder::new().backdrop_blur_sm().backdrop_blur_lg();

        let class_set = builder.build();
        assert!(class_set.classes.contains("backdrop-blur-sm"));
        assert!(class_set.classes.contains("backdrop-blur-lg"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_build_string() {
        let classes = ClassBuilder::new()
            .backdrop_blur_sm()
            .class("text-blue-500")
            .build_string();

        assert!(classes.contains("backdrop-blur-sm"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_css_classes() {
        let class_set = ClassBuilder::new()
            .backdrop_blur_lg()
            .class("font-bold")
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("backdrop-blur-lg"));
        assert!(css_classes.contains("font-bold"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_comprehensive_usage() {
        let class_set = ClassBuilder::new()
            .backdrop_blur_sm()
            .backdrop_blur_lg()
            .backdrop_blur_3xl()
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("backdrop-blur-sm"));
        assert!(css_classes.contains("backdrop-blur-lg"));
        assert!(css_classes.contains("backdrop-blur-3xl"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_custom_usage() {
        let class_set = ClassBuilder::new()
            .backdrop_blur_custom(EnhancedBackdropBlur::Xl)
            .backdrop_blur_custom(EnhancedBackdropBlur::Xl3)
            .build();

        let css_classes = class_set.to_css_classes();
        assert!(css_classes.contains("backdrop-blur-xl"));
        assert!(css_classes.contains("backdrop-blur-3xl"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_all_variants() {
        let class_set = ClassBuilder::new()
            .backdrop_blur_none()
            .backdrop_blur_sm()
            .backdrop_blur()
            .backdrop_blur_md()
            .backdrop_blur_lg()
            .backdrop_blur_xl()
            .backdrop_blur_2xl()
            .backdrop_blur_3xl()
            .build();

        let css_classes = class_set.to_css_classes();

        // Test that all backdrop blur utilities are present
        assert!(css_classes.contains("backdrop-blur-none"));
        assert!(css_classes.contains("backdrop-blur-sm"));
        assert!(css_classes.contains("backdrop-blur"));
        assert!(css_classes.contains("backdrop-blur-md"));
        assert!(css_classes.contains("backdrop-blur-lg"));
        assert!(css_classes.contains("backdrop-blur-xl"));
        assert!(css_classes.contains("backdrop-blur-2xl"));
        assert!(css_classes.contains("backdrop-blur-3xl"));
    }
}
