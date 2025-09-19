//! Validation system for tailwind-rs
//!
//! This module provides class validation functionality to ensure that
//! generated Tailwind CSS classes are valid and don't conflict.

use std::collections::{HashMap, HashSet};
use thiserror::Error;
use crate::custom_variant::CustomVariantManager;

/// Represents validation errors
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid class name: {0}")]
    InvalidClass(String),

    #[error("Class conflict: {0} conflicts with {1}")]
    ClassConflict(String, String),

    #[error("Deprecated class: {0}")]
    DeprecatedClass(String),

    #[error("Unsupported class: {0}")]
    UnsupportedClass(String),

    #[error("Invalid custom variant: {0}")]
    InvalidCustomVariant(String),

    #[error("Custom variant validation failed: {0}")]
    CustomVariantValidation(String),
}

/// Validation rules for class validation
#[derive(Debug, Clone)]
pub struct ValidationRules {
    /// Allowed class patterns
    pub allowed_patterns: Vec<regex::Regex>,
    /// Forbidden class patterns
    pub forbidden_patterns: Vec<regex::Regex>,
    /// Deprecated classes
    pub deprecated_classes: HashSet<String>,
    /// Class conflicts (classes that can't be used together)
    pub class_conflicts: HashMap<String, HashSet<String>>,
    /// Required classes (classes that must be present when certain classes are used)
    pub required_classes: HashMap<String, HashSet<String>>,
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationRules {
    /// Create new validation rules with default Tailwind CSS patterns
    pub fn new() -> Self {
        let mut rules = Self {
            allowed_patterns: Vec::new(),
            forbidden_patterns: Vec::new(),
            deprecated_classes: HashSet::new(),
            class_conflicts: HashMap::new(),
            required_classes: HashMap::new(),
        };

        // Add common Tailwind CSS class patterns
        rules.add_allowed_patterns();
        rules.add_class_conflicts();
        rules.add_required_classes();

        rules
    }

    /// Add allowed class patterns
    fn add_allowed_patterns(&mut self) {
        let patterns = vec![
            // Layout
            r"^(block|inline-block|inline|flex|inline-flex|table|inline-table|table-caption|table-cell|table-column|table-column-group|table-footer-group|table-header-group|table-row-group|table-row|flow-root|grid|inline-grid|contents|list-item|hidden)$",
            // Flexbox
            r"^(flex-row|flex-row-reverse|flex-col|flex-col-reverse|flex-wrap|flex-wrap-reverse|flex-nowrap|items-start|items-end|items-center|items-baseline|items-stretch|justify-start|justify-end|justify-center|justify-between|justify-around|justify-evenly|justify-stretch|content-start|content-end|content-center|content-between|content-around|content-evenly|content-stretch|self-auto|self-start|self-end|self-center|self-stretch|self-baseline)$",
            // Grid
            r"^(grid-cols-\d+|grid-cols-none|grid-cols-subgrid|col-auto|col-span-\d+|col-start-\d+|col-end-\d+|col-start-auto|col-end-auto|grid-rows-\d+|grid-rows-none|grid-rows-subgrid|row-auto|row-span-\d+|row-start-\d+|row-end-\d+|row-start-auto|row-end-auto|auto-cols-auto|auto-cols-min|auto-cols-max|auto-cols-fr|auto-rows-auto|auto-rows-min|auto-rows-max|auto-rows-fr|gap-\d+|gap-x-\d+|gap-y-\d+|justify-items-start|justify-items-end|justify-items-center|justify-items-stretch|justify-self-auto|justify-self-start|justify-self-end|justify-self-center|justify-self-stretch|place-content-start|place-content-end|place-content-center|place-content-between|place-content-around|place-content-evenly|place-content-stretch|place-items-start|place-items-end|place-items-center|place-items-stretch|place-self-auto|place-self-start|place-self-end|place-self-center|place-self-stretch)$",
            // Spacing (Enhanced with fractional values and space/divide utilities)
            r"^(p-\d+(\.\d+)?|pt-\d+(\.\d+)?|pr-\d+(\.\d+)?|pb-\d+(\.\d+)?|pl-\d+(\.\d+)?|px-\d+(\.\d+)?|py-\d+(\.\d+)?|ps-\d+(\.\d+)?|pe-\d+(\.\d+)?|m-\d+(\.\d+)?|mt-\d+(\.\d+)?|mr-\d+(\.\d+)?|mb-\d+(\.\d+)?|ml-\d+(\.\d+)?|mx-\d+(\.\d+)?|my-\d+(\.\d+)?|ms-\d+(\.\d+)?|me-\d+(\.\d+)?|-m-\d+(\.\d+)?|-mt-\d+(\.\d+)?|-mr-\d+(\.\d+)?|-mb-\d+(\.\d+)?|-ml-\d+(\.\d+)?|-mx-\d+(\.\d+)?|-my-\d+(\.\d+)?|space-x-\d+(\.\d+)?|space-y-\d+(\.\d+)?|space-x-reverse|space-y-reverse|divide-x-\d+(\.\d+)?|divide-y-\d+(\.\d+)?|divide-x-reverse|divide-y-reverse)$",
            // Sizing
            r"^(w-\d+|w-auto|w-px|w-0\.5|w-1\.5|w-2\.5|w-3\.5|w-1\/2|w-1\/3|w-2\/3|w-1\/4|w-2\/4|w-3\/4|w-1\/5|w-2\/5|w-3\/5|w-4\/5|w-1\/6|w-2\/6|w-3\/6|w-4\/6|w-5\/6|w-1\/12|w-2\/12|w-3\/12|w-4\/12|w-5\/12|w-6\/12|w-7\/12|w-8\/12|w-9\/12|w-10\/12|w-11\/12|w-full|w-screen|w-min|w-max|w-fit|h-\d+|h-auto|h-px|h-0\.5|h-1\.5|h-2\.5|h-3\.5|h-1\/2|h-1\/3|h-2\/3|h-1\/4|h-2\/4|h-3\/4|h-1\/5|h-2\/5|h-3\/5|h-4\/5|h-1\/6|h-2\/6|h-3\/6|h-4\/6|h-5\/6|h-1\/12|h-2\/12|h-3\/12|h-4\/12|h-5\/12|h-6\/12|h-7\/12|h-8\/12|h-9\/12|h-10\/12|h-11\/12|h-full|h-screen|h-min|h-max|h-fit|min-w-0|min-w-full|min-w-min|min-w-max|min-w-fit|max-w-0|max-w-none|max-w-xs|max-w-sm|max-w-md|max-w-lg|max-w-xl|max-w-2xl|max-w-3xl|max-w-4xl|max-w-5xl|max-w-6xl|max-w-7xl|max-w-full|max-w-min|max-w-max|max-w-fit|max-w-prose|max-w-screen-sm|max-w-screen-md|max-w-screen-lg|max-w-screen-xl|max-w-screen-2xl|min-h-0|min-h-full|min-h-screen|min-h-min|min-h-max|min-h-fit|max-h-0|max-h-px|max-h-0\.5|max-h-1|max-h-1\.5|max-h-2|max-h-2\.5|max-h-3|max-h-3\.5|max-h-4|max-h-5|max-h-6|max-h-7|max-h-8|max-h-9|max-h-10|max-h-11|max-h-12|max-h-14|max-h-16|max-h-20|max-h-24|max-h-28|max-h-32|max-h-36|max-h-40|max-h-44|max-h-48|max-h-52|max-h-56|max-h-60|max-h-64|max-h-72|max-h-80|max-h-96|max-h-px|max-h-0\.5|max-h-1|max-h-1\.5|max-h-2|max-h-2\.5|max-h-3|max-h-3\.5|max-h-4|max-h-5|max-h-6|max-h-7|max-h-8|max-h-9|max-h-10|max-h-11|max-h-12|max-h-14|max-h-16|max-h-20|max-h-24|max-h-28|max-h-32|max-h-36|max-h-40|max-h-44|max-h-48|max-h-52|max-h-56|max-h-60|max-h-64|max-h-72|max-h-80|max-h-96|max-h-full|max-h-screen)$",
            // Typography
            r"^(text-xs|text-sm|text-base|text-lg|text-xl|text-2xl|text-3xl|text-4xl|text-5xl|text-6xl|text-7xl|text-8xl|text-9xl|font-thin|font-extralight|font-light|font-normal|font-medium|font-semibold|font-bold|font-extrabold|font-black|italic|not-italic|leading-3|leading-4|leading-5|leading-6|leading-7|leading-8|leading-9|leading-10|leading-none|leading-tight|leading-snug|leading-normal|leading-relaxed|leading-loose|tracking-tighter|tracking-tight|tracking-normal|tracking-wide|tracking-wider|tracking-widest|text-left|text-center|text-right|text-justify|text-start|text-end|text-inherit|text-current|text-transparent|text-black|text-white|text-slate-\d+|text-gray-\d+|text-zinc-\d+|text-neutral-\d+|text-stone-\d+|text-red-\d+|text-orange-\d+|text-amber-\d+|text-yellow-\d+|text-lime-\d+|text-green-\d+|text-emerald-\d+|text-teal-\d+|text-cyan-\d+|text-sky-\d+|text-blue-\d+|text-indigo-\d+|text-violet-\d+|text-purple-\d+|text-fuchsia-\d+|text-pink-\d+|text-rose-\d+|underline|overline|line-through|no-underline|decoration-solid|decoration-double|decoration-dotted|decoration-dashed|decoration-wavy|decoration-auto|decoration-from-font|decoration-0|decoration-1|decoration-2|decoration-4|decoration-8|underline-offset-auto|underline-offset-0|underline-offset-1|underline-offset-2|underline-offset-4|underline-offset-8|uppercase|lowercase|capitalize|normal-case)$",
            // Backgrounds
            r"^(bg-inherit|bg-current|bg-transparent|bg-black|bg-white|bg-slate-\d+|bg-gray-\d+|bg-zinc-\d+|bg-neutral-\d+|bg-stone-\d+|bg-red-\d+|bg-orange-\d+|bg-amber-\d+|bg-yellow-\d+|bg-lime-\d+|bg-green-\d+|bg-emerald-\d+|bg-teal-\d+|bg-cyan-\d+|bg-sky-\d+|bg-blue-\d+|bg-indigo-\d+|bg-violet-\d+|bg-purple-\d+|bg-fuchsia-\d+|bg-pink-\d+|bg-rose-\d+)$",
            // Borders
            r"^(border-0|border-2|border-4|border-8|border|border-t-0|border-t-2|border-t-4|border-t-8|border-t|border-r-0|border-r-2|border-r-4|border-r-8|border-r|border-b-0|border-b-2|border-b-4|border-b-8|border-b|border-l-0|border-l-2|border-l-4|border-l-8|border-l|border-x-0|border-x-2|border-x-4|border-x-8|border-x|border-y-0|border-y-2|border-y-4|border-y-8|border-y|border-solid|border-dashed|border-dotted|border-double|border-none|border-inherit|border-current|border-transparent|border-black|border-white|border-slate-\d+|border-gray-\d+|border-zinc-\d+|border-neutral-\d+|border-stone-\d+|border-red-\d+|border-orange-\d+|border-amber-\d+|border-yellow-\d+|border-lime-\d+|border-green-\d+|border-emerald-\d+|border-teal-\d+|border-cyan-\d+|border-sky-\d+|border-blue-\d+|border-indigo-\d+|border-violet-\d+|border-purple-\d+|border-fuchsia-\d+|border-pink-\d+|border-rose-\d+|rounded-none|rounded-sm|rounded|rounded-md|rounded-lg|rounded-xl|rounded-2xl|rounded-3xl|rounded-full|rounded-t-none|rounded-t-sm|rounded-t|rounded-t-md|rounded-t-lg|rounded-t-xl|rounded-t-2xl|rounded-t-3xl|rounded-t-full|rounded-r-none|rounded-r-sm|rounded-r|rounded-r-md|rounded-r-lg|rounded-r-xl|rounded-r-2xl|rounded-r-3xl|rounded-r-full|rounded-b-none|rounded-b-sm|rounded-b|rounded-b-md|rounded-b-lg|rounded-b-xl|rounded-b-2xl|rounded-b-3xl|rounded-b-full|rounded-l-none|rounded-l-sm|rounded-l|rounded-l-md|rounded-l-lg|rounded-l-xl|rounded-l-2xl|rounded-l-3xl|rounded-l-full|rounded-tl-none|rounded-tl-sm|rounded-tl|rounded-tl-md|rounded-tl-lg|rounded-tl-xl|rounded-tl-2xl|rounded-tl-3xl|rounded-tl-full|rounded-tr-none|rounded-tr-sm|rounded-tr|rounded-tr-md|rounded-tr-lg|rounded-tr-xl|rounded-tr-2xl|rounded-tr-3xl|rounded-tr-full|rounded-br-none|rounded-br-sm|rounded-br|rounded-br-md|rounded-br-lg|rounded-br-xl|rounded-br-2xl|rounded-br-3xl|rounded-br-full|rounded-bl-none|rounded-bl-sm|rounded-bl|rounded-bl-md|rounded-bl-lg|rounded-bl-xl|rounded-bl-2xl|rounded-bl-3xl|rounded-bl-full)$",
            // Effects
            r"^(shadow-sm|shadow|shadow-md|shadow-lg|shadow-xl|shadow-2xl|shadow-inner|shadow-none|shadow-inherit|shadow-current|shadow-transparent|shadow-black|shadow-white|shadow-slate-\d+|shadow-gray-\d+|shadow-zinc-\d+|shadow-neutral-\d+|shadow-stone-\d+|shadow-red-\d+|shadow-orange-\d+|shadow-amber-\d+|shadow-yellow-\d+|shadow-lime-\d+|shadow-green-\d+|shadow-emerald-\d+|shadow-teal-\d+|shadow-cyan-\d+|shadow-sky-\d+|shadow-blue-\d+|shadow-indigo-\d+|shadow-violet-\d+|shadow-purple-\d+|shadow-fuchsia-\d+|shadow-pink-\d+|shadow-rose-\d+|opacity-0|opacity-5|opacity-10|opacity-20|opacity-25|opacity-30|opacity-40|opacity-50|opacity-60|opacity-70|opacity-75|opacity-80|opacity-90|opacity-95|opacity-100)$",
            // Transforms
            r"^(transform|transform-gpu|transform-none|origin-center|origin-top|origin-top-right|origin-right|origin-bottom-right|origin-bottom|origin-bottom-left|origin-left|origin-top-left|scale-0|scale-50|scale-75|scale-90|scale-95|scale-100|scale-105|scale-110|scale-125|scale-150|scale-x-0|scale-x-50|scale-x-75|scale-x-90|scale-x-95|scale-x-100|scale-x-105|scale-x-110|scale-x-125|scale-x-150|scale-y-0|scale-y-50|scale-y-75|scale-y-90|scale-y-95|scale-y-100|scale-y-105|scale-y-110|scale-y-125|scale-y-150|rotate-0|rotate-1|rotate-2|rotate-3|rotate-6|rotate-12|rotate-45|rotate-90|rotate-180|-rotate-180|-rotate-90|-rotate-45|-rotate-12|-rotate-6|-rotate-3|-rotate-2|-rotate-1|translate-x-0|translate-x-px|translate-x-0\.5|translate-x-1|translate-x-1\.5|translate-x-2|translate-x-2\.5|translate-x-3|translate-x-3\.5|translate-x-4|translate-x-5|translate-x-6|translate-x-7|translate-x-8|translate-x-9|translate-x-10|translate-x-11|translate-x-12|translate-x-14|translate-x-16|translate-x-20|translate-x-24|translate-x-28|translate-x-32|translate-x-36|translate-x-40|translate-x-44|translate-x-48|translate-x-52|translate-x-56|translate-x-60|translate-x-64|translate-x-72|translate-x-80|translate-x-96|translate-x-1\/2|translate-x-1\/3|translate-x-2\/3|translate-x-1\/4|translate-x-2\/4|translate-x-3\/4|translate-x-full|-translate-x-0|-translate-x-px|-translate-x-0\.5|-translate-x-1|-translate-x-1\.5|-translate-x-2|-translate-x-2\.5|-translate-x-3|-translate-x-3\.5|-translate-x-4|-translate-x-5|-translate-x-6|-translate-x-7|-translate-x-8|-translate-x-9|-translate-x-10|-translate-x-11|-translate-x-12|-translate-x-14|-translate-x-16|-translate-x-20|-translate-x-24|-translate-x-28|-translate-x-32|-translate-x-36|-translate-x-40|-translate-x-44|-translate-x-48|-translate-x-52|-translate-x-56|-translate-x-60|-translate-x-64|-translate-x-72|-translate-x-80|-translate-x-96|-translate-x-1\/2|-translate-x-1\/3|-translate-x-2\/3|-translate-x-1\/4|-translate-x-2\/4|-translate-x-3\/4|-translate-x-full|translate-y-0|translate-y-px|translate-y-0\.5|translate-y-1|translate-y-1\.5|translate-y-2|translate-y-2\.5|translate-y-3|translate-y-3\.5|translate-y-4|translate-y-5|translate-y-6|translate-y-7|translate-y-8|translate-y-9|translate-y-10|translate-y-11|translate-y-12|translate-y-14|translate-y-16|translate-y-20|translate-y-24|translate-y-28|translate-y-32|translate-y-36|translate-y-40|translate-y-44|translate-y-48|translate-y-52|translate-y-56|translate-y-60|translate-y-64|translate-y-72|translate-y-80|translate-y-96|translate-y-1\/2|translate-y-1\/3|translate-y-2\/3|translate-y-1\/4|translate-y-2\/4|translate-y-3\/4|translate-y-full|-translate-y-0|-translate-y-px|-translate-y-0\.5|-translate-y-1|-translate-y-1\.5|-translate-y-2|-translate-y-2\.5|-translate-y-3|-translate-y-3\.5|-translate-y-4|-translate-y-5|-translate-y-6|-translate-y-7|-translate-y-8|-translate-y-9|-translate-y-10|-translate-y-11|-translate-y-12|-translate-y-14|-translate-y-16|-translate-y-20|-translate-y-24|-translate-y-28|-translate-y-32|-translate-y-36|-translate-y-40|-translate-y-44|-translate-y-48|-translate-y-52|-translate-y-56|-translate-y-60|-translate-y-64|-translate-y-72|-translate-y-80|-translate-y-96|-translate-y-1\/2|-translate-y-1\/3|-translate-y-2\/3|-translate-y-1\/4|-translate-y-2\/4|-translate-y-3\/4|-translate-y-full|skew-x-0|skew-x-1|skew-x-2|skew-x-3|skew-x-6|skew-x-12|-skew-x-12|-skew-x-6|-skew-x-3|-skew-x-2|-skew-x-1|skew-y-0|skew-y-1|skew-y-2|skew-y-3|skew-y-6|skew-y-12|-skew-y-12|-skew-y-6|-skew-y-3|-skew-y-2|-skew-y-1)$",
            // Interactivity
            r"^(appearance-none|appearance-auto|cursor-auto|cursor-default|cursor-pointer|cursor-wait|cursor-text|cursor-move|cursor-help|cursor-not-allowed|pointer-events-none|pointer-events-auto|resize-none|resize-y|resize-x|resize|select-none|select-text|select-all|select-auto)$",
            // SVG
            r"^(fill-current|stroke-current|stroke-0|stroke-1|stroke-2)$",
            // Accessibility
            r"^(sr-only|not-sr-only|focus-within|focus-visible|focus|focus:outline-none|focus:outline|focus:outline-2|focus:outline-4|focus:outline-8|focus:outline-offset-0|focus:outline-offset-1|focus:outline-offset-2|focus:outline-offset-4|focus:outline-offset-8)$",
            // Responsive prefixes
            r"^(sm:|md:|lg:|xl:|2xl:).*$",
            // State prefixes (removed catch-all to allow more specific patterns)
            r"^(hover:|focus:|active:|visited:|disabled:|checked:|indeterminate:|required:|valid:|invalid:|in-range:|out-of-range:|read-only:|read-write:|optional:|placeholder-shown:|autofill:|default:|first-child:|last-child:|only-child:|first-of-type:|last-of-type:|only-of-type:|empty:|target:|root:|not:|where:|is:|has:|before:|after:|first-letter:|first-line:|selection:|marker:|placeholder:|file:|backdrop:|any-link:|link:|local-link:|scope:|current:|past:|future:|playing:|paused:|seeking:|buffering:|stalled:|muted:|volume-locked:|user-invalid:|user-valid:|modal:|picture-in-picture:|fullscreen:|resize:|scroll:|snap:|touch:|user-select:|will-change:|accent-color:|appearance:|cursor:|outline:)(bg-|text-|border-|p-|m-|w-|h-|flex|grid|block|hidden).*$",
            // Dark mode variants
            r"^dark:.*$",
            r"^dark:hover:.*$",
            r"^dark:focus:.*$",
            r"^dark:active:.*$",
            r"^dark:disabled:.*$",
            r"^dark:checked:.*$",
            r"^dark:group-hover:.*$",
            r"^dark:group-focus:.*$",
            // Gradient utilities
            r"^bg-gradient-to-(r|l|t|b|tr|tl|br|bl)$",
            r"^from-[a-zA-Z-]+-\d+$",
            r"^via-[a-zA-Z-]+-\d+$",
            r"^to-[a-zA-Z-]+-\d+$",
            // Animation utilities (Enhanced with new animation types)
            r"^animate-(none|spin|ping|pulse|bounce|fade-in|fade-out|slide-in-left|slide-in-right|slide-in-top|slide-in-bottom|zoom-in|zoom-out|wobble|shake|flip|heartbeat)$",
            // Enhanced animation controls
            r"^animation-iteration-count-\d+$",
            r"^animation-play-state-(paused|running)$",
            // State-based animations
            r"^hover:animate-(none|spin|ping|pulse|bounce|fade-in|fade-out|slide-in-left|slide-in-right|slide-in-top|slide-in-bottom|zoom-in|zoom-out|wobble|shake|flip|heartbeat)$",
            r"^focus:animate-(none|spin|ping|pulse|bounce|fade-in|fade-out|slide-in-left|slide-in-right|slide-in-top|slide-in-bottom|zoom-in|zoom-out|wobble|shake|flip|heartbeat)$",
            // Transition utilities
            r"^transition-(none|all|colors|opacity|shadow|transform)$",
            r"^duration-(75|100|150|200|300|500|700|1000)$",
            r"^delay-(75|100|150|200|300|500|700|1000)$",
            r"^ease-(linear|in|out|in-out)$",
            // Arbitrary values
            r"^[a-zA-Z-]+-\[[^\]]+\]$",
        ];

        for pattern in patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                self.allowed_patterns.push(regex);
            }
        }
    }

    /// Add class conflicts
    fn add_class_conflicts(&mut self) {
        // Display conflicts
        let mut display_conflicts = HashSet::new();
        display_conflicts.insert("block".to_string());
        display_conflicts.insert("inline-block".to_string());
        display_conflicts.insert("inline".to_string());
        display_conflicts.insert("flex".to_string());
        display_conflicts.insert("inline-flex".to_string());
        display_conflicts.insert("grid".to_string());
        display_conflicts.insert("inline-grid".to_string());
        display_conflicts.insert("hidden".to_string());
        self.class_conflicts
            .insert("display".to_string(), display_conflicts);

        // Position conflicts
        let mut position_conflicts = HashSet::new();
        position_conflicts.insert("static".to_string());
        position_conflicts.insert("fixed".to_string());
        position_conflicts.insert("absolute".to_string());
        position_conflicts.insert("relative".to_string());
        position_conflicts.insert("sticky".to_string());
        self.class_conflicts
            .insert("position".to_string(), position_conflicts);
    }

    /// Add required classes
    fn add_required_classes(&mut self) {
        // Grid requires display: grid
        let mut grid_required = HashSet::new();
        grid_required.insert("grid".to_string());
        self.required_classes
            .insert("grid-cols-".to_string(), grid_required.clone());
        self.required_classes
            .insert("grid-rows-".to_string(), grid_required.clone());
        self.required_classes
            .insert("gap-".to_string(), grid_required);

        // Flex requires display: flex
        let mut flex_required = HashSet::new();
        flex_required.insert("flex".to_string());
        self.required_classes
            .insert("flex-".to_string(), flex_required.clone());
        self.required_classes
            .insert("items-".to_string(), flex_required.clone());
        self.required_classes
            .insert("justify-".to_string(), flex_required);
    }

    /// Add a custom allowed pattern
    pub fn add_allowed_pattern(&mut self, pattern: regex::Regex) {
        self.allowed_patterns.push(pattern);
    }

    /// Add a custom forbidden pattern
    pub fn add_forbidden_pattern(&mut self, pattern: regex::Regex) {
        self.forbidden_patterns.push(pattern);
    }

    /// Add a deprecated class
    pub fn add_deprecated_class(&mut self, class: String) {
        self.deprecated_classes.insert(class);
    }

    /// Add a class conflict
    pub fn add_class_conflict(&mut self, group: String, class: String) {
        self.class_conflicts
            .entry(group)
            .or_default()
            .insert(class);
    }

    /// Add a required class
    pub fn add_required_class(&mut self, class: String, required: String) {
        self.required_classes
            .entry(class)
            .or_default()
            .insert(required);
    }
}

