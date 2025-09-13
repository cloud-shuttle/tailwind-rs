//! Interactivity utilities for tailwind-rs
//!
//! This module provides utilities for CSS interactivity including cursor,
//! pointer events, resize, scroll behavior, scroll snap, touch action,
//! user select, and will change.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Cursor values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cursor {
    /// Auto cursor
    Auto,
    /// Default cursor
    Default,
    /// Pointer cursor
    Pointer,
    /// Wait cursor
    Wait,
    /// Text cursor
    Text,
    /// Move cursor
    Move,
    /// Help cursor
    Help,
    /// Not allowed cursor
    NotAllowed,
    /// None cursor
    None,
    /// Context menu cursor
    ContextMenu,
    /// Progress cursor
    Progress,
    /// Cell cursor
    Cell,
    /// Crosshair cursor
    Crosshair,
    /// Vertical text cursor
    VerticalText,
    /// Alias cursor
    Alias,
    /// Copy cursor
    Copy,
    /// No drop cursor
    NoDrop,
    /// Grab cursor
    Grab,
    /// Grabbing cursor
    Grabbing,
    /// All scroll cursor
    AllScroll,
    /// Col resize cursor
    ColResize,
    /// Row resize cursor
    RowResize,
    /// N resize cursor
    NResize,
    /// E resize cursor
    EResize,
    /// S resize cursor
    SResize,
    /// W resize cursor
    WResize,
    /// Ne resize cursor
    NeResize,
    /// Nw resize cursor
    NwResize,
    /// Se resize cursor
    SeResize,
    /// Sw resize cursor
    SwResize,
    /// Ew resize cursor
    EwResize,
    /// Ns resize cursor
    NsResize,
    /// Nesw resize cursor
    NeswResize,
    /// Nwse resize cursor
    NwseResize,
    /// Zoom in cursor
    ZoomIn,
    /// Zoom out cursor
    ZoomOut,
}

/// Pointer events values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointerEvents {
    /// Auto pointer events
    Auto,
    /// None pointer events
    None,
}

/// Resize values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resize {
    /// None resize
    None,
    /// Both resize
    Both,
    /// Horizontal resize
    Horizontal,
    /// Vertical resize
    Vertical,
    /// Block resize
    Block,
    /// Inline resize
    Inline,
}

/// Scroll behavior values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScrollBehavior {
    /// Auto scroll behavior
    Auto,
    /// Smooth scroll behavior
    Smooth,
}

/// Scroll snap type values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScrollSnapType {
    /// None scroll snap
    None,
    /// X scroll snap
    X,
    /// Y scroll snap
    Y,
    /// Both scroll snap
    Both,
    /// Mandatory scroll snap
    Mandatory,
    /// Proximity scroll snap
    Proximity,
}

/// Scroll snap align values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScrollSnapAlign {
    /// Start scroll snap align
    Start,
    /// End scroll snap align
    End,
    /// Center scroll snap align
    Center,
    /// None scroll snap align
    None,
}

/// Touch action values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TouchAction {
    /// Auto touch action
    Auto,
    /// None touch action
    None,
    /// Pan X touch action
    PanX,
    /// Pan Y touch action
    PanY,
    /// Pan left touch action
    PanLeft,
    /// Pan right touch action
    PanRight,
    /// Pan up touch action
    PanUp,
    /// Pan down touch action
    PanDown,
    /// Manipulation touch action
    Manipulation,
}

/// User select values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserSelect {
    /// None user select
    None,
    /// Text user select
    Text,
    /// All user select
    All,
    /// Auto user select
    Auto,
}

/// Will change values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WillChange {
    /// Auto will change
    Auto,
    /// Scroll position will change
    ScrollPosition,
    /// Contents will change
    Contents,
    /// Transform will change
    Transform,
}

