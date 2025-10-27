mod length;
mod temperature;

use clap::ValueEnum;
pub use length::*;
pub use temperature::*;

#[derive(Debug, Clone, ValueEnum)]
pub enum Length {
    Centimeter,
    Inch,
    Kilometer,
    Miles,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Degree {
    Celsius,
    Fahrenheit,
    Kelvin,
}