/// Error reporter for validation errors
pub struct ErrorReporter {
    /// Whether to report errors
    pub enabled: bool,
    /// Error callback
    #[allow(clippy::type_complexity)]
    pub callback: Option<Box<dyn Fn(&ValidationError) + Send + Sync>>,
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorReporter {
    /// Create a new error reporter
    pub fn new() -> Self {
        Self {
            enabled: true,
            callback: None,
        }
    }

    /// Set error callback
    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(&ValidationError) + Send + Sync + 'static,
    {
        self.callback = Some(Box::new(callback));
    }

    /// Report an error
    pub fn report(&self, error: &ValidationError) {
        if self.enabled {
            if let Some(ref callback) = self.callback {
                callback(error);
            }
        }
    }
}

/// Validates Tailwind class names at runtime
pub struct ClassValidator {
    #[allow(dead_code)]
    valid_classes: HashSet<String>,
    validation_rules: ValidationRules,
    error_reporter: ErrorReporter,
    custom_variant_manager: CustomVariantManager,
}

impl Default for ClassValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassValidator {
    /// Create a new validator instance
    pub fn new() -> Self {
        Self {
            valid_classes: HashSet::new(),
            validation_rules: ValidationRules::new(),
            error_reporter: ErrorReporter::new(),
            custom_variant_manager: CustomVariantManager::with_defaults(),
        }
    }