impl Cursor {
    pub fn to_class_name(&self) -> String {
        match self {
            Cursor::Auto => "auto".to_string(),
            Cursor::Default => "default".to_string(),
            Cursor::Pointer => "pointer".to_string(),
            Cursor::Wait => "wait".to_string(),
            Cursor::Text => "text".to_string(),
            Cursor::Move => "move".to_string(),
            Cursor::Help => "help".to_string(),
            Cursor::NotAllowed => "not-allowed".to_string(),
            Cursor::None => "none".to_string(),
            Cursor::ContextMenu => "context-menu".to_string(),
            Cursor::Progress => "progress".to_string(),
            Cursor::Cell => "cell".to_string(),
            Cursor::Crosshair => "crosshair".to_string(),
            Cursor::VerticalText => "vertical-text".to_string(),
            Cursor::Alias => "alias".to_string(),
            Cursor::Copy => "copy".to_string(),
            Cursor::NoDrop => "no-drop".to_string(),
            Cursor::Grab => "grab".to_string(),
            Cursor::Grabbing => "grabbing".to_string(),
            Cursor::AllScroll => "all-scroll".to_string(),
            Cursor::ColResize => "col-resize".to_string(),
            Cursor::RowResize => "row-resize".to_string(),
            Cursor::NResize => "n-resize".to_string(),
            Cursor::EResize => "e-resize".to_string(),
            Cursor::SResize => "s-resize".to_string(),
            Cursor::WResize => "w-resize".to_string(),
            Cursor::NeResize => "ne-resize".to_string(),
            Cursor::NwResize => "nw-resize".to_string(),
            Cursor::SeResize => "se-resize".to_string(),
            Cursor::SwResize => "sw-resize".to_string(),
            Cursor::EwResize => "ew-resize".to_string(),
            Cursor::NsResize => "ns-resize".to_string(),
            Cursor::NeswResize => "nesw-resize".to_string(),
            Cursor::NwseResize => "nwse-resize".to_string(),
            Cursor::ZoomIn => "zoom-in".to_string(),
            Cursor::ZoomOut => "zoom-out".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Cursor::Auto => "auto".to_string(),
            Cursor::Default => "default".to_string(),
            Cursor::Pointer => "pointer".to_string(),
            Cursor::Wait => "wait".to_string(),
            Cursor::Text => "text".to_string(),
            Cursor::Move => "move".to_string(),
            Cursor::Help => "help".to_string(),
            Cursor::NotAllowed => "not-allowed".to_string(),
            Cursor::None => "none".to_string(),
            Cursor::ContextMenu => "context-menu".to_string(),
            Cursor::Progress => "progress".to_string(),
            Cursor::Cell => "cell".to_string(),
            Cursor::Crosshair => "crosshair".to_string(),
            Cursor::VerticalText => "vertical-text".to_string(),
            Cursor::Alias => "alias".to_string(),
            Cursor::Copy => "copy".to_string(),
            Cursor::NoDrop => "no-drop".to_string(),
            Cursor::Grab => "grab".to_string(),
            Cursor::Grabbing => "grabbing".to_string(),
            Cursor::AllScroll => "all-scroll".to_string(),
            Cursor::ColResize => "col-resize".to_string(),
            Cursor::RowResize => "row-resize".to_string(),
            Cursor::NResize => "n-resize".to_string(),
            Cursor::EResize => "e-resize".to_string(),
            Cursor::SResize => "s-resize".to_string(),
            Cursor::WResize => "w-resize".to_string(),
            Cursor::NeResize => "ne-resize".to_string(),
            Cursor::NwResize => "nw-resize".to_string(),
            Cursor::SeResize => "se-resize".to_string(),
            Cursor::SwResize => "sw-resize".to_string(),
            Cursor::EwResize => "ew-resize".to_string(),
            Cursor::NsResize => "ns-resize".to_string(),
            Cursor::NeswResize => "nesw-resize".to_string(),
            Cursor::NwseResize => "nwse-resize".to_string(),
            Cursor::ZoomIn => "zoom-in".to_string(),
            Cursor::ZoomOut => "zoom-out".to_string(),
        }
    }
}

impl PointerEvents {
    pub fn to_class_name(&self) -> String {
        match self {
            PointerEvents::Auto => "auto".to_string(),
            PointerEvents::None => "none".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            PointerEvents::Auto => "auto".to_string(),
            PointerEvents::None => "none".to_string(),
        }
    }
}

impl Resize {
    pub fn to_class_name(&self) -> String {
        match self {
            Resize::None => "none".to_string(),
            Resize::Both => "both".to_string(),
            Resize::Horizontal => "horizontal".to_string(),
            Resize::Vertical => "vertical".to_string(),
            Resize::Block => "block".to_string(),
            Resize::Inline => "inline".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Resize::None => "none".to_string(),
            Resize::Both => "both".to_string(),
            Resize::Horizontal => "horizontal".to_string(),
            Resize::Vertical => "vertical".to_string(),
            Resize::Block => "block".to_string(),
            Resize::Inline => "inline".to_string(),
        }
    }
}

impl ScrollBehavior {
    pub fn to_class_name(&self) -> String {
        match self {
            ScrollBehavior::Auto => "auto".to_string(),
            ScrollBehavior::Smooth => "smooth".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            ScrollBehavior::Auto => "auto".to_string(),
            ScrollBehavior::Smooth => "smooth".to_string(),
        }
    }
}

