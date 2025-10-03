//! Multi-Language Template Support
//!
//! Parse Tailwind CSS classes embedded in various web frameworks and templating languages.
//! Inspired by the official Tailwind Oxide implementation.

use crate::boundary::{BoundaryClass, BoundaryValidator, LanguageBoundaryRules, TemplateLanguage};
use std::collections::HashMap;

/// Language-specific parser for extracting Tailwind classes from templates
#[derive(Debug, Clone)]
pub struct MultiLanguageParser {
    /// Language type
    language: TemplateLanguage,
    /// Custom extraction patterns for this language
    patterns: LanguagePatterns,
    /// Boundary rules for this language
    boundary_rules: LanguageBoundaryRules,
}

#[derive(Debug, Clone)]
struct LanguagePatterns {
    /// Class attribute patterns (e.g., "class=", ":class=", "[class]=")
    class_patterns: Vec<String>,
    /// Dynamic class patterns (e.g., "{classes}", "v-bind:class")
    dynamic_patterns: Vec<String>,
    /// Conditional patterns (e.g., "ng-class", "x-bind:class")
    conditional_patterns: Vec<String>,
    /// String delimiters used in this language
    string_delimiters: Vec<char>,
}

impl Default for LanguagePatterns {
    fn default() -> Self {
        Self {
            class_patterns: vec!["class=".to_string()],
            dynamic_patterns: Vec::new(),
            conditional_patterns: Vec::new(),
            string_delimiters: vec!['"', '\''],
        }
    }
}

impl MultiLanguageParser {
    /// Create a parser for a specific language
    pub fn new(language: TemplateLanguage) -> Self {
        let patterns = Self::get_language_patterns(language);
        let boundary_rules = LanguageBoundaryRules::for_language(language);

        Self {
            language,
            patterns,
            boundary_rules,
        }
    }

    /// Extract all Tailwind classes from a template string
    pub fn extract_classes(&self, template: &str) -> Vec<String> {
        let mut classes = Vec::new();

        match self.language {
            TemplateLanguage::HTML => self.extract_html_classes(template, &mut classes),
            TemplateLanguage::Vue => self.extract_vue_classes(template, &mut classes),
            TemplateLanguage::Svelte => self.extract_svelte_classes(template, &mut classes),
            TemplateLanguage::Angular => self.extract_angular_classes(template, &mut classes),
            TemplateLanguage::React => self.extract_react_classes(template, &mut classes),
            TemplateLanguage::Haml => self.extract_haml_classes(template, &mut classes),
            TemplateLanguage::Pug => self.extract_pug_classes(template, &mut classes),
            _ => self.extract_html_classes(template, &mut classes), // fallback
        }

        classes
    }

    /// Extract classes from HTML templates
    fn extract_html_classes(&self, template: &str, classes: &mut Vec<String>) {
        for pattern in &self.patterns.class_patterns {
            self.extract_class_attributes(template, pattern, classes);
        }
    }

    /// Extract classes from Vue.js templates
    fn extract_vue_classes(&self, template: &str, classes: &mut Vec<String>) {
        // Standard HTML classes
        self.extract_html_classes(template, classes);

        // Vue-specific patterns
        let vue_patterns = [
            ":class=",           // Shorthand for v-bind:class
            "v-bind:class=",     // Full v-bind syntax
            ":class.",           // Object syntax
            "v-bind:class.",     // Object syntax
        ];

        for pattern in &vue_patterns {
            self.extract_vue_dynamic_classes(template, pattern, classes);
        }
    }

    /// Extract classes from Svelte templates
    fn extract_svelte_classes(&self, template: &str, classes: &mut Vec<String>) {
        // Standard HTML classes
        self.extract_html_classes(template, classes);

        // Svelte-specific patterns
        let svelte_patterns = [
            "class:",            // Class directive
            "{",                 // Variable interpolation in class attribute
        ];

        for pattern in &svelte_patterns {
            self.extract_svelte_dynamic_classes(template, pattern, classes);
        }
    }

    /// Extract classes from Angular templates
    fn extract_angular_classes(&self, template: &str, classes: &mut Vec<String>) {
        // Standard HTML classes
        self.extract_html_classes(template, classes);

        // Angular-specific patterns
        let angular_patterns = [
            "[class]=",          // Property binding
            "[class.",           // Class binding
            "ng-class=",         // ngClass directive
            "[ng-class]=",       // Property bound ngClass
        ];

        for pattern in &angular_patterns {
            self.extract_angular_dynamic_classes(template, pattern, classes);
        }
    }