    /// Create a new validator with custom rules
    pub fn with_rules(validation_rules: ValidationRules) -> Self {
        Self {
            valid_classes: HashSet::new(),
            validation_rules,
            error_reporter: ErrorReporter::new(),
            custom_variant_manager: CustomVariantManager::with_defaults(),
        }
    }

    /// Create a new validator with custom variant manager
    pub fn with_custom_variants(custom_variant_manager: CustomVariantManager) -> Self {
        Self {
            valid_classes: HashSet::new(),
            validation_rules: ValidationRules::new(),
            error_reporter: ErrorReporter::new(),
            custom_variant_manager,
        }
    }

    /// Set error reporter
    pub fn set_error_reporter(&mut self, error_reporter: ErrorReporter) {
        self.error_reporter = error_reporter;
    }

    /// Validate a single class name
    pub fn validate_class(&self, class_name: &str) -> std::result::Result<(), ValidationError> {
        // Check if class is deprecated
        if self
            .validation_rules
            .deprecated_classes
            .contains(class_name)
        {
            let error = ValidationError::DeprecatedClass(class_name.to_string());
            self.error_reporter.report(&error);
            return Err(error);
        }

        // Check forbidden patterns
        for pattern in &self.validation_rules.forbidden_patterns {
            if pattern.is_match(class_name) {
                let error = ValidationError::UnsupportedClass(class_name.to_string());
                self.error_reporter.report(&error);
                return Err(error);
            }
        }

        // Check allowed patterns
        let mut is_valid = false;
        for pattern in &self.validation_rules.allowed_patterns {
            if pattern.is_match(class_name) {
                is_valid = true;
                break;
            }
        }

        if !is_valid {
            let error = ValidationError::InvalidClass(class_name.to_string());
            self.error_reporter.report(&error);
            return Err(error);
        }

        Ok(())
    }

