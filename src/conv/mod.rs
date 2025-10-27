mod length;
mod temperature;

pub use length::*;
pub use temperature::*;

pub enum Length {
    Centimeter,
    Inch,
    Kilometer,
    Miles,
}

pub enum Degree {
    Celsius,
    Fahrenheit,
    Kelvin,
}
