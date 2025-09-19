//! Fraction utilities for tailwind-rs

use serde::{Deserialize, Serialize};
use std::fmt;

/// Fraction enum for fractional sizing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fraction {
    /// 1/2 fraction
    Half,
    /// 1/3 fraction
    Third,
    /// 2/3 fraction
    TwoThirds,
    /// 1/4 fraction
    Quarter,
    /// 2/4 fraction
    TwoQuarters,
    /// 3/4 fraction
    ThreeQuarters,
    /// 1/5 fraction
    Fifth,
    /// 2/5 fraction
    TwoFifths,
    /// 3/5 fraction
    ThreeFifths,
    /// 4/5 fraction
    FourFifths,
    /// 1/6 fraction
    Sixth,
    /// 2/6 fraction
    TwoSixths,
    /// 3/6 fraction
    ThreeSixths,
    /// 4/6 fraction
    FourSixths,
    /// 5/6 fraction
    FiveSixths,
    /// 1/12 fraction
    Twelfth,
    /// 2/12 fraction
    TwoTwelfths,
    /// 3/12 fraction
    ThreeTwelfths,
    /// 4/12 fraction
    FourTwelfths,
    /// 5/12 fraction
    FiveTwelfths,
    /// 6/12 fraction
    SixTwelfths,
    /// 7/12 fraction
    SevenTwelfths,
    /// 8/12 fraction
    EightTwelfths,
    /// 9/12 fraction
    NineTwelfths,
    /// 10/12 fraction
    TenTwelfths,
    /// 11/12 fraction
    ElevenTwelfths,
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fraction::Half => write!(f, "1/2"),
            Fraction::Third => write!(f, "1/3"),
            Fraction::TwoThirds => write!(f, "2/3"),
            Fraction::Quarter => write!(f, "1/4"),
            Fraction::TwoQuarters => write!(f, "2/4"),
            Fraction::ThreeQuarters => write!(f, "3/4"),
            Fraction::Fifth => write!(f, "1/5"),
            Fraction::TwoFifths => write!(f, "2/5"),
            Fraction::ThreeFifths => write!(f, "3/5"),
            Fraction::FourFifths => write!(f, "4/5"),
            Fraction::Sixth => write!(f, "1/6"),
            Fraction::TwoSixths => write!(f, "2/6"),
            Fraction::ThreeSixths => write!(f, "3/6"),
            Fraction::FourSixths => write!(f, "4/6"),
            Fraction::FiveSixths => write!(f, "5/6"),
            Fraction::Twelfth => write!(f, "1/12"),
            Fraction::TwoTwelfths => write!(f, "2/12"),
            Fraction::ThreeTwelfths => write!(f, "3/12"),
            Fraction::FourTwelfths => write!(f, "4/12"),
            Fraction::FiveTwelfths => write!(f, "5/12"),
            Fraction::SixTwelfths => write!(f, "6/12"),
            Fraction::SevenTwelfths => write!(f, "7/12"),
            Fraction::EightTwelfths => write!(f, "8/12"),
            Fraction::NineTwelfths => write!(f, "9/12"),
            Fraction::TenTwelfths => write!(f, "10/12"),
            Fraction::ElevenTwelfths => write!(f, "11/12"),
        }
    }
}

impl Fraction {
    pub fn to_class_name(&self) -> String {
        self.to_string()
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Fraction::Half => "50%".to_string(),
            Fraction::Third => "33.333333%".to_string(),
            Fraction::TwoThirds => "66.666667%".to_string(),
            Fraction::Quarter => "25%".to_string(),
            Fraction::TwoQuarters => "50%".to_string(),
            Fraction::ThreeQuarters => "75%".to_string(),
            Fraction::Fifth => "20%".to_string(),
            Fraction::TwoFifths => "40%".to_string(),
            Fraction::ThreeFifths => "60%".to_string(),
            Fraction::FourFifths => "80%".to_string(),
            Fraction::Sixth => "16.666667%".to_string(),
            Fraction::TwoSixths => "33.333333%".to_string(),
            Fraction::ThreeSixths => "50%".to_string(),
            Fraction::FourSixths => "66.666667%".to_string(),
            Fraction::FiveSixths => "83.333333%".to_string(),
            Fraction::Twelfth => "8.333333%".to_string(),
            Fraction::TwoTwelfths => "16.666667%".to_string(),
            Fraction::ThreeTwelfths => "25%".to_string(),
            Fraction::FourTwelfths => "33.333333%".to_string(),
            Fraction::FiveTwelfths => "41.666667%".to_string(),
            Fraction::SixTwelfths => "50%".to_string(),
            Fraction::SevenTwelfths => "58.333333%".to_string(),
            Fraction::EightTwelfths => "66.666667%".to_string(),
            Fraction::NineTwelfths => "75%".to_string(),
            Fraction::TenTwelfths => "83.333333%".to_string(),
            Fraction::ElevenTwelfths => "91.666667%".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_display() {
        assert_eq!(Fraction::Half.to_string(), "1/2");
        assert_eq!(Fraction::Third.to_string(), "1/3");
        assert_eq!(Fraction::TwoThirds.to_string(), "2/3");
    }

    #[test]
    fn test_fraction_css_value() {
        assert_eq!(Fraction::Half.to_css_value(), "50%");
        assert_eq!(Fraction::Third.to_css_value(), "33.333333%");
        assert_eq!(Fraction::TwoThirds.to_css_value(), "66.666667%");
    }
}
