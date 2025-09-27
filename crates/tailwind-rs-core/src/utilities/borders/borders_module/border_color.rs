//! Border color utilities
//!
//! This module provides utilities for border color values.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Border color values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderColor {
    /// Black border
    Black,
    /// White border
    White,
    /// Gray border
    Gray,
    /// Red border
    Red,
    /// Green border
    Green,
    /// Blue border
    Blue,
    /// Yellow border
    Yellow,
    /// Purple border
    Purple,
    /// Pink border
    Pink,
    /// Indigo border
    Indigo,
    /// Cyan border
    Cyan,
    /// Orange border
    Orange,
    /// Teal border
    Teal,
    /// Lime border
    Lime,
    /// Emerald border
    Emerald,
    /// Sky border
    Sky,
    /// Violet border
    Violet,
    /// Fuchia border
    Fuchia,
    /// Rose border
    Rose,
    /// Slate border
    Slate,
    /// Zinc border
    Zinc,
    /// Neutral border
    Neutral,
    /// Stone border
    Stone,
    /// Amber border
    Amber,
}

impl BorderColor {
    /// Get the CSS value for the border color
    pub fn css_value(&self) -> &'static str {
        match self {
            BorderColor::Black => "#000000",
            BorderColor::White => "#ffffff",
            BorderColor::Gray => "#6b7280",
            BorderColor::Red => "#ef4444",
            BorderColor::Green => "#22c55e",
            BorderColor::Blue => "#3b82f6",
            BorderColor::Yellow => "#eab308",
            BorderColor::Purple => "#a855f7",
            BorderColor::Pink => "#ec4899",
            BorderColor::Indigo => "#6366f1",
            BorderColor::Cyan => "#06b6d4",
            BorderColor::Orange => "#f97316",
            BorderColor::Teal => "#14b8a6",
            BorderColor::Lime => "#84cc16",
            BorderColor::Emerald => "#10b981",
            BorderColor::Sky => "#0ea5e9",
            BorderColor::Violet => "#8b5cf6",
            BorderColor::Fuchia => "#d946ef",
            BorderColor::Rose => "#f43f5e",
            BorderColor::Slate => "#64748b",
            BorderColor::Zinc => "#71717a",
            BorderColor::Neutral => "#737373",
            BorderColor::Stone => "#78716c",
            BorderColor::Amber => "#f59e0b",
        }
    }

    /// Get the Tailwind class for the border color
    pub fn class_name(&self) -> &'static str {
        match self {
            BorderColor::Black => "border-black",
            BorderColor::White => "border-white",
            BorderColor::Gray => "border-gray-500",
            BorderColor::Red => "border-red-500",
            BorderColor::Green => "border-green-500",
            BorderColor::Blue => "border-blue-500",
            BorderColor::Yellow => "border-yellow-500",
            BorderColor::Purple => "border-purple-500",
            BorderColor::Pink => "border-pink-500",
            BorderColor::Indigo => "border-indigo-500",
            BorderColor::Cyan => "border-cyan-500",
            BorderColor::Orange => "border-orange-500",
            BorderColor::Teal => "border-teal-500",
            BorderColor::Lime => "border-lime-500",
            BorderColor::Emerald => "border-emerald-500",
            BorderColor::Sky => "border-sky-500",
            BorderColor::Violet => "border-violet-500",
            BorderColor::Fuchia => "border-fuchsia-500",
            BorderColor::Rose => "border-rose-500",
            BorderColor::Slate => "border-slate-500",
            BorderColor::Zinc => "border-zinc-500",
            BorderColor::Neutral => "border-neutral-500",
            BorderColor::Stone => "border-stone-500",
            BorderColor::Amber => "border-amber-500",
        }
    }
}

impl fmt::Display for BorderColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_value())
    }
}

impl ClassBuilder {
    /// Set border color
    pub fn border_color(&mut self, color: BorderColor) -> &mut Self {
        *self = self.clone().class(color.class_name());
        self
    }

    /// Set border color to black
    pub fn border_black(&mut self) -> &mut Self {
        self.border_color(BorderColor::Black)
    }

    /// Set border color to white
    pub fn border_white(&mut self) -> &mut Self {
        self.border_color(BorderColor::White)
    }

    /// Set border color to gray
    pub fn border_gray(&mut self) -> &mut Self {
        self.border_color(BorderColor::Gray)
    }

    /// Set border color to red
    pub fn border_red(&mut self) -> &mut Self {
        self.border_color(BorderColor::Red)
    }

    /// Set border color to green
    pub fn border_green(&mut self) -> &mut Self {
        self.border_color(BorderColor::Green)
    }

    /// Set border color to blue
    pub fn border_blue(&mut self) -> &mut Self {
        self.border_color(BorderColor::Blue)
    }
}