    /// Validate multiple class names
    pub fn validate_classes(&self, classes: &[String]) -> std::result::Result<(), ValidationError> {
        for class in classes {
            self.validate_class(class)?;
        }

        // Check for class conflicts
        self.check_class_conflicts(classes)?;

        // Check for required classes
        self.check_required_classes(classes)?;

        Ok(())
    }

    /// Validate a custom variant (Tailwind v4.1.13 @custom-variant support)
    pub fn validate_custom_variant(&self, variant: &str) -> std::result::Result<(), ValidationError> {
        // Use the custom variant manager to validate
        self.custom_variant_manager.validate_variant(variant)
            .map_err(|e| ValidationError::CustomVariantValidation(e.to_string()))
    }

    /// Validate a class with custom variant
    pub fn validate_variant_class(&self, variant: &str, class: &str) -> std::result::Result<(), ValidationError> {
        // Validate the variant first
        self.validate_custom_variant(variant)?;
        
        // Validate the class
        self.validate_class(class)?;
        
        Ok(())
    }

    /// Get suggestions for a custom variant
    pub fn get_variant_suggestions(&self, partial: &str) -> Vec<String> {
        self.custom_variant_manager.get_suggestions(partial)
    }

    /// Register a custom variant
    pub fn register_custom_variant(&mut self, variant: crate::custom_variant::CustomVariant) -> std::result::Result<(), ValidationError> {
        self.custom_variant_manager.register(variant)
            .map_err(|e| ValidationError::CustomVariantValidation(e.to_string()))
    }

