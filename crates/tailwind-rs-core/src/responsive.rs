//! Responsive design system for tailwind-rs

use crate::error::{Result, TailwindError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Breakpoint definitions for responsive design
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Breakpoint {
    /// Base breakpoint (no prefix)
    Base,
    /// Small screens (640px and up)
    Sm,
    /// Medium screens (768px and up)
    Md,
    /// Large screens (1024px and up)
    Lg,
    /// Extra large screens (1280px and up)
    Xl,
    /// 2X large screens (1536px and up)
    Xl2,
}

/// State definitions for pseudo-classes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum State {
    /// Hover state
    Hover,
    /// Focus state
    Focus,
    /// Active state
    Active,
    /// Visited state
    Visited,
    /// Disabled state
    Disabled,
    /// Checked state
    Checked,
    /// Indeterminate state
    Indeterminate,
    /// Required state
    Required,
    /// Valid state
    Valid,
    /// Invalid state
    Invalid,
    /// In-range state
    InRange,
    /// Out-of-range state
    OutOfRange,
    /// Read-only state
    ReadOnly,
    /// Read-write state
    ReadWrite,
    /// Optional state
    Optional,
    /// Placeholder-shown state
    PlaceholderShown,
    /// Autofill state
    Autofill,
    /// Default state
    Default,
    /// First-child state
    FirstChild,
    /// Last-child state
    LastChild,
    /// Only-child state
    OnlyChild,
    /// First-of-type state
    FirstOfType,
    /// Last-of-type state
    LastOfType,
    /// Only-of-type state
    OnlyOfType,
    /// Empty state
    Empty,
    /// Target state
    Target,
    /// Root state
    Root,
    /// Not state
    Not,
    /// Where state
    Where,
    /// Is state
    Is,
    /// Has state
    Has,
    /// Before state
    Before,
    /// After state
    After,
    /// First-letter state
    FirstLetter,
    /// First-line state
    FirstLine,
    /// Selection state
    Selection,
    /// Marker state
    Marker,
    /// Placeholder state
    Placeholder,
    /// File state
    File,
    /// Backdrop state
    Backdrop,
    /// Any-link state
    AnyLink,
    /// Link state
    Link,
    /// Local-link state
    LocalLink,
    /// Scope state
    Scope,
    /// Current state
    Current,
    /// Past state
    Past,
    /// Future state
    Future,
    /// Playing state
    Playing,
    /// Paused state
    Paused,
    /// Seeking state
    Seeking,
    /// Buffering state
    Buffering,
    /// Stalled state
    Stalled,
    /// Muted state
    Muted,
    /// Volume-locked state
    VolumeLocked,
    /// User-invalid state
    UserInvalid,
    /// User-valid state
    UserValid,
    /// Modal state
    Modal,
    /// Picture-in-picture state
    PictureInPicture,
    /// Fullscreen state
    Fullscreen,
    /// Resize state
    Resize,
    /// Scroll state
    Scroll,
    /// Snap state
    Snap,
    /// Touch state
    Touch,
    /// User-select state
    UserSelect,
    /// Will-change state
    WillChange,
    /// Accent-color state
    AccentColor,
    /// Appearance state
    Appearance,
    /// Cursor state
    Cursor,
    /// Outline state
    Outline,
}

