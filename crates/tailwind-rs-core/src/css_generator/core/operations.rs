//! Core CSS generation operations
//! Contains the main business logic for CSS generation

use super::super::CssGenerator;
use crate::error::Result;
use crate::css_generator::types::{CssRule, CssProperty};
use crate::css_generator::parsers::UtilityParser;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Comprehensive parser registry for all utility parsers
pub struct ParserRegistry {
    parsers: Vec<Box<dyn UtilityParser>>,
}

impl std::fmt::Debug for ParserRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParserRegistry")
            .field("parser_count", &self.parsers.len())
            .finish()
    }
}

impl ParserRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            parsers: Vec::new(),
        };
        registry.register_all_parsers();
        registry.sort_by_priority();
        registry
    }

    fn register_all_parsers(&mut self) {
        // PHASE 1: Register ALL available parsers (Phase 1 of Tailwind CSS v4.1.13 alignment)

        // Core Layout & Display
        self.parsers.push(Box::new(crate::css_generator::parsers::layout::LayoutParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::layout_utilities::LayoutUtilitiesParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::position::PositionParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::positioning::PositioningParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::box_utilities::BoxUtilitiesParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::visibility::VisibilityParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::z_index::ZIndexParser::default()));

        // Spacing & Sizing
        self.parsers.push(Box::new(crate::css_generator::parsers::spacing::SpacingParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::spacing_advanced::AdvancedSpacingParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::gap::GapParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::sizing::SizingParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::aspect_ratio::AspectRatioParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::field_sizing::FieldSizingParser::new()));

        // Flexbox & Grid
        self.parsers.push(Box::new(crate::css_generator::parsers::flexbox::FlexboxParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::flex::FlexParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::flex_direction::FlexDirectionParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::flex_wrap::FlexWrapParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::flex_grow::FlexGrowParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::flex_shrink::FlexShrinkParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::flex_basis::FlexBasisParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::order::OrderParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::grid::GridParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::grid_advanced::AdvancedGridParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::grid_template_columns::GridTemplateColumnsParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::grid_template_rows::GridTemplateRowsParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::grid_column::GridColumnParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::grid_row::GridRowParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::grid_auto_columns::GridAutoColumnsParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::grid_auto_flow::GridAutoFlowParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::grid_auto_rows::GridAutoRowsParser::default()));

        // Alignment & Justification
        self.parsers.push(Box::new(crate::css_generator::parsers::align_content::AlignContentParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::align_items::AlignItemsParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::align_self::AlignSelfParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::justify_content::JustifyContentParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::justify_items::JustifyItemsParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::justify_self::JustifySelfParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::place_content::PlaceContentParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::place_items::PlaceItemsParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::place_self::PlaceSelfParser::default()));

        // Colors & Background
        self.parsers.push(Box::new(crate::css_generator::parsers::color::ColorParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::colors_advanced::AdvancedColorParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::accent_color::AccentColorParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::BackgroundParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::background_properties::BackgroundPropertiesParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::gradients::GradientParser::default()));

        // Typography
        self.parsers.push(Box::new(crate::css_generator::parsers::typography::TypographyParser::default()));

        // Borders & Rings
        self.parsers.push(Box::new(crate::css_generator::parsers::borders::BorderParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::borders_advanced::AdvancedBorderParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::OutlineParser::new()));
        self.parsers.push(Box::new(crate::css_generator::parsers::border_radius_parser::BorderRadiusParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::divide::DivideParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::rings::RingParser::default()));

        // Effects & Shadows
        self.parsers.push(Box::new(crate::css_generator::parsers::shadows::ShadowParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::effects_modules::EffectsParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::effects_utilities_modules::EffectsParser::new()));
        self.parsers.push(Box::new(crate::css_generator::parsers::backdrop_filter_utilities::BackdropFilterUtilitiesParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::filter_utilities_modules::FilterUtilitiesParser::default()));

        // Transforms & Animations
        self.parsers.push(Box::new(crate::css_generator::parsers::TransformParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::basic_transforms::BasicTransformsParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::scale_parser::ScaleParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::fractional_transforms::FractionalTransformsParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::animations::AnimationParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::transitions::TransitionParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::transition_properties::TransitionPropertiesParser::default()));

        // Interactivity
        self.parsers.push(Box::new(crate::css_generator::parsers::interactive::InteractiveParser::default()));

        // Tables & Layout
        self.parsers.push(Box::new(crate::css_generator::parsers::table::TableParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::columns::ColumnsParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::break_control::BreakControlParser::default()));

        // Overflow & Scrolling
        self.parsers.push(Box::new(crate::css_generator::parsers::overflow::OverflowParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::overscroll::OverscrollParser::default()));

        // Masks & Clipping
        self.parsers.push(Box::new(crate::css_generator::parsers::mask_utilities::MaskUtilitiesParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::mask_image_parser::MaskImageParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::mask_properties_parser::MaskPropertiesParser::default()));

        // Object & Image
        self.parsers.push(Box::new(crate::css_generator::parsers::object_fit::ObjectFitParser::default()));

        // Prose & Content
        self.parsers.push(Box::new(crate::css_generator::parsers::prose::ProseParser::default()));

        // Data Attributes & Arbitrary
        self.parsers.push(Box::new(crate::css_generator::parsers::data_attributes::DataAttributeParser::default()));
        self.parsers.push(Box::new(crate::css_generator::parsers::arbitrary::ArbitraryParser::default()));

        // Accessibility
        self.parsers.push(Box::new(crate::css_generator::parsers::accessibility::AccessibilityParser::default()));

        // SVG
        self.parsers.push(Box::new(crate::css_generator::parsers::svg::SvgParser::default()));

        // Inset utilities (top, right, bottom, left)
        self.parsers.push(Box::new(crate::css_generator::parsers::inset_utilities::InsetParser::default()));
    }

    fn sort_by_priority(&mut self) {
        self.parsers.sort_by(|a, b| b.get_priority().cmp(&a.get_priority()));
    }

    pub fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        for (i, parser) in self.parsers.iter().enumerate() {
            // Check if this parser supports this class pattern
            let patterns = parser.get_supported_patterns();
            let supports_class = patterns.iter().any(|pattern| {
                if pattern.ends_with("-*") {
                    // Pattern like "p-*" matches "p-4", "px-*", etc.
                    let prefix = &pattern[..pattern.len() - 2];
                    class.starts_with(prefix) && class.len() > prefix.len()
                } else if pattern.ends_with("*") {
                    // Pattern like "animate*" matches "animate-spin", etc.
                    let prefix = &pattern[..pattern.len() - 1];
                    class.starts_with(prefix)
                } else if pattern.ends_with("-") {
                    // Pattern like "animate-" matches "animate-pulse", "animate-spin", etc.
                    class.starts_with(pattern)
                } else {
                    // Exact match
                    class == *pattern
                }
            });

            if supports_class {
                if let Some(properties) = parser.parse_class(class) {
                    return Some(properties);
                }
            }
        }
        None
    }
}