    /// Get the custom variant manager
    pub fn custom_variant_manager(&self) -> &CustomVariantManager {
        &self.custom_variant_manager
    }

    /// Get mutable access to the custom variant manager
    pub fn custom_variant_manager_mut(&mut self) -> &mut CustomVariantManager {
        &mut self.custom_variant_manager
    }

    /// Check for class conflicts
    fn check_class_conflicts(
        &self,
        classes: &[String],
    ) -> std::result::Result<(), ValidationError> {
        for conflicting_classes in self.validation_rules.class_conflicts.values() {
            let mut found_classes = Vec::new();

            for class in classes {
                if conflicting_classes.contains(class) {
                    found_classes.push(class.clone());
                }
            }

            if found_classes.len() > 1 {
                let error = ValidationError::ClassConflict(
                    found_classes[0].clone(),
                    found_classes[1].clone(),
                );
                self.error_reporter.report(&error);
                return Err(error);
            }
        }

        Ok(())
    }

    /// Check for required classes
    fn check_required_classes(
        &self,
        classes: &[String],
    ) -> std::result::Result<(), ValidationError> {
        for class in classes {
            for (required_prefix, required_classes) in &self.validation_rules.required_classes {
                if class.starts_with(required_prefix) {
                    let mut has_required = false;
                    for required_class in required_classes {
                        if classes.contains(required_class) {
                            has_required = true;
                            break;
                        }
                    }

                    if !has_required {
                        let error = ValidationError::InvalidClass(format!(
                            "Class '{}' requires one of: {}",
                            class,
                            required_classes
                                .iter()
                                .map(|s| s.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ));
                        self.error_reporter.report(&error);
                        return Err(error);
                    }
                }
            }
        }

        Ok(())
    }

    /// Get validation rules
    pub fn rules(&self) -> &ValidationRules {
        &self.validation_rules
    }

    /// Get mutable validation rules
    pub fn rules_mut(&mut self) -> &mut ValidationRules {
        &mut self.validation_rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_rules_creation() {
        let rules = ValidationRules::new();
        assert!(!rules.allowed_patterns.is_empty());
        assert!(!rules.class_conflicts.is_empty());
        assert!(!rules.required_classes.is_empty());
    }

    #[test]
    fn test_class_validator_creation() {
        let validator = ClassValidator::new();
        assert!(!validator.rules().allowed_patterns.is_empty());
    }

    #[test]
    fn test_validate_valid_class() {
        let validator = ClassValidator::new();
        assert!(validator.validate_class("bg-blue-600").is_ok());
        assert!(validator.validate_class("text-white").is_ok());
        assert!(validator.validate_class("px-4").is_ok());
        assert!(validator.validate_class("py-2").is_ok());
    }
    
    #[test]
    fn test_validate_arbitrary_values() {
        let validator = ClassValidator::new();
        // Test arbitrary values
        assert!(validator.validate_class("w-[123px]").is_ok());
        assert!(validator.validate_class("bg-[#ff0000]").is_ok());
        assert!(validator.validate_class("text-[14px]").is_ok());
        assert!(validator.validate_class("p-[1.5rem]").is_ok());
        assert!(validator.validate_class("m-[2rem]").is_ok());
        assert!(validator.validate_class("rounded-[8px]").is_ok());
        assert!(validator.validate_class("shadow-[0_4px_6px_rgba(0,0,0,0.1)]").is_ok());
        assert!(validator.validate_class("opacity-[0.5]").is_ok());
        assert!(validator.validate_class("z-[999]").is_ok());
        assert!(validator.validate_class("top-[10px]").is_ok());
        assert!(validator.validate_class("right-[20px]").is_ok());
        assert!(validator.validate_class("bottom-[30px]").is_ok());
        assert!(validator.validate_class("left-[40px]").is_ok());
    }
    
    #[test]
    fn test_validate_dark_mode_variants() {
        let validator = ClassValidator::new();
        // Test dark mode variants
        assert!(validator.validate_class("dark:bg-gray-800").is_ok());
        assert!(validator.validate_class("dark:text-white").is_ok());
        assert!(validator.validate_class("dark:border-gray-700").is_ok());
        assert!(validator.validate_class("dark:hover:bg-gray-700").is_ok());
        assert!(validator.validate_class("dark:focus:bg-gray-600").is_ok());
        assert!(validator.validate_class("dark:active:bg-gray-500").is_ok());
        assert!(validator.validate_class("dark:disabled:bg-gray-400").is_ok());
        assert!(validator.validate_class("dark:checked:bg-gray-300").is_ok());
        assert!(validator.validate_class("dark:group-hover:bg-gray-200").is_ok());
        assert!(validator.validate_class("dark:group-focus:bg-gray-100").is_ok());
    }
    
    #[test]
    fn test_validate_gradient_variants() {
        let validator = ClassValidator::new();
        // Test gradient variants
        assert!(validator.validate_class("bg-gradient-to-r").is_ok());
        assert!(validator.validate_class("bg-gradient-to-l").is_ok());
        assert!(validator.validate_class("bg-gradient-to-t").is_ok());
        assert!(validator.validate_class("bg-gradient-to-b").is_ok());
        assert!(validator.validate_class("bg-gradient-to-tr").is_ok());
        assert!(validator.validate_class("bg-gradient-to-tl").is_ok());
        assert!(validator.validate_class("bg-gradient-to-br").is_ok());
        assert!(validator.validate_class("bg-gradient-to-bl").is_ok());
        assert!(validator.validate_class("from-blue-500").is_ok());
        assert!(validator.validate_class("via-purple-500").is_ok());
        assert!(validator.validate_class("to-pink-500").is_ok());
    }
    
    #[test]
    fn test_validate_fractional_spacing() {
        let validator = ClassValidator::new();
        // Test fractional spacing values
        assert!(validator.validate_class("p-0.5").is_ok());
        assert!(validator.validate_class("p-1.5").is_ok());
        assert!(validator.validate_class("p-2.5").is_ok());
        assert!(validator.validate_class("p-3.5").is_ok());
        assert!(validator.validate_class("m-0.5").is_ok());
        assert!(validator.validate_class("m-1.5").is_ok());
        assert!(validator.validate_class("m-2.5").is_ok());
        assert!(validator.validate_class("m-3.5").is_ok());
        assert!(validator.validate_class("px-0.5").is_ok());
        assert!(validator.validate_class("py-1.5").is_ok());
        assert!(validator.validate_class("pt-2.5").is_ok());
        assert!(validator.validate_class("pr-3.5").is_ok());
        assert!(validator.validate_class("pb-0.5").is_ok());
        assert!(validator.validate_class("pl-1.5").is_ok());
        assert!(validator.validate_class("mx-2.5").is_ok());
        assert!(validator.validate_class("my-3.5").is_ok());
        assert!(validator.validate_class("mt-0.5").is_ok());
        assert!(validator.validate_class("mr-1.5").is_ok());
        assert!(validator.validate_class("mb-2.5").is_ok());
        assert!(validator.validate_class("ml-3.5").is_ok());
    }
    
    #[test]
    fn test_validate_animation_system() {
        let validator = ClassValidator::new();
        // Test animation classes
        assert!(validator.validate_class("animate-none").is_ok());
        assert!(validator.validate_class("animate-spin").is_ok());
        assert!(validator.validate_class("animate-ping").is_ok());
        assert!(validator.validate_class("animate-pulse").is_ok());
        assert!(validator.validate_class("animate-bounce").is_ok());
        
        // Test transition classes
        assert!(validator.validate_class("transition-none").is_ok());
        assert!(validator.validate_class("transition-all").is_ok());
        assert!(validator.validate_class("transition-colors").is_ok());
        assert!(validator.validate_class("transition-opacity").is_ok());
        assert!(validator.validate_class("transition-shadow").is_ok());
        assert!(validator.validate_class("transition-transform").is_ok());
        
        // Test transition duration classes
        assert!(validator.validate_class("duration-75").is_ok());
        assert!(validator.validate_class("duration-100").is_ok());
        assert!(validator.validate_class("duration-150").is_ok());
        assert!(validator.validate_class("duration-200").is_ok());
        assert!(validator.validate_class("duration-300").is_ok());
        assert!(validator.validate_class("duration-500").is_ok());
        assert!(validator.validate_class("duration-700").is_ok());
        assert!(validator.validate_class("duration-1000").is_ok());
        
        // Test transition delay classes
        assert!(validator.validate_class("delay-75").is_ok());
        assert!(validator.validate_class("delay-100").is_ok());
        assert!(validator.validate_class("delay-150").is_ok());
        assert!(validator.validate_class("delay-200").is_ok());
        assert!(validator.validate_class("delay-300").is_ok());
        assert!(validator.validate_class("delay-500").is_ok());
        assert!(validator.validate_class("delay-700").is_ok());
        assert!(validator.validate_class("delay-1000").is_ok());
        
        // Test transition timing function classes
        assert!(validator.validate_class("ease-linear").is_ok());
        assert!(validator.validate_class("ease-in").is_ok());
        assert!(validator.validate_class("ease-out").is_ok());
        assert!(validator.validate_class("ease-in-out").is_ok());
    }

    #[test]
    fn test_validate_invalid_class() {
        let validator = ClassValidator::new();
        assert!(validator.validate_class("invalid-class").is_err());
        assert!(validator.validate_class("bg-invalid-color").is_err());
    }

    #[test]
    fn test_validate_deprecated_class() {
        let mut validator = ClassValidator::new();
        validator
            .rules_mut()
            .add_deprecated_class("deprecated-class".to_string());

        assert!(validator.validate_class("deprecated-class").is_err());
    }

    #[test]
    fn test_validate_class_conflicts() {
        let validator = ClassValidator::new();
        let classes = vec!["block".to_string(), "flex".to_string()];

        assert!(validator.validate_classes(&classes).is_err());
    }

    #[test]
    fn test_validate_required_classes() {
        let validator = ClassValidator::new();
        let classes = vec!["grid-cols-2".to_string()];

        // This should fail because grid-cols-2 requires display: grid
        assert!(validator.validate_classes(&classes).is_err());
    }

    #[test]
    fn test_validate_multiple_classes() {
        let validator = ClassValidator::new();
        let classes = vec![
            "bg-blue-600".to_string(),
            "text-white".to_string(),
            "px-4".to_string(),
            "py-2".to_string(),
        ];

        assert!(validator.validate_classes(&classes).is_ok());
    }

    #[test]
    fn test_error_reporter() {
        let mut reporter = ErrorReporter::new();
        let error_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let error_count_clone = error_count.clone();

        reporter.set_callback(move |_error| {
            error_count_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });

        let error = ValidationError::InvalidClass("test".to_string());
        reporter.report(&error);

        assert_eq!(error_count.load(std::sync::atomic::Ordering::Relaxed), 1);
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::InvalidClass("test".to_string());
        assert_eq!(format!("{}", error), "Invalid class name: test");

        let error = ValidationError::ClassConflict("class1".to_string(), "class2".to_string());
        assert_eq!(
            format!("{}", error),
            "Class conflict: class1 conflicts with class2"
        );

        let error = ValidationError::DeprecatedClass("deprecated".to_string());
        assert_eq!(format!("{}", error), "Deprecated class: deprecated");

        let error = ValidationError::UnsupportedClass("unsupported".to_string());
        assert_eq!(format!("{}", error), "Unsupported class: unsupported");
    }

    #[test]
    fn test_validate_enhanced_animations() {
        let validator = ClassValidator::new();

        // Valid basic animations
        assert!(validator.validate_class("animate-spin").is_ok());
        assert!(validator.validate_class("animate-bounce").is_ok());
        assert!(validator.validate_class("animate-pulse").is_ok());

        // Valid extended animations
        assert!(validator.validate_class("animate-fade-in").is_ok());
        assert!(validator.validate_class("animate-fade-out").is_ok());
        assert!(validator.validate_class("animate-slide-in-left").is_ok());
        assert!(validator.validate_class("animate-slide-in-right").is_ok());
        assert!(validator.validate_class("animate-slide-in-top").is_ok());
        assert!(validator.validate_class("animate-slide-in-bottom").is_ok());
        assert!(validator.validate_class("animate-zoom-in").is_ok());
        assert!(validator.validate_class("animate-zoom-out").is_ok());
        assert!(validator.validate_class("animate-wobble").is_ok());
        assert!(validator.validate_class("animate-shake").is_ok());
        assert!(validator.validate_class("animate-flip").is_ok());
        assert!(validator.validate_class("animate-heartbeat").is_ok());

        // Valid animation controls
        assert!(validator.validate_class("animation-iteration-count-1").is_ok());
        assert!(validator.validate_class("animation-iteration-count-3").is_ok());
        assert!(validator.validate_class("animation-play-state-paused").is_ok());
        assert!(validator.validate_class("animation-play-state-running").is_ok());

        // Valid state-based animations
        assert!(validator.validate_class("hover:animate-bounce").is_ok());
        assert!(validator.validate_class("focus:animate-pulse").is_ok());
        assert!(validator.validate_class("hover:animate-fade-in").is_ok());
        assert!(validator.validate_class("focus:animate-slide-in-left").is_ok());

        // Invalid animation classes
        assert!(validator.validate_class("animate-invalid").is_err());
        assert!(validator.validate_class("animate-unknown-animation").is_err());
        assert!(validator.validate_class("hover:animate-invalid").is_err());
    }

    #[test]
    fn test_validate_enhanced_spacing() {
        let validator = ClassValidator::new();

        // Valid enhanced spacing with fractional values
        assert!(validator.validate_class("p-0.5").is_ok());
        assert!(validator.validate_class("p-1.5").is_ok());
        assert!(validator.validate_class("p-2.5").is_ok());
        assert!(validator.validate_class("p-3.5").is_ok());
        assert!(validator.validate_class("m-0.5").is_ok());
        assert!(validator.validate_class("m-1.5").is_ok());

        // Valid enhanced spacing with new directions
        assert!(validator.validate_class("ps-4").is_ok());
        assert!(validator.validate_class("pe-4").is_ok());
        assert!(validator.validate_class("ms-4").is_ok());
        assert!(validator.validate_class("me-4").is_ok());

        // Valid negative margins
        assert!(validator.validate_class("-m-4").is_ok());
        assert!(validator.validate_class("-mt-2").is_ok());
        assert!(validator.validate_class("-mx-3").is_ok());

        // Valid space and divide utilities
        assert!(validator.validate_class("space-x-4").is_ok());
        assert!(validator.validate_class("space-y-2").is_ok());
        assert!(validator.validate_class("space-x-reverse").is_ok());
        assert!(validator.validate_class("space-y-reverse").is_ok());
        assert!(validator.validate_class("divide-x-2").is_ok());
        assert!(validator.validate_class("divide-y-4").is_ok());
        assert!(validator.validate_class("divide-x-reverse").is_ok());
        assert!(validator.validate_class("divide-y-reverse").is_ok());

        // Valid fractional space utilities
        assert!(validator.validate_class("space-x-0.5").is_ok());
        assert!(validator.validate_class("space-y-1.5").is_ok());
        assert!(validator.validate_class("divide-x-0.5").is_ok());
        assert!(validator.validate_class("divide-y-2.5").is_ok());
    }

    #[test]
    fn test_validate_comprehensive_modern_features() {
        let validator = ClassValidator::new();

        // Test comprehensive validation of all new features
        let modern_classes = vec![
            // Enhanced animations
            "animate-fade-in",
            "animate-slide-in-right",
            "animate-zoom-out",
            "animate-heartbeat",
            "hover:animate-wobble",
            "focus:animate-shake",
            "animation-iteration-count-2",
            "animation-play-state-paused",

            // Enhanced spacing
            "p-1.5",
            "m-2.5",
            "ps-4",
            "me-6",
            "-mx-8",
            "space-x-0.5",
            "divide-y-3.5",
            "space-x-reverse",
            "divide-y-reverse",

            // Dark mode variants
            "dark:bg-gray-800",
            "dark:text-white",
            "dark:hover:bg-gray-700",
            "dark:focus:text-gray-100",

            // Gradients
            "bg-gradient-to-r",
            "from-blue-500",
            "via-purple-500",
            "to-pink-500",

            // Arbitrary values
            "w-[123px]",
            "bg-[#ff0000]",
            "text-[14px]",
            "p-[1.5rem]",
        ];

        for class in modern_classes {
            assert!(
                validator.validate_class(class).is_ok(),
                "Failed to validate modern class: {}",
                class
            );
        }
    }
}