impl State {
    /// Get the CSS prefix for this state
    pub fn prefix(&self) -> String {
        match self {
            State::Hover => "hover:".to_string(),
            State::Focus => "focus:".to_string(),
            State::Active => "active:".to_string(),
            State::Visited => "visited:".to_string(),
            State::Disabled => "disabled:".to_string(),
            State::Checked => "checked:".to_string(),
            State::Indeterminate => "indeterminate:".to_string(),
            State::Required => "required:".to_string(),
            State::Valid => "valid:".to_string(),
            State::Invalid => "invalid:".to_string(),
            State::InRange => "in-range:".to_string(),
            State::OutOfRange => "out-of-range:".to_string(),
            State::ReadOnly => "read-only:".to_string(),
            State::ReadWrite => "read-write:".to_string(),
            State::Optional => "optional:".to_string(),
            State::PlaceholderShown => "placeholder-shown:".to_string(),
            State::Autofill => "autofill:".to_string(),
            State::Default => "default:".to_string(),
            State::FirstChild => "first-child:".to_string(),
            State::LastChild => "last-child:".to_string(),
            State::OnlyChild => "only-child:".to_string(),
            State::FirstOfType => "first-of-type:".to_string(),
            State::LastOfType => "last-of-type:".to_string(),
            State::OnlyOfType => "only-of-type:".to_string(),
            State::Empty => "empty:".to_string(),
            State::Target => "target:".to_string(),
            State::Root => "root:".to_string(),
            State::Not => "not:".to_string(),
            State::Where => "where:".to_string(),
            State::Is => "is:".to_string(),
            State::Has => "has:".to_string(),
            State::Before => "before:".to_string(),
            State::After => "after:".to_string(),
            State::FirstLetter => "first-letter:".to_string(),
            State::FirstLine => "first-line:".to_string(),
            State::Selection => "selection:".to_string(),
            State::Marker => "marker:".to_string(),
            State::Placeholder => "placeholder:".to_string(),
            State::File => "file:".to_string(),
            State::Backdrop => "backdrop:".to_string(),
            State::AnyLink => "any-link:".to_string(),
            State::Link => "link:".to_string(),
            State::LocalLink => "local-link:".to_string(),
            State::Scope => "scope:".to_string(),
            State::Current => "current:".to_string(),
            State::Past => "past:".to_string(),
            State::Future => "future:".to_string(),
            State::Playing => "playing:".to_string(),
            State::Paused => "paused:".to_string(),
            State::Seeking => "seeking:".to_string(),
            State::Buffering => "buffering:".to_string(),
            State::Stalled => "stalled:".to_string(),
            State::Muted => "muted:".to_string(),
            State::VolumeLocked => "volume-locked:".to_string(),
            State::UserInvalid => "user-invalid:".to_string(),
            State::UserValid => "user-valid:".to_string(),
            State::Modal => "modal:".to_string(),
            State::PictureInPicture => "picture-in-picture:".to_string(),
            State::Fullscreen => "fullscreen:".to_string(),
            State::Resize => "resize:".to_string(),
            State::Scroll => "scroll:".to_string(),
            State::Snap => "snap:".to_string(),
            State::Touch => "touch:".to_string(),
            State::UserSelect => "user-select:".to_string(),
            State::WillChange => "will-change:".to_string(),
            State::AccentColor => "accent-color:".to_string(),
            State::Appearance => "appearance:".to_string(),
            State::Cursor => "cursor:".to_string(),
            State::Outline => "outline:".to_string(),
        }
    }

    /// Parse a state from a string
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "hover" => Ok(State::Hover),
            "focus" => Ok(State::Focus),
            "active" => Ok(State::Active),
            "visited" => Ok(State::Visited),
            "disabled" => Ok(State::Disabled),
            "checked" => Ok(State::Checked),
            "indeterminate" => Ok(State::Indeterminate),
            "required" => Ok(State::Required),
            "valid" => Ok(State::Valid),
            "invalid" => Ok(State::Invalid),
            "in-range" => Ok(State::InRange),
            "out-of-range" => Ok(State::OutOfRange),
            "read-only" => Ok(State::ReadOnly),
            "read-write" => Ok(State::ReadWrite),
            "optional" => Ok(State::Optional),
            "placeholder-shown" => Ok(State::PlaceholderShown),
            "autofill" => Ok(State::Autofill),
            "default" => Ok(State::Default),
            "first-child" => Ok(State::FirstChild),
            "last-child" => Ok(State::LastChild),
            "only-child" => Ok(State::OnlyChild),
            "first-of-type" => Ok(State::FirstOfType),
            "last-of-type" => Ok(State::LastOfType),
            "only-of-type" => Ok(State::OnlyOfType),
            "empty" => Ok(State::Empty),
            "target" => Ok(State::Target),
            "root" => Ok(State::Root),
            "not" => Ok(State::Not),
            "where" => Ok(State::Where),
            "is" => Ok(State::Is),
            "has" => Ok(State::Has),
            "before" => Ok(State::Before),
            "after" => Ok(State::After),
            "first-letter" => Ok(State::FirstLetter),
            "first-line" => Ok(State::FirstLine),
            "selection" => Ok(State::Selection),
            "marker" => Ok(State::Marker),
            "placeholder" => Ok(State::Placeholder),
            "file" => Ok(State::File),
            "backdrop" => Ok(State::Backdrop),
            "any-link" => Ok(State::AnyLink),
            "link" => Ok(State::Link),
            "local-link" => Ok(State::LocalLink),
            "scope" => Ok(State::Scope),
            "current" => Ok(State::Current),
            "past" => Ok(State::Past),
            "future" => Ok(State::Future),
            "playing" => Ok(State::Playing),
            "paused" => Ok(State::Paused),
            "seeking" => Ok(State::Seeking),
            "buffering" => Ok(State::Buffering),
            "stalled" => Ok(State::Stalled),
            "muted" => Ok(State::Muted),
            "volume-locked" => Ok(State::VolumeLocked),
            "user-invalid" => Ok(State::UserInvalid),
            "user-valid" => Ok(State::UserValid),
            "modal" => Ok(State::Modal),
            "picture-in-picture" => Ok(State::PictureInPicture),
            "fullscreen" => Ok(State::Fullscreen),
            "resize" => Ok(State::Resize),
            "scroll" => Ok(State::Scroll),
            "snap" => Ok(State::Snap),
            "touch" => Ok(State::Touch),
            "user-select" => Ok(State::UserSelect),
            "will-change" => Ok(State::WillChange),
            "accent-color" => Ok(State::AccentColor),
            "appearance" => Ok(State::Appearance),
            "cursor" => Ok(State::Cursor),
            "outline" => Ok(State::Outline),
            _ => Err(TailwindError::config(format!("Invalid state: {}", s))),
        }
    }
}