impl ScrollSnapType {
    pub fn to_class_name(&self) -> String {
        match self {
            ScrollSnapType::None => "none".to_string(),
            ScrollSnapType::X => "x".to_string(),
            ScrollSnapType::Y => "y".to_string(),
            ScrollSnapType::Both => "both".to_string(),
            ScrollSnapType::Mandatory => "mandatory".to_string(),
            ScrollSnapType::Proximity => "proximity".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            ScrollSnapType::None => "none".to_string(),
            ScrollSnapType::X => "x".to_string(),
            ScrollSnapType::Y => "y".to_string(),
            ScrollSnapType::Both => "both".to_string(),
            ScrollSnapType::Mandatory => "mandatory".to_string(),
            ScrollSnapType::Proximity => "proximity".to_string(),
        }
    }
}

impl ScrollSnapAlign {
    pub fn to_class_name(&self) -> String {
        match self {
            ScrollSnapAlign::Start => "start".to_string(),
            ScrollSnapAlign::End => "end".to_string(),
            ScrollSnapAlign::Center => "center".to_string(),
            ScrollSnapAlign::None => "none".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            ScrollSnapAlign::Start => "start".to_string(),
            ScrollSnapAlign::End => "end".to_string(),
            ScrollSnapAlign::Center => "center".to_string(),
            ScrollSnapAlign::None => "none".to_string(),
        }
    }
}

impl TouchAction {
    pub fn to_class_name(&self) -> String {
        match self {
            TouchAction::Auto => "auto".to_string(),
            TouchAction::None => "none".to_string(),
            TouchAction::PanX => "pan-x".to_string(),
            TouchAction::PanY => "pan-y".to_string(),
            TouchAction::PanLeft => "pan-left".to_string(),
            TouchAction::PanRight => "pan-right".to_string(),
            TouchAction::PanUp => "pan-up".to_string(),
            TouchAction::PanDown => "pan-down".to_string(),
            TouchAction::Manipulation => "manipulation".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TouchAction::Auto => "auto".to_string(),
            TouchAction::None => "none".to_string(),
            TouchAction::PanX => "pan-x".to_string(),
            TouchAction::PanY => "pan-y".to_string(),
            TouchAction::PanLeft => "pan-left".to_string(),
            TouchAction::PanRight => "pan-right".to_string(),
            TouchAction::PanUp => "pan-up".to_string(),
            TouchAction::PanDown => "pan-down".to_string(),
            TouchAction::Manipulation => "manipulation".to_string(),
        }
    }
}

impl UserSelect {
    pub fn to_class_name(&self) -> String {
        match self {
            UserSelect::None => "none".to_string(),
            UserSelect::Text => "text".to_string(),
            UserSelect::All => "all".to_string(),
            UserSelect::Auto => "auto".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            UserSelect::None => "none".to_string(),
            UserSelect::Text => "text".to_string(),
            UserSelect::All => "all".to_string(),
            UserSelect::Auto => "auto".to_string(),
        }
    }
}

