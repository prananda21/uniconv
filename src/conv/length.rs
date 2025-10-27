use std::fmt::{Display, Result};

use super::Length;

pub struct LengthConverter {
    pub value: f64,
    pub unit: Length,
}

pub struct LengthResults {
    pub centimeter: f64,
    pub inch: f64,
    pub kilometer: f64,
    pub miles: f64,
}

impl LengthConverter {
    pub fn new(value: f64, unit: Length) -> Self {
        Self { value, unit }
    }

    pub fn convert_to(&self, target_unit: Length) -> f64 {
        match (&self.unit, &target_unit) {
            // Case same unit, no convertion needed
            (Length::Centimeter, Length::Centimeter) => self.value,
            (Length::Inch, Length::Inch) => self.value,
            (Length::Kilometer, Length::Kilometer) => self.value,
            (Length::Miles, Length::Miles) => self.value,

            // Centimeter convertion
            (Length::Centimeter, Length::Inch) => self.value * 0.393701,
            (Length::Centimeter, Length::Kilometer) => self.value * 0.00001,
            (Length::Centimeter, Length::Miles) => self.value * 0.0000062137,

            // Inch convertion
            (Length::Inch, Length::Centimeter) => self.value * 2.54,
            (Length::Inch, Length::Kilometer) => self.value * 0.0000254,
            (Length::Inch, Length::Miles) => self.value * 0.0000157828,

            // Kilometer convertion
            (Length::Kilometer, Length::Centimeter) => self.value * 100000.0,
            (Length::Kilometer, Length::Inch) => self.value * 39370.08,
            (Length::Kilometer, Length::Miles) => self.value * 0.621371,

            // Miles convertion
            (Length::Miles, Length::Centimeter) => self.value * 160934.4,
            (Length::Miles, Length::Inch) => self.value * 63360.0,
            (Length::Miles, Length::Kilometer) => self.value * 1.609344,
        }
    }

    pub fn convert_to_all(&self) -> LengthResults {
        LengthResults {
            centimeter: self.convert_to(Length::Centimeter),
            inch: self.convert_to(Length::Inch),
            kilometer: self.convert_to(Length::Kilometer),
            miles: self.convert_to(Length::Miles),
        }
    }
}

impl Display for LengthResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Centimeter: {:?} cm, Inch: {:?} inch, Kilometer: {:?} km, Miles: {:?} miles",
            self.centimeter, self.inch, self.kilometer, self.miles
        )
    }
}

pub fn cm_to_km(value: f64) -> f64 {
    LengthConverter::new(value, Length::Centimeter).convert_to(Length::Kilometer)
}
pub fn cm_to_inch(value: f64) -> f64 {
    LengthConverter::new(value, Length::Centimeter).convert_to(Length::Inch)
}
pub fn cm_to_mile(value: f64) -> f64 {
    LengthConverter::new(value, Length::Centimeter).convert_to(Length::Miles)
}

pub fn inch_to_cm(value: f64) -> f64 {
    LengthConverter::new(value, Length::Inch).convert_to(Length::Centimeter)
}
pub fn inch_to_km(value: f64) -> f64 {
    LengthConverter::new(value, Length::Inch).convert_to(Length::Kilometer)
}
pub fn inch_to_mile(value: f64) -> f64 {
    LengthConverter::new(value, Length::Inch).convert_to(Length::Miles)
}

pub fn km_to_cm(value: f64) -> f64 {
    LengthConverter::new(value, Length::Kilometer).convert_to(Length::Centimeter)
}
pub fn km_to_inch(value: f64) -> f64 {
    LengthConverter::new(value, Length::Kilometer).convert_to(Length::Inch)
}
pub fn km_to_mile(value: f64) -> f64 {
    LengthConverter::new(value, Length::Kilometer).convert_to(Length::Miles)
}

pub fn mile_to_cm(value: f64) -> f64 {
    LengthConverter::new(value, Length::Miles).convert_to(Length::Centimeter)
}
pub fn mile_to_km(value: f64) -> f64 {
    LengthConverter::new(value, Length::Miles).convert_to(Length::Kilometer)
}
pub fn mile_to_inch(value: f64) -> f64 {
    LengthConverter::new(value, Length::Miles).convert_to(Length::Inch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cm_convertions() {
        let converter = LengthConverter::new(10.0, Length::Centimeter);

        assert_eq!(converter.convert_to(Length::Inch), 3.9370100000000003);
        assert_eq!(converter.convert_to(Length::Kilometer), 0.0001);
        assert_eq!(converter.convert_to(Length::Miles), 0.000062137)
    }

    #[test]
    fn test_inch_convertions() {
        let converter = LengthConverter::new(10.0, Length::Inch);

        assert_eq!(converter.convert_to(Length::Centimeter), 25.4);
        assert_eq!(converter.convert_to(Length::Kilometer), 0.000254);
        assert_eq!(converter.convert_to(Length::Miles), 0.000157828)
    }

    #[test]
    fn test_km_convertions() {
        let converter = LengthConverter::new(10.0, Length::Kilometer);

        assert_eq!(converter.convert_to(Length::Centimeter), 1000000.0);
        assert_eq!(converter.convert_to(Length::Inch), 393700.80000000005);
        assert_eq!(converter.convert_to(Length::Miles), 6.21371)
    }

    #[test]
    fn test_mile_convertions() {
        let converter = LengthConverter::new(10.0, Length::Miles);

        assert_eq!(converter.convert_to(Length::Centimeter), 1609344.0);
        assert_eq!(converter.convert_to(Length::Inch), 633600.0);
        assert_eq!(converter.convert_to(Length::Kilometer), 16.09344)
    }
}