/// Processing context that provides safe access to generator components
/// without creating borrow conflicts
pub struct ProcessingContext<'a> {
    pub variant_parser: &'a super::super::variants::VariantParser,
    pub color_cache: &'a mut super::super::color_cache::ColorCache,
    pub rule_cache: &'a mut super::super::caching::rule_cache::RuleCache,
    pub parser_registry: &'a ParserRegistry,
}

impl<'a> ProcessingContext<'a> {
    /// Process a class using the provided context
    pub fn process_class(&mut self, class: &str) -> Result<Vec<CssProperty>> {
        // Use the parser registry to find the right parser for this class
        if let Some(properties) = self.parser_registry.parse_class(class) {
            Ok(properties)
        } else {
            // For unknown classes, return empty (they'll be handled by fallback CSS)
            Ok(vec![])
        }
    }

}

/// Core operations trait for CSS generation
pub trait CssGeneratorOperations {
    /// Process multiple classes for an element (element-based processing)
    fn process_element_classes(&mut self, classes: &[&str]) -> String;

    /// Generate individual CSS rule for a class
    fn generate_individual_css_rule(&mut self, class: &str) -> Result<CssRule>;

    /// Generate minified CSS output
    fn generate_minified_css(&self) -> String;
}

/// Helper trait for internal operations
pub trait CssGeneratorInternalOps {
    /// Convert class string to CSS properties
    fn class_to_properties(&mut self, class: &str) -> Result<Vec<CssProperty>>;

