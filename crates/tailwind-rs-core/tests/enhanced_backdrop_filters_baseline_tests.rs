use tailwind_rs_core::utilities::enhanced_backdrop_filters::*;
use tailwind_rs_core::Breakpoint;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod enhanced_backdrop_filters_baseline_tests {
    use super::*;

    #[test]
    fn test_enhanced_backdrop_filters_css_output_baseline() {
        let builder = ClassBuilder::new().backdrop_blur_sm().backdrop_blur_lg();

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain both backdrop blur classes
        assert!(classes.contains("backdrop-blur-sm"));
        assert!(classes.contains("backdrop-blur-lg"));
    }

    #[test]
    fn test_enhanced_backdrop_filters_class_generation_baseline() {
        let builder = ClassBuilder::new().backdrop_blur_sm().backdrop_blur_3xl();

        let class_set = builder.build();
        let classes = class_set.to_css_classes();

        // Baseline: Should contain both backdrop blur classes
        assert!(classes.contains("backdrop-blur-sm"));
        assert!(classes.contains("backdrop-blur-3xl"));
    }

    #[test]
    fn test_enhanced_backdrop_blur_none_baseline() {
        let blur = EnhancedBackdropBlur::None;
        let string_value = blur.to_string();

        // Baseline string output
        assert_eq!(string_value, "none");
    }

    #[test]
    fn test_enhanced_backdrop_blur_sm_baseline() {
        let blur = EnhancedBackdropBlur::Sm;
        let string_value = blur.to_string();

        // Baseline string output
        assert_eq!(string_value, "4px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_default_baseline() {
        let blur = EnhancedBackdropBlur::Default;
        let string_value = blur.to_string();

        // Baseline string output
        assert_eq!(string_value, "8px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_md_baseline() {
        let blur = EnhancedBackdropBlur::Md;
        let string_value = blur.to_string();

        // Baseline string output
        assert_eq!(string_value, "12px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_lg_baseline() {
        let blur = EnhancedBackdropBlur::Lg;
        let string_value = blur.to_string();

        // Baseline string output
        assert_eq!(string_value, "16px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_xl_baseline() {
        let blur = EnhancedBackdropBlur::Xl;
        let string_value = blur.to_string();

        // Baseline string output
        assert_eq!(string_value, "24px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_2xl_baseline() {
        let blur = EnhancedBackdropBlur::Xl2;
        let string_value = blur.to_string();

        // Baseline string output
        assert_eq!(string_value, "40px");
    }

    #[test]
    fn test_enhanced_backdrop_blur_3xl_baseline() {
        let blur = EnhancedBackdropBlur::Xl3;
        let string_value = blur.to_string();

        // Baseline string output
        assert_eq!(string_value, "64px");
    }

    #[test]
    fn test_enhanced_backdrop_brightness_zero_baseline() {
        let brightness = EnhancedBackdropBrightness::Zero;
        let string_value = brightness.to_string();

        // Baseline string output
        assert_eq!(string_value, "0");
    }

    #[test]
    fn test_enhanced_backdrop_brightness_half_baseline() {
        let brightness = EnhancedBackdropBrightness::Half;
        let string_value = brightness.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_brightness_default_baseline() {
        let brightness = EnhancedBackdropBrightness::Default;
        let string_value = brightness.to_string();

        // Baseline string output
        assert_eq!(string_value, "1");
    }

    #[test]
    fn test_enhanced_backdrop_brightness_one_hundred_fifty_baseline() {
        let brightness = EnhancedBackdropBrightness::OneHundredFifty;
        let string_value = brightness.to_string();

        // Baseline string output
        assert_eq!(string_value, "1.5");
    }

    #[test]
    fn test_enhanced_backdrop_brightness_two_hundred_baseline() {
        let brightness = EnhancedBackdropBrightness::TwoHundred;
        let string_value = brightness.to_string();

        // Baseline string output
        assert_eq!(string_value, "2");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_zero_baseline() {
        let contrast = EnhancedBackdropContrast::Zero;
        let string_value = contrast.to_string();

        // Baseline string output
        assert_eq!(string_value, "0");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_half_baseline() {
        let contrast = EnhancedBackdropContrast::Half;
        let string_value = contrast.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_default_baseline() {
        let contrast = EnhancedBackdropContrast::Default;
        let string_value = contrast.to_string();

        // Baseline string output
        assert_eq!(string_value, "1");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_one_hundred_fifty_baseline() {
        let contrast = EnhancedBackdropContrast::OneHundredFifty;
        let string_value = contrast.to_string();

        // Baseline string output
        assert_eq!(string_value, "1.5");
    }

    #[test]
    fn test_enhanced_backdrop_contrast_two_hundred_baseline() {
        let contrast = EnhancedBackdropContrast::TwoHundred;
        let string_value = contrast.to_string();

        // Baseline string output
        assert_eq!(string_value, "2");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_none_baseline() {
        let grayscale = EnhancedBackdropGrayscale::None;
        let string_value = grayscale.to_string();

        // Baseline string output
        assert_eq!(string_value, "0");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_quarter_baseline() {
        let grayscale = EnhancedBackdropGrayscale::Quarter;
        let string_value = grayscale.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.25");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_half_baseline() {
        let grayscale = EnhancedBackdropGrayscale::Half;
        let string_value = grayscale.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_three_quarters_baseline() {
        let grayscale = EnhancedBackdropGrayscale::ThreeQuarters;
        let string_value = grayscale.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.75");
    }

    #[test]
    fn test_enhanced_backdrop_grayscale_full_baseline() {
        let grayscale = EnhancedBackdropGrayscale::Full;
        let string_value = grayscale.to_string();

        // Baseline string output
        assert_eq!(string_value, "1");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_none_baseline() {
        let hue_rotate = EnhancedBackdropHueRotate::None;
        let string_value = hue_rotate.to_string();

        // Baseline string output
        assert_eq!(string_value, "0deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_fifteen_baseline() {
        let hue_rotate = EnhancedBackdropHueRotate::Fifteen;
        let string_value = hue_rotate.to_string();

        // Baseline string output
        assert_eq!(string_value, "15deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_thirty_baseline() {
        let hue_rotate = EnhancedBackdropHueRotate::Thirty;
        let string_value = hue_rotate.to_string();

        // Baseline string output
        assert_eq!(string_value, "30deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_sixty_baseline() {
        let hue_rotate = EnhancedBackdropHueRotate::Sixty;
        let string_value = hue_rotate.to_string();

        // Baseline string output
        assert_eq!(string_value, "60deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_ninety_baseline() {
        let hue_rotate = EnhancedBackdropHueRotate::Ninety;
        let string_value = hue_rotate.to_string();

        // Baseline string output
        assert_eq!(string_value, "90deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_one_hundred_eighty_baseline() {
        let hue_rotate = EnhancedBackdropHueRotate::OneHundredEighty;
        let string_value = hue_rotate.to_string();

        // Baseline string output
        assert_eq!(string_value, "180deg");
    }

    #[test]
    fn test_enhanced_backdrop_hue_rotate_two_hundred_seventy_baseline() {
        let hue_rotate = EnhancedBackdropHueRotate::TwoHundredSeventy;
        let string_value = hue_rotate.to_string();

        // Baseline string output
        assert_eq!(string_value, "270deg");
    }

    #[test]
    fn test_enhanced_backdrop_invert_none_baseline() {
        let invert = EnhancedBackdropInvert::None;
        let string_value = invert.to_string();

        // Baseline string output
        assert_eq!(string_value, "0");
    }

    #[test]
    fn test_enhanced_backdrop_invert_quarter_baseline() {
        let invert = EnhancedBackdropInvert::Quarter;
        let string_value = invert.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.25");
    }

    #[test]
    fn test_enhanced_backdrop_invert_half_baseline() {
        let invert = EnhancedBackdropInvert::Half;
        let string_value = invert.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_invert_three_quarters_baseline() {
        let invert = EnhancedBackdropInvert::ThreeQuarters;
        let string_value = invert.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.75");
    }

    #[test]
    fn test_enhanced_backdrop_invert_full_baseline() {
        let invert = EnhancedBackdropInvert::Full;
        let string_value = invert.to_string();

        // Baseline string output
        assert_eq!(string_value, "1");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_zero_baseline() {
        let opacity = EnhancedBackdropOpacity::Zero;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_five_baseline() {
        let opacity = EnhancedBackdropOpacity::Five;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.05");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_ten_baseline() {
        let opacity = EnhancedBackdropOpacity::Ten;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.1");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_twenty_baseline() {
        let opacity = EnhancedBackdropOpacity::Twenty;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.2");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_twenty_five_baseline() {
        let opacity = EnhancedBackdropOpacity::TwentyFive;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.25");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_thirty_baseline() {
        let opacity = EnhancedBackdropOpacity::Thirty;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.3");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_forty_baseline() {
        let opacity = EnhancedBackdropOpacity::Forty;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.4");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_fifty_baseline() {
        let opacity = EnhancedBackdropOpacity::Fifty;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_sixty_baseline() {
        let opacity = EnhancedBackdropOpacity::Sixty;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.6");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_seventy_baseline() {
        let opacity = EnhancedBackdropOpacity::Seventy;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.7");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_seventy_five_baseline() {
        let opacity = EnhancedBackdropOpacity::SeventyFive;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.75");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_eighty_baseline() {
        let opacity = EnhancedBackdropOpacity::Eighty;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.8");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_ninety_baseline() {
        let opacity = EnhancedBackdropOpacity::Ninety;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.9");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_ninety_five_baseline() {
        let opacity = EnhancedBackdropOpacity::NinetyFive;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.95");
    }

    #[test]
    fn test_enhanced_backdrop_opacity_default_baseline() {
        let opacity = EnhancedBackdropOpacity::Default;
        let string_value = opacity.to_string();

        // Baseline string output
        assert_eq!(string_value, "1");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_zero_baseline() {
        let saturate = EnhancedBackdropSaturate::Zero;
        let string_value = saturate.to_string();

        // Baseline string output
        assert_eq!(string_value, "0");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_half_baseline() {
        let saturate = EnhancedBackdropSaturate::Half;
        let string_value = saturate.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_default_baseline() {
        let saturate = EnhancedBackdropSaturate::Default;
        let string_value = saturate.to_string();

        // Baseline string output
        assert_eq!(string_value, "1");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_one_hundred_fifty_baseline() {
        let saturate = EnhancedBackdropSaturate::OneHundredFifty;
        let string_value = saturate.to_string();

        // Baseline string output
        assert_eq!(string_value, "1.5");
    }

    #[test]
    fn test_enhanced_backdrop_saturate_two_hundred_baseline() {
        let saturate = EnhancedBackdropSaturate::TwoHundred;
        let string_value = saturate.to_string();

        // Baseline string output
        assert_eq!(string_value, "2");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_none_baseline() {
        let sepia = EnhancedBackdropSepia::None;
        let string_value = sepia.to_string();

        // Baseline string output
        assert_eq!(string_value, "0");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_quarter_baseline() {
        let sepia = EnhancedBackdropSepia::Quarter;
        let string_value = sepia.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.25");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_half_baseline() {
        let sepia = EnhancedBackdropSepia::Half;
        let string_value = sepia.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.5");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_three_quarters_baseline() {
        let sepia = EnhancedBackdropSepia::ThreeQuarters;
        let string_value = sepia.to_string();

        // Baseline string output
        assert_eq!(string_value, "0.75");
    }

    #[test]
    fn test_enhanced_backdrop_sepia_full_baseline() {
        let sepia = EnhancedBackdropSepia::Full;
        let string_value = sepia.to_string();

        // Baseline string output
        assert_eq!(string_value, "1");
    }

    #[test]
    fn test_enhanced_backdrop_filters_serialization_baseline() {
        let blur = EnhancedBackdropBlur::Lg;
        let serialized = serde_json::to_string(&blur).unwrap();

        // Baseline: Should serialize to JSON
        assert!(serialized.contains("Lg"));

        // Should deserialize back to the same value
        let deserialized: EnhancedBackdropBlur = serde_json::from_str(&serialized).unwrap();
        assert_eq!(blur, deserialized);
    }

    #[test]
    fn test_enhanced_backdrop_filters_equality_baseline() {
        let blur1 = EnhancedBackdropBlur::Sm;
        let blur2 = EnhancedBackdropBlur::Sm;
        let blur3 = EnhancedBackdropBlur::Lg;

        // Baseline: Same variants should be equal
        assert_eq!(blur1, blur2);
        assert_ne!(blur1, blur3);
    }

    #[test]
    fn test_enhanced_backdrop_filters_clone_baseline() {
        let blur = EnhancedBackdropBlur::Xl;
        let cloned = blur.clone();

        // Baseline: Cloned blur should be equal to original
        assert_eq!(blur, cloned);
    }

    #[test]
    fn test_enhanced_backdrop_filters_complex_builder_baseline() {
        let class_set = ClassBuilder::new()
            .backdrop_blur_sm()
            .backdrop_blur_lg()
            .backdrop_blur_3xl()
            .class("text-blue-500")
            .class("font-bold")
            .responsive(Breakpoint::Md, "backdrop-blur-xl")
            .conditional("hover", "backdrop-blur-none")
            .build();

        let classes = class_set.to_css_classes();

        // Baseline: Should contain expected classes
        assert!(classes.contains("backdrop-blur-sm"));
        assert!(classes.contains("backdrop-blur-lg"));
        assert!(classes.contains("backdrop-blur-3xl"));
        assert!(classes.contains("text-blue-500"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("md:backdrop-blur-xl"));
        assert!(classes.contains("hover:backdrop-blur-none"));
    }
}