    /// Extract classes from React/JSX templates
    fn extract_react_classes(&self, template: &str, classes: &mut Vec<String>) {
        // Standard HTML classes
        self.extract_html_classes(template, classes);

        // React-specific patterns
        let react_patterns = [
            "className=",        // React className prop
            "className:",        // Object/className syntax
            "clsx(",             // clsx utility
            "cn(",               // cn utility (common in Tailwind projects)
        ];

        for pattern in &react_patterns {
            self.extract_react_dynamic_classes(template, pattern, classes);
        }
    }

    /// Extract classes from Haml templates
    fn extract_haml_classes(&self, template: &str, classes: &mut Vec<String>) {
        // Haml uses %tag.class#id syntax
        let lines = template.lines();
        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with('%') {
                self.extract_haml_tag_classes(trimmed, classes);
            } else {
                // Also check for Ruby interpolation in class attributes
                self.extract_ruby_interpolation_classes(trimmed, classes);
            }
        }
    }

    /// Extract classes from Pug templates
    fn extract_pug_classes(&self, template: &str, classes: &mut Vec<String>) {
        let lines = template.lines();
        for line in lines {
            let trimmed = line.trim();
            self.extract_pug_tag_classes(trimmed, classes);
        }
    }

    /// Generic class attribute extraction
    fn extract_class_attributes(&self, template: &str, pattern: &str, classes: &mut Vec<String>) {
        let mut start = 0;
        while let Some(pos) = template[start..].find(pattern) {
            let abs_pos = start + pos + pattern.len();

            // Find the opening quote
            if let Some(open_quote_pos) = Self::find_next_quote(&template[abs_pos..]) {
                let abs_open_quote = abs_pos + open_quote_pos;
                let quote_char = template.as_bytes()[abs_open_quote] as char;

                // Find the matching closing quote
                if let Some(close_quote_pos) = Self::find_matching_quote(&template[abs_open_quote + 1..], quote_char) {
                    let abs_close_quote = abs_open_quote + 1 + close_quote_pos;
                    let class_content = &template[abs_open_quote + 1..abs_close_quote];

                    // Extract individual classes
                    self.parse_class_string(class_content, classes);
                }
            }

            start = abs_pos;
        }
    }

    /// Extract Vue dynamic classes
    fn extract_vue_dynamic_classes(&self, template: &str, pattern: &str, classes: &mut Vec<String>) {
        let mut start = 0;
        while let Some(pos) = template[start..].find(pattern) {
            let abs_pos = start + pos + pattern.len();

            // Handle different Vue patterns
            if pattern.ends_with('=') {
                // :class="..." or v-bind:class="..."
                self.extract_vue_binding_classes(&template[abs_pos..], classes);
            } else if pattern.ends_with('.') {
                // :class.object or v-bind:class.object
                self.extract_vue_object_classes(&template[abs_pos..], classes);
            }

            start = abs_pos;
        }
    }

    /// Extract Vue binding classes (:class="...")
    fn extract_vue_binding_classes(&self, content: &str, classes: &mut Vec<String>) {
        if let Some(open_quote_pos) = Self::find_next_quote(content) {
            let quote_char = content.as_bytes()[open_quote_pos] as char;

            if let Some(close_quote_pos) = Self::find_matching_quote(&content[open_quote_pos + 1..], quote_char) {
                let binding_content = &content[open_quote_pos + 1..open_quote_pos + 1 + close_quote_pos];
                self.parse_vue_binding_content(binding_content, classes);
            }
        }
    }

    /// Extract Vue object classes (:class.object)
    fn extract_vue_object_classes(&self, content: &str, classes: &mut Vec<String>) {
        // Look for the object syntax pattern
        if let Some(space_pos) = content.find(' ') {
            let object_part = &content[..space_pos];
            // In object syntax, the class name after the dot might indicate conditional classes
            if let Some(dot_pos) = object_part.find('.') {
                let class_name = &object_part[dot_pos + 1..];
                if !class_name.is_empty() && Self::is_valid_tailwind_class(class_name) {
                    classes.push(class_name.to_string());
                }
            }
        }
    }

    /// Extract Svelte dynamic classes
    fn extract_svelte_dynamic_classes(&self, template: &str, pattern: &str, classes: &mut Vec<String>) {
        let mut start = 0;
        while let Some(pos) = template[start..].find(pattern) {
            let abs_pos = start + pos;

            if pattern == "class:" {
                // class:directive syntax
                self.extract_svelte_class_directive(&template[abs_pos..], classes);
            } else if pattern == "{" {
                // {variable} interpolation
                self.extract_svelte_interpolation(&template[abs_pos..], classes);
            }

            start = abs_pos + pattern.len();
        }
    }

    /// Extract Angular dynamic classes
    fn extract_angular_dynamic_classes(&self, template: &str, pattern: &str, classes: &mut Vec<String>) {
        let mut start = 0;
        while let Some(pos) = template[start..].find(pattern) {
            let abs_pos = start + pos + pattern.len();

            if pattern.starts_with('[') && pattern.ends_with('=') {
                // [class]="..." or [ng-class]="..."
                self.extract_angular_property_binding(&template[abs_pos..], classes);
            } else if pattern.starts_with('[') && pattern.ends_with('.') {
                // [class.condition]="..."
                self.extract_angular_class_binding(&template[abs_pos..], classes);
            }

            start = abs_pos;
        }
    }

    /// Extract React dynamic classes
    fn extract_react_dynamic_classes(&self, template: &str, pattern: &str, classes: &mut Vec<String>) {
        let mut start = 0;
        while let Some(pos) = template[start..].find(pattern) {
            let abs_pos = start + pos;

            if pattern == "className=" || pattern == "className:" {
                self.extract_react_classname(&template[abs_pos..], classes);
            } else if pattern == "clsx(" || pattern == "cn(" {
                self.extract_react_utility_function(&template[abs_pos..], classes);
            }

            start = abs_pos + pattern.len();
        }
    }

    /// Extract Haml tag classes (%div.class#id)
    fn extract_haml_tag_classes(&self, line: &str, classes: &mut Vec<String>) {
        if let Some(tag_end) = line.find(char::is_whitespace) {
            let tag_part = &line[..tag_end];

            // Parse Haml tag syntax: %tag.class1.class2#id
            let mut in_classes = false;
            let mut current_class = String::new();

            for ch in tag_part.chars() {
                match ch {
                    '%' => in_classes = false,
                    '.' => {
                        if !current_class.is_empty() {
                            classes.push(current_class);
                            current_class = String::new();
                        }
                        in_classes = true;
                    }
                    '#' => {
                        if !current_class.is_empty() {
                            classes.push(current_class);
                            current_class = String::new();
                        }
                        in_classes = false;
                        break; // ID part starts, no more classes
                    }
                    _ => {
                        if in_classes {
                            current_class.push(ch);
                        }
                    }
                }
            }

            if !current_class.is_empty() {
                classes.push(current_class);
            }
        }
    }

    /// Extract Pug tag classes (.class1.class2#id)
    fn extract_pug_tag_classes(&self, line: &str, classes: &mut Vec<String>) {
        let trimmed = line.trim_start();

        // Pug syntax: .class1.class2 or tag.class1#id
        let mut chars = trimmed.chars().peekable();
        let mut current_class = String::new();
        let mut in_tag_name = true;

        while let Some(ch) = chars.next() {
            match ch {
                '.' => {
                    if !current_class.is_empty() {
                        classes.push(current_class);
                        current_class = String::new();
                    }
                    in_tag_name = false;
                }
                '#' => {
                    if !current_class.is_empty() {
                        classes.push(current_class);
                        current_class = String::new();
                    }
                    in_tag_name = false;
                    break; // ID part starts
                }
                '(' => {
                    // Attributes start
                    break;
                }
                _ => {
                    if !in_tag_name {
                        current_class.push(ch);
                    } else if ch.is_whitespace() {
                        in_tag_name = false;
                        current_class = String::new();
                    }
                }
            }
        }

        if !current_class.is_empty() {
            classes.push(current_class);
        }
    }

    /// Helper methods
    fn find_next_quote(content: &str) -> Option<usize> {
        content.find('"').or_else(|| content.find('\''))
    }

    fn find_matching_quote(content: &str, quote_char: char) -> Option<usize> {
        let mut escaped = false;
        for (i, ch) in content.chars().enumerate() {
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == quote_char {
                return Some(i);
            }
        }
        None
    }

    fn parse_class_string(&self, class_string: &str, classes: &mut Vec<String>) {
        // Split by whitespace and filter out empty strings
        for class in class_string.split_whitespace() {
            let trimmed = class.trim();
            if !trimmed.is_empty() {
                // Handle dynamic interpolation within class strings
                self.parse_class_with_interpolation(trimmed, classes);
            }
        }
    }

    fn parse_class_with_interpolation(&self, class: &str, classes: &mut Vec<String>) {
        // Handle ${variable} interpolation in template literals
        if class.contains("${") {
            let parts: Vec<&str> = class.split("${").collect();
            for part in parts {
                if let Some(end) = part.find('}') {
                    let before_var = &part[..end];
                    if !before_var.is_empty() {
                        self.parse_class_string(before_var, classes);
                    }
                    // Skip the variable part
                    if let Some(after_var) = part.get(end + 1..) {
                        if !after_var.is_empty() {
                            self.parse_class_string(after_var, classes);
                        }
                    }
                } else {
                    self.parse_class_string(part, classes);
                }
            }
        } else {
            // No interpolation, check if it's a valid class
            if Self::is_valid_tailwind_class(class) {
                classes.push(class.to_string());
            }
        }
    }

    fn parse_vue_binding_content(&self, content: &str, classes: &mut Vec<String>) {
        let trimmed = content.trim();

        // Handle Vue object syntax: { 'class1': condition, 'class2': true }
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            self.parse_vue_object_binding(&trimmed[1..trimmed.len() - 1], classes);
        }
        // Handle Vue array syntax: ['class1', 'class2', condition ? 'class3' : '']
        else if trimmed.starts_with('[') && trimmed.ends_with(']') {
            self.parse_vue_array_binding(&trimmed[1..trimmed.len() - 1], classes);
        }
        // Handle Vue ternary: condition ? 'class1 class2' : 'class3'
        else if trimmed.contains(" ? ") && trimmed.contains(" : ") {
            self.parse_vue_ternary_binding(trimmed, classes);
        }
        // Handle simple string
        else {
            self.parse_class_string(trimmed.trim_matches('"').trim_matches('\''), classes);
        }
    }

    fn parse_vue_object_binding(&self, content: &str, classes: &mut Vec<String>) {
        // Simple parsing for 'class': condition pairs
        for pair in content.split(',') {
            if let Some(colon_pos) = pair.find(':') {
                let class_part = pair[..colon_pos].trim().trim_matches('"').trim_matches('\'');
                if Self::is_valid_tailwind_class(class_part) {
                    classes.push(class_part.to_string());
                }
            }
        }
    }

    fn parse_vue_array_binding(&self, content: &str, classes: &mut Vec<String>) {
        for item in content.split(',') {
            let trimmed = item.trim().trim_matches('"').trim_matches('\'');
            // Skip conditional expressions, keep string literals
            if !trimmed.contains(" ? ") && !trimmed.contains(" : ") && !trimmed.is_empty() {
                self.parse_class_string(trimmed, classes);
            }
        }
    }

    fn parse_vue_ternary_binding(&self, content: &str, classes: &mut Vec<String>) {
        // Extract classes from both sides of ternary
        if let Some(question_pos) = content.find(" ? ") {
            if let Some(colon_pos) = content.find(" : ") {
                let true_part = content[question_pos + 3..colon_pos].trim();
                let false_part = content[colon_pos + 3..].trim();

                self.parse_class_string(true_part.trim_matches('"').trim_matches('\''), classes);
                self.parse_class_string(false_part.trim_matches('"').trim_matches('\''), classes);
            }
        }
    }

    fn extract_svelte_class_directive(&self, content: &str, classes: &mut Vec<String>) {
        // class:directive={expression} syntax
        if let Some(eq_pos) = content.find('=') {
            let directive_content = &content[eq_pos + 1..];
            self.parse_svelte_directive_content(directive_content, classes);
        }
    }

    fn extract_svelte_interpolation(&self, content: &str, classes: &mut Vec<String>) {
        // {variable} syntax within class attributes
        if let Some(close_pos) = content.find('}') {
            let interpolation_content = &content[1..close_pos];
            // For now, we'll assume interpolated variables contain class names
            // In a real implementation, you'd need to resolve the variables
            if Self::is_valid_tailwind_class(interpolation_content.trim()) {
                classes.push(interpolation_content.trim().to_string());
            }
        }
    }

    fn parse_svelte_directive_content(&self, content: &str, classes: &mut Vec<String>) {
        let trimmed = content.trim().trim_start_matches('{').trim_end_matches('}');
        self.parse_vue_binding_content(trimmed, classes); // Similar parsing logic
    }

    fn extract_angular_property_binding(&self, content: &str, classes: &mut Vec<String>) {
        if let Some(open_quote_pos) = Self::find_next_quote(content) {
            let quote_char = content.as_bytes()[open_quote_pos] as char;

            if let Some(close_quote_pos) = Self::find_matching_quote(&content[open_quote_pos + 1..], quote_char) {
                let binding_content = &content[open_quote_pos + 1..open_quote_pos + 1 + close_quote_pos];
                self.parse_vue_binding_content(binding_content, classes); // Reuse Vue parsing logic
            }
        }
    }

    fn extract_angular_class_binding(&self, content: &str, classes: &mut Vec<String>) {
        // [class.condition]="expression" - the condition part might be a class name
        if let Some(eq_pos) = content.find('=') {
            let condition_part = &content[..eq_pos];
            if let Some(dot_pos) = condition_part.find('.') {
                let class_name = &condition_part[dot_pos + 1..];
                if Self::is_valid_tailwind_class(class_name.trim()) {
                    classes.push(class_name.trim().to_string());
                }
            }
        }
    }

    fn extract_react_classname(&self, content: &str, classes: &mut Vec<String>) {
        if let Some(open_quote_pos) = Self::find_next_quote(content) {
            let quote_char = content.as_bytes()[open_quote_pos] as char;

            if let Some(close_quote_pos) = Self::find_matching_quote(&content[open_quote_pos + 1..], quote_char) {
                let class_content = &content[open_quote_pos + 1..open_quote_pos + 1 + close_quote_pos];
                self.parse_class_string(class_content, classes);
            }
        }
    }

    fn extract_react_utility_function(&self, content: &str, classes: &mut Vec<String>) {
        // clsx(...) or cn(...) - extract string literals from arguments
        if let Some(open_paren) = content.find('(') {
            if let Some(close_paren) = Self::find_matching_paren(&content[open_paren + 1..]) {
                let args_content = &content[open_paren + 1..open_paren + 1 + close_paren];
                self.parse_react_utility_args(args_content, classes);
            }
        }
    }

    fn parse_react_utility_args(&self, args: &str, classes: &mut Vec<String>) {
        // Simple parsing of clsx/cn arguments - extract string literals
        let mut in_string = false;
        let mut current_string = String::new();
        let mut string_char = '"';

        for ch in args.chars() {
            match ch {
                '"' | '\'' => {
                    if !in_string {
                        in_string = true;
                        string_char = ch;
                        current_string = String::new();
                    } else if ch == string_char {
                        in_string = false;
                        if !current_string.is_empty() {
                            self.parse_class_string(&current_string, classes);
                        }
                    }
                }
                _ => {
                    if in_string {
                        current_string.push(ch);
                    }
                }
            }
        }
    }

    fn extract_ruby_interpolation_classes(&self, line: &str, classes: &mut Vec<String>) {
        // Handle Ruby interpolation #{variable} in Haml
        let mut start = 0;
        while let Some(pos) = line[start..].find("#{") {
            let abs_pos = start + pos;
            if let Some(end_pos) = line[abs_pos + 2..].find('}') {
                let interpolation_content = &line[abs_pos + 2..abs_pos + 2 + end_pos];
                // Assume interpolated variables contain class names
                if Self::is_valid_tailwind_class(interpolation_content.trim()) {
                    classes.push(interpolation_content.trim().to_string());
                }
                start = abs_pos + 2 + end_pos;
            } else {
                break;
            }
        }
    }

    fn find_matching_paren(content: &str) -> Option<usize> {
        let mut depth = 0;
        for (i, ch) in content.chars().enumerate() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Basic validation for Tailwind class names
    fn is_valid_tailwind_class(class: &str) -> bool {
        if class.is_empty() {
            return false;
        }

        // Check for valid Tailwind patterns
        let valid_patterns = [
            "flex", "grid", "block", "inline", "hidden", // display
            "p-", "m-", "px-", "py-", "pt-", "pb-", "pl-", "pr-", // spacing
            "bg-", "text-", // colors
            "w-", "h-", "max-w-", "max-h-", // sizing
            "rounded", "border", // borders
            "shadow", "ring", // effects
        ];

        for pattern in &valid_patterns {
            if class.starts_with(pattern) {
                return true;
            }
        }

        // Check for common utility classes
        let common_classes = [
            "container", "clearfix", "invisible", "visible",
            "static", "relative", "absolute", "fixed", "sticky",
            "justify-center", "items-center", "text-center",
            "font-bold", "font-medium", "font-light",
        ];

        common_classes.contains(&class)
    }

    /// Get language-specific patterns
    fn get_language_patterns(language: TemplateLanguage) -> LanguagePatterns {
        match language {
            TemplateLanguage::Vue => LanguagePatterns {
                class_patterns: vec![
                    "class=".to_string(),
                    ":class=".to_string(),
                    "v-bind:class=".to_string(),
                ],
                dynamic_patterns: vec![
                    ":class.".to_string(),
                    "v-bind:class.".to_string(),
                ],
                conditional_patterns: vec![
                    ":class.".to_string(),
                ],
                string_delimiters: vec!['"', '\'', '`'],
            },
            TemplateLanguage::Svelte => LanguagePatterns {
                class_patterns: vec![
                    "class=".to_string(),
                    "class:".to_string(),
                ],
                dynamic_patterns: vec![
                    "{".to_string(),
                ],
                conditional_patterns: vec![
                    "class:".to_string(),
                ],
                string_delimiters: vec!['"', '\'', '`'],
            },
            TemplateLanguage::Angular => LanguagePatterns {
                class_patterns: vec![
                    "class=".to_string(),
                ],
                dynamic_patterns: vec![
                    "[class]=".to_string(),
                    "[class.".to_string(),
                    "[ng-class]=".to_string(),
                ],
                conditional_patterns: vec![
                    "[class.".to_string(),
                    "ng-class=".to_string(),
                ],
                string_delimiters: vec!['"', '\'', '`'],
            },
            TemplateLanguage::React => LanguagePatterns {
                class_patterns: vec![
                    "className=".to_string(),
                ],
                dynamic_patterns: vec![
                    "className=".to_string(),
                    "clsx(".to_string(),
                    "cn(".to_string(),
                ],
                conditional_patterns: vec![
                    "clsx(".to_string(),
                    "cn(".to_string(),
                ],
                string_delimiters: vec!['"', '\'', '`'],
            },
            TemplateLanguage::Haml => LanguagePatterns {
                class_patterns: vec![
                    ".".to_string(),
                ],
                dynamic_patterns: vec![
                    "#{}".to_string(),
                ],
                conditional_patterns: vec![],
                string_delimiters: vec!['"', '\'', '`'],
            },
            TemplateLanguage::Pug => LanguagePatterns {
                class_patterns: vec![
                    ".".to_string(),
                ],
                dynamic_patterns: vec![
                    ".".to_string(),
                    "#".to_string(),
                ],
                conditional_patterns: vec![],
                string_delimiters: vec!['"', '\'', '`'],
            },
            _ => LanguagePatterns::default(),
        }
    }
}

