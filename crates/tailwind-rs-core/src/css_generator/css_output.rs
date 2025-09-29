//! CSS output generation
//!
//! This module handles converting CSS rules into actual CSS output.

use super::types::CssRule;
use std::collections::HashMap;

/// CSS output generator
pub struct CssOutputGenerator;

impl CssOutputGenerator {
    /// Generate CSS from a collection of rules
    pub fn generate_css(
        rules: &HashMap<String, CssRule>,
        custom_properties: &HashMap<String, String>,
    ) -> String {
        let mut css = String::new();

        // Add keyframes for animations
        css.push_str(&Self::generate_keyframes());

        // Add custom properties
        if !custom_properties.is_empty() {
            css.push_str(":root {\n");
            for (name, value) in custom_properties {
                css.push_str(&format!("  --{}: {};\n", name, value));
            }
            css.push_str("}\n\n");
        }

        // Group rules by media query
        let mut base_rules = Vec::new();
        let mut responsive_rules: HashMap<String, Vec<&CssRule>> = HashMap::new();

        for rule in rules.values() {
            if let Some(ref media_query) = rule.media_query {
                responsive_rules
                    .entry(media_query.clone())
                    .or_default()
                    .push(rule);
            } else {
                base_rules.push(rule);
            }
        }

        // Generate base rules
        for rule in base_rules {
            css.push_str(&Self::rule_to_css(rule));
        }

        // Generate responsive rules
        for (media_query, rules) in responsive_rules {
            css.push_str(&format!("@media {} {{\n", media_query));
            for rule in rules {
                css.push_str(&Self::rule_to_css(rule));
            }
            css.push_str("}\n\n");
        }

        css
    }

    /// Generate keyframes for animations
    fn generate_keyframes() -> String {
        let keyframes = r#"@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes ping {
  75%, 100% {
    transform: scale(2);
    opacity: 0;
  }
}

@keyframes pulse {
  50% {
    opacity: 0.5;
  }
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(-25%);
    animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
  }
  50% {
    transform: none;
    animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-20px);
  }
}

@keyframes twinkle {
  0%, 100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.3;
    transform: scale(0.8);
  }
}

@keyframes rainbow {
  0% {
    filter: hue-rotate(0deg);
  }
  100% {
    filter: hue-rotate(360deg);
  }
}

@keyframes shimmer {
  0% {
    background-position: -200% 0;
  }
  100% {
    background-position: 200% 0;
  }
}

@keyframes drift {
  0%, 100% {
    transform: translateX(0px) translateY(0px);
  }
  25% {
    transform: translateX(10px) translateY(-5px);
  }
  50% {
    transform: translateX(-5px) translateY(-10px);
  }
  75% {
    transform: translateX(-10px) translateY(5px);
  }
}

@keyframes glow {
  0% {
    box-shadow: 0 0 5px rgba(255, 255, 255, 0.5);
  }
  100% {
    box-shadow: 0 0 20px rgba(255, 255, 255, 0.8), 0 0 30px rgba(255, 255, 255, 0.6);
  }
}

"#;
        keyframes.to_string()
    }

    /// Convert a CSS rule to CSS string
    fn rule_to_css(rule: &CssRule) -> String {
        let mut css = String::new();
        css.push_str(&format!("{} {{\n", rule.selector));

        for property in &rule.properties {
            let important = if property.important {
                " !important"
            } else {
                ""
            };
            css.push_str(&format!(
                "  {}: {}{};\n",
                property.name, property.value, important
            ));
        }

        css.push_str("}\n\n");
        css
    }

    /// Generate minified CSS
    pub fn generate_minified_css(
        rules: &HashMap<String, CssRule>,
        custom_properties: &HashMap<String, String>,
    ) -> String {
        let mut css = String::new();

        // Add custom properties
        if !custom_properties.is_empty() {
            css.push_str(":root{");
            for (name, value) in custom_properties {
                css.push_str(&format!("--{}:{};", name, value));
            }
            css.push('}');
        }

        // Group rules by media query
        let mut base_rules = Vec::new();
        let mut responsive_rules: HashMap<String, Vec<&CssRule>> = HashMap::new();

        for rule in rules.values() {
            if let Some(ref media_query) = rule.media_query {
                responsive_rules
                    .entry(media_query.clone())
                    .or_default()
                    .push(rule);
            } else {
                base_rules.push(rule);
            }
        }

        // Generate base rules
        for rule in base_rules {
            css.push_str(&Self::rule_to_minified_css(rule));
        }

        // Generate responsive rules
        for (media_query, rules) in responsive_rules {
            css.push_str(&format!("@media {} {{", media_query));
            for rule in rules {
                css.push_str(&Self::rule_to_minified_css(rule));
            }
            css.push('}');
        }

        css
    }

    /// Convert a CSS rule to minified CSS string
    fn rule_to_minified_css(rule: &CssRule) -> String {
        let mut css = String::new();
        css.push_str(&format!("{} {{", rule.selector));

        for property in &rule.properties {
            let important = if property.important { "!important" } else { "" };
            css.push_str(&format!(
                "{}:{}{};",
                property.name, property.value, important
            ));
        }

        css.push('}');
        css
    }
}
