//! # State Definitions for Pseudo-classes
//!
//! This module provides state definitions for CSS pseudo-classes.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
    /// Indeterminate state (alternative)
    IndeterminateAlt,
    /// In-range state (alternative)
    InRangeAlt,
    /// Out-of-range state (alternative)
    OutOfRangeAlt,
    /// Read-only state (alternative)
    ReadOnlyAlt,
    /// Read-write state (alternative)
    ReadWriteAlt,
    /// Optional state (alternative)
    OptionalAlt,
    /// Placeholder-shown state (alternative)
    PlaceholderShownAlt,
    /// Autofill state (alternative)
    AutofillAlt,
}

impl State {
    /// Get the CSS pseudo-class prefix for this state
    pub fn prefix(&self) -> &'static str {
        match self {
            State::Hover => "hover:",
            State::Focus => "focus:",
            State::Active => "active:",
            State::Visited => "visited:",
            State::Disabled => "disabled:",
            State::Checked => "checked:",
            State::Indeterminate => "indeterminate:",
            State::Required => "required:",
            State::Valid => "valid:",
            State::Invalid => "invalid:",
            State::InRange => "in-range:",
            State::OutOfRange => "out-of-range:",
            State::ReadOnly => "read-only:",
            State::ReadWrite => "read-write:",
            State::Optional => "optional:",
            State::PlaceholderShown => "placeholder-shown:",
            State::Autofill => "autofill:",
            State::Default => "default:",
            State::IndeterminateAlt => "indeterminate:",
            State::InRangeAlt => "in-range:",
            State::OutOfRangeAlt => "out-of-range:",
            State::ReadOnlyAlt => "read-only:",
            State::ReadWriteAlt => "read-write:",
            State::OptionalAlt => "optional:",
            State::PlaceholderShownAlt => "placeholder-shown:",
            State::AutofillAlt => "autofill:",
        }
    }

    /// Get the CSS pseudo-class name for this state
    pub fn css_name(&self) -> &'static str {
        match self {
            State::Hover => ":hover",
            State::Focus => ":focus",
            State::Active => ":active",
            State::Visited => ":visited",
            State::Disabled => ":disabled",
            State::Checked => ":checked",
            State::Indeterminate => ":indeterminate",
            State::Required => ":required",
            State::Valid => ":valid",
            State::Invalid => ":invalid",
            State::InRange => ":in-range",
            State::OutOfRange => ":out-of-range",
            State::ReadOnly => ":read-only",
            State::ReadWrite => ":read-write",
            State::Optional => ":optional",
            State::PlaceholderShown => ":placeholder-shown",
            State::Autofill => ":autofill",
            State::Default => ":default",
            State::IndeterminateAlt => ":indeterminate",
            State::InRangeAlt => ":in-range",
            State::OutOfRangeAlt => ":out-of-range",
            State::ReadOnlyAlt => ":read-only",
            State::ReadWriteAlt => ":read-write",
            State::OptionalAlt => ":optional",
            State::PlaceholderShownAlt => ":placeholder-shown",
            State::AutofillAlt => ":autofill",
        }
    }

    /// Get all states
    pub fn all() -> Vec<State> {
        vec![
            State::Hover,
            State::Focus,
            State::Active,
            State::Visited,
            State::Disabled,
            State::Checked,
            State::Indeterminate,
            State::Required,
            State::Valid,
            State::Invalid,
            State::InRange,
            State::OutOfRange,
            State::ReadOnly,
            State::ReadWrite,
            State::Optional,
            State::PlaceholderShown,
            State::Autofill,
            State::Default,
        ]
    }

    /// Check if this state is interactive (user can trigger it)
    pub fn is_interactive(&self) -> bool {
        matches!(self, State::Hover | State::Focus | State::Active)
    }

    /// Check if this state is form-related
    pub fn is_form_related(&self) -> bool {
        matches!(
            self,
            State::Checked
                | State::Indeterminate
                | State::Required
                | State::Valid
                | State::Invalid
                | State::InRange
                | State::OutOfRange
                | State::ReadOnly
                | State::ReadWrite
                | State::Optional
                | State::PlaceholderShown
                | State::Autofill
                | State::Default
        )
    }
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
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
            _ => Err(format!("Invalid state: {}", s)),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Hover => write!(f, "hover"),
            State::Focus => write!(f, "focus"),
            State::Active => write!(f, "active"),
            State::Visited => write!(f, "visited"),
            State::Disabled => write!(f, "disabled"),
            State::Checked => write!(f, "checked"),
            State::Indeterminate => write!(f, "indeterminate"),
            State::Required => write!(f, "required"),
            State::Valid => write!(f, "valid"),
            State::Invalid => write!(f, "invalid"),
            State::InRange => write!(f, "in-range"),
            State::OutOfRange => write!(f, "out-of-range"),
            State::ReadOnly => write!(f, "read-only"),
            State::ReadWrite => write!(f, "read-write"),
            State::Optional => write!(f, "optional"),
            State::PlaceholderShown => write!(f, "placeholder-shown"),
            State::Autofill => write!(f, "autofill"),
            State::Default => write!(f, "default"),
            State::IndeterminateAlt => write!(f, "indeterminate"),
            State::InRangeAlt => write!(f, "in-range"),
            State::OutOfRangeAlt => write!(f, "out-of-range"),
            State::ReadOnlyAlt => write!(f, "read-only"),
            State::ReadWriteAlt => write!(f, "read-write"),
            State::OptionalAlt => write!(f, "optional"),
            State::PlaceholderShownAlt => write!(f, "placeholder-shown"),
            State::AutofillAlt => write!(f, "autofill"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_prefix() {
        assert_eq!(State::Hover.prefix(), "hover:");
        assert_eq!(State::Focus.prefix(), "focus:");
        assert_eq!(State::Active.prefix(), "active:");
        assert_eq!(State::Disabled.prefix(), "disabled:");
        assert_eq!(State::Checked.prefix(), "checked:");
    }

    #[test]
    fn test_state_css_name() {
        assert_eq!(State::Hover.css_name(), ":hover");
        assert_eq!(State::Focus.css_name(), ":focus");
        assert_eq!(State::Active.css_name(), ":active");
        assert_eq!(State::Disabled.css_name(), ":disabled");
        assert_eq!(State::Checked.css_name(), ":checked");
    }

    #[test]
    fn test_state_from_str() {
        assert_eq!(State::from_str("hover").unwrap(), State::Hover);
        assert_eq!(State::from_str("focus").unwrap(), State::Focus);
        assert_eq!(State::from_str("active").unwrap(), State::Active);
        assert_eq!(State::from_str("disabled").unwrap(), State::Disabled);
        assert_eq!(State::from_str("checked").unwrap(), State::Checked);
    }

    #[test]
    fn test_state_display() {
        assert_eq!(format!("{}", State::Hover), "hover");
        assert_eq!(format!("{}", State::Focus), "focus");
        assert_eq!(format!("{}", State::Active), "active");
        assert_eq!(format!("{}", State::Disabled), "disabled");
        assert_eq!(format!("{}", State::Checked), "checked");
    }

    #[test]
    fn test_state_all() {
        let all = State::all();
        assert_eq!(all.len(), 18);
        assert!(all.contains(&State::Hover));
        assert!(all.contains(&State::Focus));
        assert!(all.contains(&State::Active));
    }

    #[test]
    fn test_state_is_interactive() {
        assert!(State::Hover.is_interactive());
        assert!(State::Focus.is_interactive());
        assert!(State::Active.is_interactive());
        assert!(!State::Disabled.is_interactive());
        assert!(!State::Checked.is_interactive());
    }

    #[test]
    fn test_state_is_form_related() {
        assert!(!State::Hover.is_form_related());
        assert!(!State::Focus.is_form_related());
        assert!(!State::Active.is_form_related());
        assert!(State::Checked.is_form_related());
        assert!(State::Required.is_form_related());
        assert!(State::Valid.is_form_related());
    }
}