impl Breakpoint {
    /// Get the minimum width for this breakpoint in pixels
    pub fn min_width(&self) -> u32 {
        match self {
            Breakpoint::Base => 0,
            Breakpoint::Sm => 640,
            Breakpoint::Md => 768,
            Breakpoint::Lg => 1024,
            Breakpoint::Xl => 1280,
            Breakpoint::Xl2 => 1536,
        }
    }

    /// Get the CSS media query for this breakpoint
    pub fn media_query(&self) -> String {
        match self {
            Breakpoint::Base => String::new(),
            _ => format!("(min-width: {}px)", self.min_width()),
        }
    }

    /// Get the Tailwind CSS prefix for this breakpoint
    pub fn prefix(&self) -> String {
        match self {
            Breakpoint::Base => String::new(),
            Breakpoint::Sm => "sm:".to_string(),
            Breakpoint::Md => "md:".to_string(),
            Breakpoint::Lg => "lg:".to_string(),
            Breakpoint::Xl => "xl:".to_string(),
            Breakpoint::Xl2 => "2xl:".to_string(),
        }
    }

    /// Parse a breakpoint from a string
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "" | "base" => Ok(Breakpoint::Base),
            "sm" => Ok(Breakpoint::Sm),
            "md" => Ok(Breakpoint::Md),
            "lg" => Ok(Breakpoint::Lg),
            "xl" => Ok(Breakpoint::Xl),
            "2xl" => Ok(Breakpoint::Xl2),
            _ => Err(TailwindError::config(format!("Invalid breakpoint: {}", s))),
        }
    }
}

/// Represents a responsive value that can have different values at different breakpoints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveValue<T> {
    pub base: T,
    pub sm: Option<T>,
    pub md: Option<T>,
    pub lg: Option<T>,
    pub xl: Option<T>,
    pub xl2: Option<T>,
}

impl<T> ResponsiveValue<T> {
    /// Create a new responsive value with only a base value
    pub fn new(base: T) -> Self {
        Self {
            base,
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xl2: None,
        }
    }

    /// Set a value for a specific breakpoint
    pub fn set_breakpoint(&mut self, breakpoint: Breakpoint, value: T) {
        match breakpoint {
            Breakpoint::Base => self.base = value,
            Breakpoint::Sm => self.sm = Some(value),
            Breakpoint::Md => self.md = Some(value),
            Breakpoint::Lg => self.lg = Some(value),
            Breakpoint::Xl => self.xl = Some(value),
            Breakpoint::Xl2 => self.xl2 = Some(value),
        }
    }

    /// Get a value for a specific breakpoint
    pub fn get_breakpoint(&self, breakpoint: Breakpoint) -> &T {
        match breakpoint {
            Breakpoint::Base => &self.base,
            Breakpoint::Sm => self.sm.as_ref().unwrap_or(&self.base),
            Breakpoint::Md => self.md.as_ref().unwrap_or(&self.base),
            Breakpoint::Lg => self.lg.as_ref().unwrap_or(&self.base),
            Breakpoint::Xl => self.xl.as_ref().unwrap_or(&self.base),
            Breakpoint::Xl2 => self.xl2.as_ref().unwrap_or(&self.base),
        }
    }

    /// Get all breakpoint values as a vector
    pub fn all_values(&self) -> Vec<(Breakpoint, &T)> {
        let mut values = vec![(Breakpoint::Base, &self.base)];

        if let Some(ref sm) = self.sm {
            values.push((Breakpoint::Sm, sm));
        }
        if let Some(ref md) = self.md {
            values.push((Breakpoint::Md, md));
        }
        if let Some(ref lg) = self.lg {
            values.push((Breakpoint::Lg, lg));
        }
        if let Some(ref xl) = self.xl {
            values.push((Breakpoint::Xl, xl));
        }
        if let Some(ref xl2) = self.xl2 {
            values.push((Breakpoint::Xl2, xl2));
        }

        values
    }
}

/// Responsive configuration for the theme system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveConfig {
    pub breakpoints: HashMap<String, u32>,
    pub container_centering: bool,
    pub container_padding: ResponsiveValue<u32>,
}

impl ResponsiveConfig {
    /// Create a new responsive configuration with default breakpoints
    pub fn new() -> Self {
        let mut breakpoints = HashMap::new();
        breakpoints.insert("sm".to_string(), 640);
        breakpoints.insert("md".to_string(), 768);
        breakpoints.insert("lg".to_string(), 1024);
        breakpoints.insert("xl".to_string(), 1280);
        breakpoints.insert("2xl".to_string(), 1536);

        Self {
            breakpoints,
            container_centering: true,
            container_padding: ResponsiveValue::new(16),
        }
    }

