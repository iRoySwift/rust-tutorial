//! # Arts
//!
//! A library for modeling artistic concepts

pub use self::kind::PrimaryColor;
pub use self::kind::SecondaryColor;
pub use self::utils::mix;

pub mod kind {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kind::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