    /// Calculate specificity for CSS rules
    fn calculate_specificity(&self, variants: &[String]) -> u32;

    /// Handle gradient hover rules (special case)
    fn generate_gradient_hover_rules(&mut self, classes: &[&str]) -> Option<String>;

    /// Get all rules from the generator
    fn get_all_rules(&self) -> &HashMap<String, CssRule>;

    /// Process element classes without gradient handling (helper)
    fn process_element_classes_basic(&mut self, classes: &[&str]) -> String;
}

impl CssGeneratorOperations for CssGenerator {
    fn process_element_classes(&mut self, classes: &[&str]) -> String {
        use std::collections::HashMap;

        let mut base_rules = Vec::new();
        let mut responsive_rules: HashMap<String, Vec<CssRule>> = HashMap::new();

        // Check if this element has transform classes
        let has_transforms = classes.iter().any(|class| {
            let (_variants, base_class) = self.variant_parser.parse_variants(class);
            base_class == "transform" ||
            base_class.starts_with("translate-") ||
            base_class.starts_with("scale-") ||
            base_class.starts_with("rotate-") ||
            base_class.starts_with("skew-") ||
            base_class.starts_with("origin-")
        });

        // Handle gradient classes first (they need compound rules)
        if classes.iter().any(|class| self.class_processor.is_gradient_class(class)) {
            if let Some(gradient_css) = self.generate_gradient_hover_rules(classes) {
                return gradient_css + &self.process_element_classes_basic(
                    &classes.iter().filter(|class| !self.class_processor.is_gradient_class(class)).cloned().collect::<Vec<_>>()
                );
            }
        }

        // Process each class individually and collect rules
        for class in classes {
            let (variants, base_class) = self.variant_parser.parse_variants(class);

            // Skip gradient stop classes with hover variants (handled in compound rules)
            if variants.contains(&"hover".to_string()) &&
              (base_class.starts_with("from-") || base_class.starts_with("via-") || base_class.starts_with("to-")) {
               continue;
            }

            // Generate CSS rule for this class using the new architecture
            if let Ok(rule) = self.generate_individual_css_rule(class) {
                if let Some(ref media_query) = rule.media_query {
                    responsive_rules
                        .entry(media_query.clone())
                        .or_default()
                        .push(rule);
                } else {
                    base_rules.push(rule);
                }
            }
        }

        // Generate final CSS with proper media query organization
        let mut css = self.css_output.generate_css(base_rules, responsive_rules);

        // Add transform CSS if this element uses transforms
        if has_transforms {
            if let Some(transform_css) = self.class_processor.generate_transform_css() {
                css = transform_css + &css;
            }
        }

        css
    }