    /// Get the breakpoint value for a given name
    pub fn get_breakpoint(&self, name: &str) -> Result<u32> {
        self.breakpoints
            .get(name)
            .copied()
            .ok_or_else(|| TailwindError::config(format!("Breakpoint '{}' not found", name)))
    }

    /// Set a custom breakpoint value
    pub fn set_breakpoint(&mut self, name: String, value: u32) {
        self.breakpoints.insert(name, value);
    }
}

impl Default for ResponsiveConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// The primary type for responsive class generation
#[derive(Debug, Clone, PartialEq)]
pub struct Responsive {
    pub sm: Option<String>,
    pub md: Option<String>,
    pub lg: Option<String>,
    pub xl: Option<String>,
    pub xl2: Option<String>,
}

impl Responsive {
    /// Create a new responsive configuration
    pub fn new() -> Self {
        Self {
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xl2: None,
        }
    }

    /// Set classes for small screens (640px+)
    pub fn sm(mut self, classes: &str) -> Self {
        self.sm = Some(classes.to_string());
        self
    }

    /// Set classes for medium screens (768px+)
    pub fn md(mut self, classes: &str) -> Self {
        self.md = Some(classes.to_string());
        self
    }

    /// Set classes for large screens (1024px+)
    pub fn lg(mut self, classes: &str) -> Self {
        self.lg = Some(classes.to_string());
        self
    }

    /// Set classes for extra large screens (1280px+)
    pub fn xl(mut self, classes: &str) -> Self {
        self.xl = Some(classes.to_string());
        self
    }

    /// Set classes for extra extra large screens (1536px+)
    pub fn xl2(mut self, classes: &str) -> Self {
        self.xl2 = Some(classes.to_string());
        self
    }

    /// Build the final responsive class string
    pub fn build(self) -> String {
        let mut classes = Vec::new();

        if let Some(sm) = self.sm {
            classes.push(format!("sm:{}", sm));
        }
        if let Some(md) = self.md {
            classes.push(format!("md:{}", md));
        }
        if let Some(lg) = self.lg {
            classes.push(format!("lg:{}", lg));
        }
        if let Some(xl) = self.xl {
            classes.push(format!("xl:{}", xl));
        }
        if let Some(xl2) = self.xl2 {
            classes.push(format!("2xl:{}", xl2));
        }

        classes.join(" ")
    }
}

impl Default for Responsive {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder pattern for responsive class generation
#[derive(Debug, Clone)]
pub struct ResponsiveBuilder {
    base_classes: Vec<String>,
    breakpoint_classes: HashMap<Breakpoint, Vec<String>>,
}

impl ResponsiveBuilder {
    /// Create a new responsive builder
    pub fn new() -> Self {
        Self {
            base_classes: Vec::new(),
            breakpoint_classes: HashMap::new(),
        }
    }

    /// Add base classes
    pub fn base(mut self, classes: &str) -> Self {
        self.base_classes
            .extend(classes.split_whitespace().map(|s| s.to_string()));
        self
    }

    /// Add classes for a specific breakpoint
    pub fn breakpoint(mut self, breakpoint: Breakpoint, classes: &str) -> Self {
        let breakpoint_classes = self
            .breakpoint_classes
            .entry(breakpoint)
            .or_default();
        breakpoint_classes.extend(classes.split_whitespace().map(|s| s.to_string()));
        self
    }

    /// Build the final responsive class string
    pub fn build(self) -> String {
        let mut classes = Vec::new();

        // Add base classes
        classes.extend(self.base_classes);

        // Add responsive classes sorted by breakpoint
        let mut breakpoint_classes: Vec<(Breakpoint, Vec<String>)> =
            self.breakpoint_classes.into_iter().collect();
        breakpoint_classes.sort_by(|a, b| a.0.min_width().cmp(&b.0.min_width()));

        for (breakpoint, class_list) in breakpoint_classes {
            for class in class_list {
                classes.push(format!("{}{}", breakpoint.prefix(), class));
            }
        }

        classes.join(" ")
    }
}

impl Default for ResponsiveBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Flex direction values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl FlexDirection {
    /// Returns the CSS class for the flex direction
    pub fn class(&self) -> &'static str {
        match self {
            FlexDirection::Row => "flex-row",
            FlexDirection::Column => "flex-col",
            FlexDirection::RowReverse => "flex-row-reverse",
            FlexDirection::ColumnReverse => "flex-col-reverse",
        }
    }
}

/// Flex wrap values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl FlexWrap {
    /// Returns the CSS class for the flex wrap
    pub fn class(&self) -> &'static str {
        match self {
            FlexWrap::NoWrap => "flex-nowrap",
            FlexWrap::Wrap => "flex-wrap",
            FlexWrap::WrapReverse => "flex-wrap-reverse",
        }
    }
}