impl WillChange {
    pub fn to_class_name(&self) -> String {
        match self {
            WillChange::Auto => "auto".to_string(),
            WillChange::ScrollPosition => "scroll-position".to_string(),
            WillChange::Contents => "contents".to_string(),
            WillChange::Transform => "transform".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            WillChange::Auto => "auto".to_string(),
            WillChange::ScrollPosition => "scroll-position".to_string(),
            WillChange::Contents => "contents".to_string(),
            WillChange::Transform => "transform".to_string(),
        }
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for PointerEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Resize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for ScrollBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for ScrollSnapType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for ScrollSnapAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TouchAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for UserSelect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for WillChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding cursor utilities to a class builder
pub trait CursorUtilities {
    fn cursor(self, cursor: Cursor) -> Self;
}

impl CursorUtilities for ClassBuilder {
    fn cursor(self, cursor: Cursor) -> Self {
        self.class(format!("cursor-{}", cursor.to_class_name()))
    }
}

/// Trait for adding pointer events utilities to a class builder
pub trait PointerEventsUtilities {
    fn pointer_events(self, events: PointerEvents) -> Self;
}

impl PointerEventsUtilities for ClassBuilder {
    fn pointer_events(self, events: PointerEvents) -> Self {
        self.class(format!("pointer-events-{}", events.to_class_name()))
    }
}

/// Trait for adding resize utilities to a class builder
pub trait ResizeUtilities {
    fn resize(self, resize: Resize) -> Self;
}

impl ResizeUtilities for ClassBuilder {
    fn resize(self, resize: Resize) -> Self {
        self.class(format!("resize-{}", resize.to_class_name()))
    }
}

/// Trait for adding scroll behavior utilities to a class builder
pub trait ScrollBehaviorUtilities {
    fn scroll_behavior(self, behavior: ScrollBehavior) -> Self;
}

impl ScrollBehaviorUtilities for ClassBuilder {
    fn scroll_behavior(self, behavior: ScrollBehavior) -> Self {
        self.class(format!("scroll-{}", behavior.to_class_name()))
    }
}

/// Trait for adding scroll snap type utilities to a class builder
pub trait ScrollSnapTypeUtilities {
    fn scroll_snap_type(self, snap_type: ScrollSnapType) -> Self;
}

impl ScrollSnapTypeUtilities for ClassBuilder {
    fn scroll_snap_type(self, snap_type: ScrollSnapType) -> Self {
        self.class(format!("snap-{}", snap_type.to_class_name()))
    }
}

/// Trait for adding scroll snap align utilities to a class builder
pub trait ScrollSnapAlignUtilities {
    fn scroll_snap_align(self, align: ScrollSnapAlign) -> Self;
}

impl ScrollSnapAlignUtilities for ClassBuilder {
    fn scroll_snap_align(self, align: ScrollSnapAlign) -> Self {
        self.class(format!("snap-align-{}", align.to_class_name()))
    }
}

/// Trait for adding touch action utilities to a class builder
pub trait TouchActionUtilities {
    fn touch_action(self, action: TouchAction) -> Self;
}

impl TouchActionUtilities for ClassBuilder {
    fn touch_action(self, action: TouchAction) -> Self {
        self.class(format!("touch-{}", action.to_class_name()))
    }
}

/// Trait for adding user select utilities to a class builder
pub trait UserSelectUtilities {
    fn user_select(self, select: UserSelect) -> Self;
}

impl UserSelectUtilities for ClassBuilder {
    fn user_select(self, select: UserSelect) -> Self {
        self.class(format!("select-{}", select.to_class_name()))
    }
}

/// Trait for adding will change utilities to a class builder
pub trait WillChangeUtilities {
    fn will_change(self, change: WillChange) -> Self;
}

impl WillChangeUtilities for ClassBuilder {
    fn will_change(self, change: WillChange) -> Self {
        self.class(format!("will-change-{}", change.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cursor_utilities() {
        let classes = ClassBuilder::new()
            .cursor(Cursor::Auto)
            .cursor(Cursor::Default)
            .cursor(Cursor::Pointer)
            .cursor(Cursor::Wait)
            .cursor(Cursor::Text)
            .cursor(Cursor::Move)
            .cursor(Cursor::Help)
            .cursor(Cursor::NotAllowed)
            .cursor(Cursor::None)
            .cursor(Cursor::Grab)
            .cursor(Cursor::Grabbing)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("cursor-auto"));
        assert!(css_classes.contains("cursor-default"));
        assert!(css_classes.contains("cursor-pointer"));
        assert!(css_classes.contains("cursor-wait"));
        assert!(css_classes.contains("cursor-text"));
        assert!(css_classes.contains("cursor-move"));
        assert!(css_classes.contains("cursor-help"));
        assert!(css_classes.contains("cursor-not-allowed"));
        assert!(css_classes.contains("cursor-none"));
        assert!(css_classes.contains("cursor-grab"));
        assert!(css_classes.contains("cursor-grabbing"));
    }
    
    #[test]
    fn test_pointer_events_utilities() {
        let classes = ClassBuilder::new()
            .pointer_events(PointerEvents::Auto)
            .pointer_events(PointerEvents::None)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("pointer-events-auto"));
        assert!(css_classes.contains("pointer-events-none"));
    }
    
    #[test]
    fn test_resize_utilities() {
        let classes = ClassBuilder::new()
            .resize(Resize::None)
            .resize(Resize::Both)
            .resize(Resize::Horizontal)
            .resize(Resize::Vertical)
            .resize(Resize::Block)
            .resize(Resize::Inline)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("resize-none"));
        assert!(css_classes.contains("resize-both"));
        assert!(css_classes.contains("resize-horizontal"));
        assert!(css_classes.contains("resize-vertical"));
        assert!(css_classes.contains("resize-block"));
        assert!(css_classes.contains("resize-inline"));
    }
    
    #[test]
    fn test_scroll_behavior_utilities() {
        let classes = ClassBuilder::new()
            .scroll_behavior(ScrollBehavior::Auto)
            .scroll_behavior(ScrollBehavior::Smooth)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("scroll-auto"));
        assert!(css_classes.contains("scroll-smooth"));
    }
    
    #[test]
    fn test_scroll_snap_type_utilities() {
        let classes = ClassBuilder::new()
            .scroll_snap_type(ScrollSnapType::None)
            .scroll_snap_type(ScrollSnapType::X)
            .scroll_snap_type(ScrollSnapType::Y)
            .scroll_snap_type(ScrollSnapType::Both)
            .scroll_snap_type(ScrollSnapType::Mandatory)
            .scroll_snap_type(ScrollSnapType::Proximity)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("snap-none"));
        assert!(css_classes.contains("snap-x"));
        assert!(css_classes.contains("snap-y"));
        assert!(css_classes.contains("snap-both"));
        assert!(css_classes.contains("snap-mandatory"));
        assert!(css_classes.contains("snap-proximity"));
    }
    