/// Multi-language parser registry
#[derive(Debug)]
pub struct MultiLanguageRegistry {
    parsers: HashMap<TemplateLanguage, MultiLanguageParser>,
}

impl Default for MultiLanguageRegistry {
    fn default() -> Self {
        let mut parsers = HashMap::new();

        // Initialize parsers for all supported languages
        let languages = [
            TemplateLanguage::HTML,
            TemplateLanguage::Vue,
            TemplateLanguage::Svelte,
            TemplateLanguage::Angular,
            TemplateLanguage::React,
            TemplateLanguage::Haml,
            TemplateLanguage::Pug,
        ];

        for language in languages {
            parsers.insert(language, MultiLanguageParser::new(language));
        }

        Self { parsers }
    }
}

impl MultiLanguageRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Extract classes from a template in the specified language
    pub fn extract_classes(&self, template: &str, language: TemplateLanguage) -> Vec<String> {
        if let Some(parser) = self.parsers.get(&language) {
            parser.extract_classes(template)
        } else {
            // Fallback to HTML parser
            if let Some(html_parser) = self.parsers.get(&TemplateLanguage::HTML) {
                html_parser.extract_classes(template)
            } else {
                Vec::new()
            }
        }
    }

    /// Auto-detect language and extract classes
    pub fn extract_classes_auto(&self, template: &str) -> Vec<String> {
        let detected_language = self.detect_language(template);
        self.extract_classes(template, detected_language)
    }

    /// Detect the template language
    pub fn detect_language(&self, template: &str) -> TemplateLanguage {
        // Simple language detection based on unique patterns
        if template.contains("<template>") && (template.contains(":class") || template.contains("v-bind:")) {
            TemplateLanguage::Vue
        } else if template.contains("{#") || template.contains("{/") {
            TemplateLanguage::Svelte
        } else if template.contains("[class") || template.contains("(click)") {
            TemplateLanguage::Angular
        } else if template.contains("className=") || template.contains("clsx(") {
            TemplateLanguage::React
        } else if template.contains("%") && template.contains(".") && !template.contains("<") {
            TemplateLanguage::Haml
        } else if !template.contains("<") && (template.contains(".") || template.contains("#")) {
            TemplateLanguage::Pug
        } else {
            TemplateLanguage::HTML
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_class_extraction() {
        let registry = MultiLanguageRegistry::new();
        let html = r#"<div class="flex items-center px-4 bg-blue-500"></div>"#;

        let classes = registry.extract_classes(html, TemplateLanguage::HTML);
        assert_eq!(classes.len(), 4);
        assert!(classes.contains(&"flex".to_string()));
        assert!(classes.contains(&"items-center".to_string()));
        assert!(classes.contains(&"px-4".to_string()));
        assert!(classes.contains(&"bg-blue-500".to_string()));
    }

    #[test]
    fn test_vue_class_extraction() {
        let registry = MultiLanguageRegistry::new();
        let vue = r#"<div class="flex" :class="{'active': isActive, 'hidden': !visible}"></div>"#;

        let classes = registry.extract_classes(vue, TemplateLanguage::Vue);
        assert!(classes.contains(&"flex".to_string()));
        assert!(classes.contains(&"active".to_string()));
        assert!(classes.contains(&"hidden".to_string()));
    }

    #[test]
    fn test_react_class_extraction() {
        let registry = MultiLanguageRegistry::new();
        let react = r#"<div className="flex items-center" className={clsx('px-4', condition && 'bg-blue-500')}></div>"#;

        let classes = registry.extract_classes(react, TemplateLanguage::React);
        assert!(classes.contains(&"flex".to_string()));
        assert!(classes.contains(&"items-center".to_string()));
        assert!(classes.contains(&"px-4".to_string()));
        assert!(classes.contains(&"bg-blue-500".to_string()));
    }

    #[test]
    fn test_haml_class_extraction() {
        let registry = MultiLanguageRegistry::new();
        let haml = "%div.flex.items-center.px-4";

        let classes = registry.extract_classes(haml, TemplateLanguage::Haml);
        assert!(classes.contains(&"flex".to_string()));
        assert!(classes.contains(&"items-center".to_string()));
        assert!(classes.contains(&"px-4".to_string()));
    }

    #[test]
    fn test_language_detection() {
        let registry = MultiLanguageRegistry::new();

        assert_eq!(registry.detect_language(r#"<div class="flex"></div>"#), TemplateLanguage::HTML);
        assert_eq!(registry.detect_language(r#"<div :class="{}"></div>"#), TemplateLanguage::Vue);
        assert_eq!(registry.detect_language(r#"<div class:directive=""></div>"#), TemplateLanguage::Svelte);
        assert_eq!(registry.detect_language(r#"<div [class]=""></div>"#), TemplateLanguage::Angular);
        assert_eq!(registry.detect_language(r#"<div className=""></div>"#), TemplateLanguage::React);
        assert_eq!(registry.detect_language("%div.flex"), TemplateLanguage::Haml);
        assert_eq!(registry.detect_language(".flex"), TemplateLanguage::Pug);
    }

    #[test]
    fn test_auto_extraction() {
        let registry = MultiLanguageRegistry::new();
        let vue_template = r#"<div class="flex" :class="{'active': true}"></div>"#;

        let classes = registry.extract_classes_auto(vue_template);
        assert!(classes.contains(&"flex".to_string()));
        assert!(classes.contains(&"active".to_string()));
    }
}