/// Justify content values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl JustifyContent {
    /// Returns the CSS class for justify content
    pub fn class(&self) -> &'static str {
        match self {
            JustifyContent::Start => "justify-start",
            JustifyContent::End => "justify-end",
            JustifyContent::Center => "justify-center",
            JustifyContent::SpaceBetween => "justify-between",
            JustifyContent::SpaceAround => "justify-around",
            JustifyContent::SpaceEvenly => "justify-evenly",
        }
    }
}

/// Align items values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl AlignItems {
    /// Returns the CSS class for align items
    pub fn class(&self) -> &'static str {
        match self {
            AlignItems::Start => "items-start",
            AlignItems::End => "items-end",
            AlignItems::Center => "items-center",
            AlignItems::Baseline => "items-baseline",
            AlignItems::Stretch => "items-stretch",
        }
    }
}

/// Specialized responsive grid system
#[derive(Debug, Clone)]
pub struct ResponsiveGrid {
    cols: HashMap<Breakpoint, u32>,
    gap: HashMap<Breakpoint, String>,
    auto_fit: bool,
}

impl ResponsiveGrid {
    /// Create a new responsive grid
    pub fn new() -> Self {
        Self {
            cols: HashMap::new(),
            gap: HashMap::new(),
            auto_fit: false,
        }
    }

    /// Set the number of columns for a breakpoint
    pub fn cols(mut self, breakpoint: Breakpoint, count: u32) -> Self {
        self.cols.insert(breakpoint, count);
        self
    }

    /// Set the gap for a breakpoint
    pub fn gap(mut self, breakpoint: Breakpoint, gap: &str) -> Self {
        self.gap.insert(breakpoint, gap.to_string());
        self
    }

    /// Enable or disable auto-fit columns
    pub fn auto_fit(mut self, enabled: bool) -> Self {
        self.auto_fit = enabled;
        self
    }

    /// Build the final grid class string
    pub fn build(self) -> String {
        let mut classes = vec!["grid".to_string()];

        // Add column classes
        let mut cols: Vec<(Breakpoint, u32)> = self.cols.into_iter().collect();
        cols.sort_by(|a, b| a.0.min_width().cmp(&b.0.min_width()));

        for (breakpoint, count) in cols {
            if breakpoint == Breakpoint::Base {
                classes.push(format!("grid-cols-{}", count));
            } else {
                classes.push(format!("{}grid-cols-{}", breakpoint.prefix(), count));
            }
        }

        // Add gap classes
        let mut gaps: Vec<(Breakpoint, String)> = self.gap.into_iter().collect();
        gaps.sort_by(|a, b| a.0.min_width().cmp(&b.0.min_width()));

        for (breakpoint, gap) in gaps {
            if breakpoint == Breakpoint::Base {
                classes.push(gap);
            } else {
                classes.push(format!("{}{}", breakpoint.prefix(), gap));
            }
        }

        // Add auto-fit if enabled
        if self.auto_fit {
            classes.push("grid-cols-auto-fit".to_string());
        }

        classes.join(" ")
    }
}

impl Default for ResponsiveGrid {
    fn default() -> Self {
        Self::new()
    }
}

/// Specialized responsive flexbox system
#[derive(Debug, Clone)]
pub struct ResponsiveFlex {
    direction: HashMap<Breakpoint, FlexDirection>,
    wrap: HashMap<Breakpoint, FlexWrap>,
    justify: HashMap<Breakpoint, JustifyContent>,
    align: HashMap<Breakpoint, AlignItems>,
    gap: HashMap<Breakpoint, String>,
}

impl ResponsiveFlex {
    /// Create a new responsive flex container
    pub fn new() -> Self {
        Self {
            direction: HashMap::new(),
            wrap: HashMap::new(),
            justify: HashMap::new(),
            align: HashMap::new(),
            gap: HashMap::new(),
        }
    }

    /// Set the flex direction for a breakpoint
    pub fn direction(mut self, breakpoint: Breakpoint, direction: FlexDirection) -> Self {
        self.direction.insert(breakpoint, direction);
        self
    }

    /// Set the flex wrap for a breakpoint
    pub fn wrap(mut self, breakpoint: Breakpoint, wrap: FlexWrap) -> Self {
        self.wrap.insert(breakpoint, wrap);
        self
    }

    /// Set the justify content for a breakpoint
    pub fn justify(mut self, breakpoint: Breakpoint, justify: JustifyContent) -> Self {
        self.justify.insert(breakpoint, justify);
        self
    }

    /// Set the align items for a breakpoint
    pub fn align(mut self, breakpoint: Breakpoint, align: AlignItems) -> Self {
        self.align.insert(breakpoint, align);
        self
    }

    /// Set the gap for a breakpoint
    pub fn gap(mut self, breakpoint: Breakpoint, gap: &str) -> Self {
        self.gap.insert(breakpoint, gap.to_string());
        self
    }