    fn generate_individual_css_rule(&mut self, class: &str) -> Result<CssRule> {
        // Parse variants and base class
        let (variants, base_class) = self.variant_parser.parse_variants(class);

        // Create processing context to avoid borrow conflicts
        let mut context = ProcessingContext {
            variant_parser: &self.variant_parser,
            color_cache: &mut self.color_cache,
            rule_cache: &mut self.rule_cache,
            parser_registry: &self.parser_registry,
        };

        // Get CSS properties for the base class using the processing context
        let properties = context.process_class(&base_class)?;

        // Build CSS selector with variants
        let selector = self.variant_processor.build_css_selector(&base_class, &variants)?;

        // Get media query if this is a responsive variant
        let media_query = self.variant_parser.get_variant_media_query(&variants);

        // Calculate specificity
        let specificity = <Self as CssGeneratorInternalOps>::calculate_specificity(self, &variants);

        Ok(CssRule {
            selector,
            properties,
            media_query,
            specificity,
        })
    }

    fn generate_minified_css(&self) -> String {
        // Get all rules from the generator
        let rules = self.get_all_rules();

        // Generate minified CSS
        self.css_output.generate_minified_css(rules)
    }
}

impl CssGeneratorInternalOps for CssGenerator {
    fn class_to_properties(&mut self, class: &str) -> Result<Vec<super::super::types::CssProperty>> {
        let mut context = ProcessingContext {
            variant_parser: &self.variant_parser,
            color_cache: &mut self.color_cache,
            rule_cache: &mut self.rule_cache,
            parser_registry: &self.parser_registry,
        };
        context.process_class(class)
    }

    fn calculate_specificity(&self, variants: &[String]) -> u32 {
        let mut specificity = 10; // Base specificity

        for variant in variants {
            match variant.as_str() {
                "hover" | "focus" | "active" => specificity += 10,
                "sm" | "md" | "lg" | "xl" | "2xl" => specificity += 100, // Responsive
                "dark" => specificity += 10,
                "group-hover" | "peer-hover" => specificity += 20,
                _ if variant.starts_with("@container-") => specificity += 100, // Container queries
                _ => specificity += 1,
            }
        }

        specificity
    }

    fn generate_gradient_hover_rules(&mut self, classes: &[&str]) -> Option<String> {
        // Extract gradient classes
        let gradient_stops: Vec<_> = classes.iter()
            .filter(|class| self.class_processor.is_gradient_class(class))
            .collect();

        if gradient_stops.is_empty() {
            return None;
        }

        // Group by hover variants
        let mut base_gradient = Vec::new();
        let mut hover_gradient = Vec::new();

        for &class in gradient_stops {
            let (variants, base_class) = self.variant_parser.parse_variants(class);

            if variants.contains(&"hover".to_string()) {
                if let Some(color) = self.class_processor.extract_gradient_color(class, &base_class) {
                    hover_gradient.push((base_class, color));
                }
            } else {
                if let Some(color) = self.class_processor.extract_gradient_color(class, &base_class) {
                    base_gradient.push((base_class, color));
                }
            }
        }

        if hover_gradient.is_empty() {
            return None;
        }

        // Generate compound CSS
        let mut css = String::new();

        // Base gradient
        if !base_gradient.is_empty() {
            css.push_str(&self.generate_gradient_css(&base_gradient));
        }

        // Hover gradient
        if !hover_gradient.is_empty() {
            css.push_str(":hover {\n");
            css.push_str(&self.generate_gradient_css(&hover_gradient));
            css.push_str("}\n");
        }

        Some(css)
    }

    fn get_all_rules(&self) -> &HashMap<String, CssRule> {
        // This would need to be implemented based on how rules are stored
        // For now, return empty hashmap
        static EMPTY_RULES: LazyLock<HashMap<String, CssRule>> = LazyLock::new(|| HashMap::new());
        &EMPTY_RULES
    }

    fn process_element_classes_basic(&mut self, classes: &[&str]) -> String {
        // Simplified version without gradient handling
        classes.iter()
            .filter_map(|&class| {
                self.generate_individual_css_rule(class).ok()
                    .map(|rule| format!("{} {{\n{}\n}}\n", rule.selector,
                        rule.properties.iter()
                            .map(|p| format!("  {}: {};", p.name, p.value))
                            .collect::<Vec<_>>()
                            .join("\n")))
            })
            .collect()
    }
}

