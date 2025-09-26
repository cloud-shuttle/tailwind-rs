//! Grid fraction utilities for tailwind-rs

use serde::{Deserialize, Serialize};
use std::fmt;

/// Grid fraction enum for grid fractional sizing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridFraction {
    /// 1/2 grid fraction
    Half,
    /// 1/3 grid fraction
    Third,
    /// 2/3 grid fraction
    TwoThirds,
    /// 1/4 grid fraction
    Quarter,
    /// 2/4 grid fraction
    TwoQuarters,
    /// 3/4 grid fraction
    ThreeQuarters,
    /// 1/5 grid fraction
    Fifth,
    /// 2/5 grid fraction
    TwoFifths,
    /// 3/5 grid fraction
    ThreeFifths,
    /// 4/5 grid fraction
    FourFifths,
    /// 1/6 grid fraction
    Sixth,
    /// 2/6 grid fraction
    TwoSixths,
    /// 3/6 grid fraction
    ThreeSixths,
    /// 4/6 grid fraction
    FourSixths,
    /// 5/6 grid fraction
    FiveSixths,
    /// 1/12 grid fraction
    Twelfth,
    /// 2/12 grid fraction
    TwoTwelfths,
    /// 3/12 grid fraction
    ThreeTwelfths,
    /// 4/12 grid fraction
    FourTwelfths,
    /// 5/12 grid fraction
    FiveTwelfths,
    /// 6/12 grid fraction
    SixTwelfths,
    /// 7/12 grid fraction
    SevenTwelfths,
    /// 8/12 grid fraction
    EightTwelfths,
    /// 9/12 grid fraction
    NineTwelfths,
    /// 10/12 grid fraction
    TenTwelfths,
    /// 11/12 grid fraction
    ElevenTwelfths,
}

impl fmt::Display for GridFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridFraction::Half => write!(f, "1/2"),
            GridFraction::Third => write!(f, "1/3"),
            GridFraction::TwoThirds => write!(f, "2/3"),
            GridFraction::Quarter => write!(f, "1/4"),
            GridFraction::TwoQuarters => write!(f, "2/4"),
            GridFraction::ThreeQuarters => write!(f, "3/4"),
            GridFraction::Fifth => write!(f, "1/5"),
            GridFraction::TwoFifths => write!(f, "2/5"),
            GridFraction::ThreeFifths => write!(f, "3/5"),
            GridFraction::FourFifths => write!(f, "4/5"),
            GridFraction::Sixth => write!(f, "1/6"),
            GridFraction::TwoSixths => write!(f, "2/6"),
            GridFraction::ThreeSixths => write!(f, "3/6"),
            GridFraction::FourSixths => write!(f, "4/6"),
            GridFraction::FiveSixths => write!(f, "5/6"),
            GridFraction::Twelfth => write!(f, "1/12"),
            GridFraction::TwoTwelfths => write!(f, "2/12"),
            GridFraction::ThreeTwelfths => write!(f, "3/12"),
            GridFraction::FourTwelfths => write!(f, "4/12"),
            GridFraction::FiveTwelfths => write!(f, "5/12"),
            GridFraction::SixTwelfths => write!(f, "6/12"),
            GridFraction::SevenTwelfths => write!(f, "7/12"),
            GridFraction::EightTwelfths => write!(f, "8/12"),
            GridFraction::NineTwelfths => write!(f, "9/12"),
            GridFraction::TenTwelfths => write!(f, "10/12"),
            GridFraction::ElevenTwelfths => write!(f, "11/12"),
        }
    }
}

impl GridFraction {
    pub fn to_class_name(&self) -> String {
        self.to_string()
    }

    pub fn to_css_value(&self) -> String {
        match self {
            GridFraction::Half => "50%".to_string(),
            GridFraction::Third => "33.333333%".to_string(),
            GridFraction::TwoThirds => "66.666667%".to_string(),
            GridFraction::Quarter => "25%".to_string(),
            GridFraction::TwoQuarters => "50%".to_string(),
            GridFraction::ThreeQuarters => "75%".to_string(),
            GridFraction::Fifth => "20%".to_string(),
            GridFraction::TwoFifths => "40%".to_string(),
            GridFraction::ThreeFifths => "60%".to_string(),
            GridFraction::FourFifths => "80%".to_string(),
            GridFraction::Sixth => "16.666667%".to_string(),
            GridFraction::TwoSixths => "33.333333%".to_string(),
            GridFraction::ThreeSixths => "50%".to_string(),
            GridFraction::FourSixths => "66.666667%".to_string(),
            GridFraction::FiveSixths => "83.333333%".to_string(),
            GridFraction::Twelfth => "8.333333%".to_string(),
            GridFraction::TwoTwelfths => "16.666667%".to_string(),
            GridFraction::ThreeTwelfths => "25%".to_string(),
            GridFraction::FourTwelfths => "33.333333%".to_string(),
            GridFraction::FiveTwelfths => "41.666667%".to_string(),
            GridFraction::SixTwelfths => "50%".to_string(),
            GridFraction::SevenTwelfths => "58.333333%".to_string(),
            GridFraction::EightTwelfths => "66.666667%".to_string(),
            GridFraction::NineTwelfths => "75%".to_string(),
            GridFraction::TenTwelfths => "83.333333%".to_string(),
            GridFraction::ElevenTwelfths => "91.666667%".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_fraction_display() {
        assert_eq!(GridFraction::Half.to_string(), "1/2");
        assert_eq!(GridFraction::Third.to_string(), "1/3");
        assert_eq!(GridFraction::TwoThirds.to_string(), "2/3");
    }

    #[test]
    fn test_grid_fraction_css_value() {
        assert_eq!(GridFraction::Half.to_css_value(), "50%");
        assert_eq!(GridFraction::Third.to_css_value(), "33.333333%");
        assert_eq!(GridFraction::TwoThirds.to_css_value(), "66.666667%");
    }
}