    /// Build the final flex class string
    pub fn build(self) -> String {
        let mut classes = vec!["flex".to_string()];

        // Add direction classes
        let mut directions: Vec<(Breakpoint, FlexDirection)> = self.direction.into_iter().collect();
        directions.sort_by(|a, b| a.0.min_width().cmp(&b.0.min_width()));

        for (breakpoint, direction) in directions {
            if breakpoint == Breakpoint::Base {
                classes.push(direction.class().to_string());
            } else {
                classes.push(format!("{}{}", breakpoint.prefix(), direction.class()));
            }
        }

        // Add wrap classes
        let mut wraps: Vec<(Breakpoint, FlexWrap)> = self.wrap.into_iter().collect();
        wraps.sort_by(|a, b| a.0.min_width().cmp(&b.0.min_width()));

        for (breakpoint, wrap) in wraps {
            if breakpoint == Breakpoint::Base {
                classes.push(wrap.class().to_string());
            } else {
                classes.push(format!("{}{}", breakpoint.prefix(), wrap.class()));
            }
        }

        // Add justify classes
        let mut justifies: Vec<(Breakpoint, JustifyContent)> = self.justify.into_iter().collect();
        justifies.sort_by(|a, b| a.0.min_width().cmp(&b.0.min_width()));

        for (breakpoint, justify) in justifies {
            if breakpoint == Breakpoint::Base {
                classes.push(justify.class().to_string());
            } else {
                classes.push(format!("{}{}", breakpoint.prefix(), justify.class()));
            }
        }

        // Add align classes
        let mut aligns: Vec<(Breakpoint, AlignItems)> = self.align.into_iter().collect();
        aligns.sort_by(|a, b| a.0.min_width().cmp(&b.0.min_width()));

        for (breakpoint, align) in aligns {
            if breakpoint == Breakpoint::Base {
                classes.push(align.class().to_string());
            } else {
                classes.push(format!("{}{}", breakpoint.prefix(), align.class()));
            }
        }

        // Add gap classes
        let mut gaps: Vec<(Breakpoint, String)> = self.gap.into_iter().collect();
        gaps.sort_by(|a, b| a.0.min_width().cmp(&b.0.min_width()));

        for (breakpoint, gap) in gaps {
            if breakpoint == Breakpoint::Base {
                classes.push(gap);
            } else {
                classes.push(format!("{}{}", breakpoint.prefix(), gap));
            }
        }

        classes.join(" ")
    }
}

impl Default for ResponsiveFlex {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for responsive design
#[allow(clippy::module_inception)]
pub mod responsive {
    use super::*;

    /// Create a responsive value with a base value
    pub fn base<T>(value: T) -> ResponsiveValue<T> {
        ResponsiveValue::new(value)
    }

    /// Create a responsive value with base and small breakpoint values
    pub fn sm<T>(base: T, sm: T) -> ResponsiveValue<T> {
        let mut responsive = ResponsiveValue::new(base);
        responsive.set_breakpoint(Breakpoint::Sm, sm);
        responsive
    }

    /// Create a responsive value with base, small, and medium breakpoint values
    pub fn md<T>(base: T, sm: T, md: T) -> ResponsiveValue<T> {
        let mut responsive = ResponsiveValue::new(base);
        responsive.set_breakpoint(Breakpoint::Sm, sm);
        responsive.set_breakpoint(Breakpoint::Md, md);
        responsive
    }

    /// Create a responsive value with base, small, medium, and large breakpoint values
    pub fn lg<T>(base: T, sm: T, md: T, lg: T) -> ResponsiveValue<T> {
        let mut responsive = ResponsiveValue::new(base);
        responsive.set_breakpoint(Breakpoint::Sm, sm);
        responsive.set_breakpoint(Breakpoint::Md, md);
        responsive.set_breakpoint(Breakpoint::Lg, lg);
        responsive
    }

    /// Create a responsive value with all breakpoint values
    pub fn xl<T>(base: T, sm: T, md: T, lg: T, xl: T) -> ResponsiveValue<T> {
        let mut responsive = ResponsiveValue::new(base);
        responsive.set_breakpoint(Breakpoint::Sm, sm);
        responsive.set_breakpoint(Breakpoint::Md, md);
        responsive.set_breakpoint(Breakpoint::Lg, lg);
        responsive.set_breakpoint(Breakpoint::Xl, xl);
        responsive
    }

