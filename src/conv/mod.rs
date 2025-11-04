mod length;
mod temperature;

use clap::ValueEnum;
pub use length::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
pub use temperature::*;

#[derive(Debug, Clone, ValueEnum)]
pub enum Length {
    Centimeter,
    Inch,
    Kilometer,
    Miles,
}
impl Display for Length {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Length::Centimeter => write!(f, "cm"),
            Length::Inch => write!(f, "in"),
            Length::Kilometer => write!(f, "km"),
            Length::Miles => write!(f, "mi"),
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Degree {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Display for Degree {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Degree::Celsius => write!(f, "°C"),
            Degree::Fahrenheit => write!(f, "°F"),
            Degree::Kelvin => write!(f, "K"),
        }
    }
}