    #[test]
    fn test_scroll_snap_align_utilities() {
        let classes = ClassBuilder::new()
            .scroll_snap_align(ScrollSnapAlign::Start)
            .scroll_snap_align(ScrollSnapAlign::End)
            .scroll_snap_align(ScrollSnapAlign::Center)
            .scroll_snap_align(ScrollSnapAlign::None)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("snap-align-start"));
        assert!(css_classes.contains("snap-align-end"));
        assert!(css_classes.contains("snap-align-center"));
        assert!(css_classes.contains("snap-align-none"));
    }
    
    #[test]
    fn test_touch_action_utilities() {
        let classes = ClassBuilder::new()
            .touch_action(TouchAction::Auto)
            .touch_action(TouchAction::None)
            .touch_action(TouchAction::PanX)
            .touch_action(TouchAction::PanY)
            .touch_action(TouchAction::PanLeft)
            .touch_action(TouchAction::PanRight)
            .touch_action(TouchAction::PanUp)
            .touch_action(TouchAction::PanDown)
            .touch_action(TouchAction::Manipulation)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("touch-auto"));
        assert!(css_classes.contains("touch-none"));
        assert!(css_classes.contains("touch-pan-x"));
        assert!(css_classes.contains("touch-pan-y"));
        assert!(css_classes.contains("touch-pan-left"));
        assert!(css_classes.contains("touch-pan-right"));
        assert!(css_classes.contains("touch-pan-up"));
        assert!(css_classes.contains("touch-pan-down"));
        assert!(css_classes.contains("touch-manipulation"));
    }
    
    #[test]
    fn test_user_select_utilities() {
        let classes = ClassBuilder::new()
            .user_select(UserSelect::None)
            .user_select(UserSelect::Text)
            .user_select(UserSelect::All)
            .user_select(UserSelect::Auto)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("select-none"));
        assert!(css_classes.contains("select-text"));
        assert!(css_classes.contains("select-all"));
        assert!(css_classes.contains("select-auto"));
    }
    
    #[test]
    fn test_will_change_utilities() {
        let classes = ClassBuilder::new()
            .will_change(WillChange::Auto)
            .will_change(WillChange::ScrollPosition)
            .will_change(WillChange::Contents)
            .will_change(WillChange::Transform)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("will-change-auto"));
        assert!(css_classes.contains("will-change-scroll-position"));
        assert!(css_classes.contains("will-change-contents"));
        assert!(css_classes.contains("will-change-transform"));
    }
    
    #[test]
    fn test_complex_interactivity_combination() {
        let classes = ClassBuilder::new()
            .cursor(Cursor::Pointer)
            .pointer_events(PointerEvents::Auto)
            .resize(Resize::Both)
            .scroll_behavior(ScrollBehavior::Smooth)
            .scroll_snap_type(ScrollSnapType::X)
            .scroll_snap_align(ScrollSnapAlign::Center)
            .touch_action(TouchAction::PanX)
            .user_select(UserSelect::None)
            .will_change(WillChange::Transform)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("cursor-pointer"));
        assert!(css_classes.contains("pointer-events-auto"));
        assert!(css_classes.contains("resize-both"));
        assert!(css_classes.contains("scroll-smooth"));
        assert!(css_classes.contains("snap-x"));
        assert!(css_classes.contains("snap-align-center"));
        assert!(css_classes.contains("touch-pan-x"));
        assert!(css_classes.contains("select-none"));
        assert!(css_classes.contains("will-change-transform"));
    }
}