impl CssGenerator {
    /// Convert class string to CSS properties
    pub(crate) fn class_to_properties(&mut self, class: &str) -> Result<Vec<super::super::types::CssProperty>> {
        let mut context = ProcessingContext {
            variant_parser: &self.variant_parser,
            color_cache: &mut self.color_cache,
            rule_cache: &mut self.rule_cache,
            parser_registry: &self.parser_registry,
        };
        context.process_class(class)
    }

    /// Calculate specificity for CSS rules
    pub(crate) fn calculate_specificity(&self, variants: &[String]) -> u32 {
        let mut specificity = 10; // Base specificity

        for variant in variants {
            match variant.as_str() {
                "hover" | "focus" | "active" => specificity += 10,
                "sm" | "md" | "lg" | "xl" | "2xl" => specificity += 100, // Responsive
                "dark" => specificity += 10,
                "group-hover" | "peer-hover" => specificity += 20,
                _ if variant.starts_with("@container-") => specificity += 100, // Container queries
                _ => specificity += 1,
            }
        }

        specificity
    }

    /// Handle gradient hover rules (special case)
    pub(crate) fn generate_gradient_hover_rules(&mut self, classes: &[&str]) -> Option<String> {
        // Extract gradient classes
        let gradient_stops: Vec<_> = classes.iter()
            .filter(|class| self.class_processor.is_gradient_class(class))
            .collect();

        if gradient_stops.is_empty() {
            return None;
        }

        // Group by hover variants
        let mut base_gradient = Vec::new();
        let mut hover_gradient = Vec::new();

        for &class in gradient_stops {
            let (variants, base_class) = self.variant_parser.parse_variants(class);

            if variants.contains(&"hover".to_string()) {
                if let Some(color) = self.class_processor.extract_gradient_color(class, &base_class) {
                    hover_gradient.push((base_class, color));
                }
            } else {
                if let Some(color) = self.class_processor.extract_gradient_color(class, &base_class) {
                    base_gradient.push((base_class, color));
                }
            }
        }

        if hover_gradient.is_empty() {
            return None;
        }

        // Generate compound CSS
        let mut css = String::new();

        // Base gradient
        if !base_gradient.is_empty() {
            css.push_str(&self.generate_gradient_css(&base_gradient));
        }

        // Hover gradient
        if !hover_gradient.is_empty() {
            css.push_str(&format!(":hover {{\n"));
            css.push_str(&self.generate_gradient_css(&hover_gradient));
            css.push_str("}\n");
        }

        Some(css)
    }

    /// Generate gradient CSS from stops
    fn generate_gradient_css(&self, stops: &[(String, String)]) -> String {
        let mut background_image = "background-image: linear-gradient(to right".to_string();

        for (stop_type, color) in stops {
            if stop_type.starts_with("from-") {
                background_image.push_str(&format!(", {} 0%", color));
            } else if stop_type.starts_with("via-") {
                background_image.push_str(&format!(", {} 50%", color));
            } else if stop_type.starts_with("to-") {
                background_image.push_str(&format!(", {} 100%", color));
            }
        }

        background_image.push_str(");\n");
        background_image
    }

    /// Get all rules from the generator
    fn get_all_rules(&self) -> &HashMap<String, CssRule> {
        // This would need to be implemented based on how rules are stored
        // For now, return empty hashmap
        &self.rules
    }

    /// Process element classes without gradient handling (helper)
    fn process_element_classes_basic(&mut self, classes: &[&str]) -> String {
        // Simplified version without gradient handling
        classes.iter()
            .filter_map(|&class| {
                self.generate_individual_css_rule(class).ok()
                    .map(|rule| format!("{} {{\n{}\n}}\n", rule.selector,
                        rule.properties.iter()
                            .map(|p| format!("  {}: {};", p.name, p.value))
                            .collect::<Vec<_>>()
                            .join("\n")))
            })
            .collect()
    }

}