    /// Create a responsive value with all breakpoint values including 2xl
    pub fn xl2<T>(base: T, sm: T, md: T, lg: T, xl: T, xl2: T) -> ResponsiveValue<T> {
        let mut responsive = ResponsiveValue::new(base);
        responsive.set_breakpoint(Breakpoint::Sm, sm);
        responsive.set_breakpoint(Breakpoint::Md, md);
        responsive.set_breakpoint(Breakpoint::Lg, lg);
        responsive.set_breakpoint(Breakpoint::Xl, xl);
        responsive.set_breakpoint(Breakpoint::Xl2, xl2);
        responsive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_min_width() {
        assert_eq!(Breakpoint::Base.min_width(), 0);
        assert_eq!(Breakpoint::Sm.min_width(), 640);
        assert_eq!(Breakpoint::Md.min_width(), 768);
        assert_eq!(Breakpoint::Lg.min_width(), 1024);
        assert_eq!(Breakpoint::Xl.min_width(), 1280);
        assert_eq!(Breakpoint::Xl2.min_width(), 1536);
    }

    #[test]
    fn test_breakpoint_media_query() {
        assert_eq!(Breakpoint::Base.media_query(), "");
        assert_eq!(Breakpoint::Sm.media_query(), "(min-width: 640px)");
        assert_eq!(Breakpoint::Md.media_query(), "(min-width: 768px)");
        assert_eq!(Breakpoint::Lg.media_query(), "(min-width: 1024px)");
        assert_eq!(Breakpoint::Xl.media_query(), "(min-width: 1280px)");
        assert_eq!(Breakpoint::Xl2.media_query(), "(min-width: 1536px)");
    }

    #[test]
    fn test_breakpoint_prefix() {
        assert_eq!(Breakpoint::Base.prefix(), "");
        assert_eq!(Breakpoint::Sm.prefix(), "sm:");
        assert_eq!(Breakpoint::Md.prefix(), "md:");
        assert_eq!(Breakpoint::Lg.prefix(), "lg:");
        assert_eq!(Breakpoint::Xl.prefix(), "xl:");
        assert_eq!(Breakpoint::Xl2.prefix(), "2xl:");
    }

    #[test]
    fn test_breakpoint_from_str() {
        assert_eq!(Breakpoint::from_str("").unwrap(), Breakpoint::Base);
        assert_eq!(Breakpoint::from_str("base").unwrap(), Breakpoint::Base);
        assert_eq!(Breakpoint::from_str("sm").unwrap(), Breakpoint::Sm);
        assert_eq!(Breakpoint::from_str("md").unwrap(), Breakpoint::Md);
        assert_eq!(Breakpoint::from_str("lg").unwrap(), Breakpoint::Lg);
        assert_eq!(Breakpoint::from_str("xl").unwrap(), Breakpoint::Xl);
        assert_eq!(Breakpoint::from_str("2xl").unwrap(), Breakpoint::Xl2);

        assert!(Breakpoint::from_str("invalid").is_err());
    }

    #[test]
    fn test_responsive_value_creation() {
        let responsive = ResponsiveValue::new(10);
        assert_eq!(responsive.base, 10);
        assert_eq!(responsive.sm, None);
        assert_eq!(responsive.md, None);
    }

    #[test]
    fn test_responsive_value_set_get() {
        let mut responsive = ResponsiveValue::new(10);
        responsive.set_breakpoint(Breakpoint::Sm, 20);
        responsive.set_breakpoint(Breakpoint::Md, 30);

        assert_eq!(*responsive.get_breakpoint(Breakpoint::Base), 10);
        assert_eq!(*responsive.get_breakpoint(Breakpoint::Sm), 20);
        assert_eq!(*responsive.get_breakpoint(Breakpoint::Md), 30);
        assert_eq!(*responsive.get_breakpoint(Breakpoint::Lg), 10); // Falls back to base
    }

    #[test]
    fn test_responsive_value_all_values() {
        let mut responsive = ResponsiveValue::new(10);
        responsive.set_breakpoint(Breakpoint::Sm, 20);
        responsive.set_breakpoint(Breakpoint::Md, 30);

        let all_values = responsive.all_values();
        assert_eq!(all_values.len(), 3);
        assert_eq!(all_values[0], (Breakpoint::Base, &10));
        assert_eq!(all_values[1], (Breakpoint::Sm, &20));
        assert_eq!(all_values[2], (Breakpoint::Md, &30));
    }

    #[test]
    fn test_responsive_config() {
        let config = ResponsiveConfig::new();
        assert_eq!(config.get_breakpoint("sm").unwrap(), 640);
        assert_eq!(config.get_breakpoint("md").unwrap(), 768);
        assert_eq!(config.get_breakpoint("lg").unwrap(), 1024);
        assert_eq!(config.get_breakpoint("xl").unwrap(), 1280);
        assert_eq!(config.get_breakpoint("2xl").unwrap(), 1536);

        assert!(config.get_breakpoint("invalid").is_err());
    }

    #[test]
    fn test_responsive_utility_functions() {
        let responsive = responsive::base(10);
        assert_eq!(responsive.base, 10);

        let responsive = responsive::sm(10, 20);
        assert_eq!(responsive.base, 10);
        assert_eq!(responsive.sm, Some(20));

        let responsive = responsive::md(10, 20, 30);
        assert_eq!(responsive.base, 10);
        assert_eq!(responsive.sm, Some(20));
        assert_eq!(responsive.md, Some(30));

        let responsive = responsive::lg(10, 20, 30, 40);
        assert_eq!(responsive.base, 10);
        assert_eq!(responsive.sm, Some(20));
        assert_eq!(responsive.md, Some(30));
        assert_eq!(responsive.lg, Some(40));

        let responsive = responsive::xl(10, 20, 30, 40, 50);
        assert_eq!(responsive.base, 10);
        assert_eq!(responsive.sm, Some(20));
        assert_eq!(responsive.md, Some(30));
        assert_eq!(responsive.lg, Some(40));
        assert_eq!(responsive.xl, Some(50));

        let responsive = responsive::xl2(10, 20, 30, 40, 50, 60);
        assert_eq!(responsive.base, 10);
        assert_eq!(responsive.sm, Some(20));
        assert_eq!(responsive.md, Some(30));
        assert_eq!(responsive.lg, Some(40));
        assert_eq!(responsive.xl, Some(50));
        assert_eq!(responsive.xl2, Some(60));
    }

    #[test]
    fn test_responsive_creation() {
        let responsive = Responsive::new();
        assert!(responsive.sm.is_none());
        assert!(responsive.md.is_none());
    }

    #[test]
    fn test_responsive_build() {
        let responsive = Responsive::new()
            .sm("text-base")
            .md("text-lg")
            .lg("text-xl");

        let classes = responsive.build();
        assert!(classes.contains("sm:text-base"));
        assert!(classes.contains("md:text-lg"));
        assert!(classes.contains("lg:text-xl"));
    }

    #[test]
    fn test_responsive_builder() {
        let classes = ResponsiveBuilder::new()
            .base("text-sm")
            .breakpoint(Breakpoint::Sm, "text-base")
            .breakpoint(Breakpoint::Md, "text-lg")
            .build();

        assert!(classes.contains("text-sm"));
        assert!(classes.contains("sm:text-base"));
        assert!(classes.contains("md:text-lg"));
    }

    #[test]
    fn test_flex_direction() {
        assert_eq!(FlexDirection::Row.class(), "flex-row");
        assert_eq!(FlexDirection::Column.class(), "flex-col");
        assert_eq!(FlexDirection::RowReverse.class(), "flex-row-reverse");
        assert_eq!(FlexDirection::ColumnReverse.class(), "flex-col-reverse");
    }

    #[test]
    fn test_flex_wrap() {
        assert_eq!(FlexWrap::NoWrap.class(), "flex-nowrap");
        assert_eq!(FlexWrap::Wrap.class(), "flex-wrap");
        assert_eq!(FlexWrap::WrapReverse.class(), "flex-wrap-reverse");
    }

    #[test]
    fn test_justify_content() {
        assert_eq!(JustifyContent::Start.class(), "justify-start");
        assert_eq!(JustifyContent::End.class(), "justify-end");
        assert_eq!(JustifyContent::Center.class(), "justify-center");
        assert_eq!(JustifyContent::SpaceBetween.class(), "justify-between");
        assert_eq!(JustifyContent::SpaceAround.class(), "justify-around");
        assert_eq!(JustifyContent::SpaceEvenly.class(), "justify-evenly");
    }

    #[test]
    fn test_align_items() {
        assert_eq!(AlignItems::Start.class(), "items-start");
        assert_eq!(AlignItems::End.class(), "items-end");
        assert_eq!(AlignItems::Center.class(), "items-center");
        assert_eq!(AlignItems::Baseline.class(), "items-baseline");
        assert_eq!(AlignItems::Stretch.class(), "items-stretch");
    }

    #[test]
    fn test_responsive_grid() {
        let classes = ResponsiveGrid::new()
            .cols(Breakpoint::Sm, 2)
            .cols(Breakpoint::Md, 3)
            .gap(Breakpoint::Sm, "gap-4")
            .gap(Breakpoint::Md, "gap-6")
            .build();

        assert!(classes.contains("grid"));
        assert!(classes.contains("sm:grid-cols-2"));
        assert!(classes.contains("md:grid-cols-3"));
        assert!(classes.contains("sm:gap-4"));
        assert!(classes.contains("md:gap-6"));
    }

    #[test]
    fn test_responsive_flex() {
        let classes = ResponsiveFlex::new()
            .direction(Breakpoint::Sm, FlexDirection::Column)
            .direction(Breakpoint::Md, FlexDirection::Row)
            .justify(Breakpoint::Sm, JustifyContent::Center)
            .justify(Breakpoint::Md, JustifyContent::SpaceBetween)
            .build();

        assert!(classes.contains("flex"));
        assert!(classes.contains("sm:flex-col"));
        assert!(classes.contains("md:flex-row"));
        assert!(classes.contains("sm:justify-center"));
        assert!(classes.contains("md:justify-between"));
    }
}
