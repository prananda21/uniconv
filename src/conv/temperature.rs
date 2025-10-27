use super::Degree;
use std::{
    f64,
    fmt::{Display, Result},
};

pub struct TemperatureConverter {
    pub value: f64,
    pub unit: Degree,
}

pub struct TemperatureResults {
    pub celsius: f64,
    pub fahrenheit: f64,
    pub kelvin: f64,
}

impl TemperatureConverter {
    pub fn new(value: f64, unit: Degree) -> Self {
        Self { value, unit }
    }

    pub fn convert_to(&self, target_unit: Degree) -> f64 {
        match (&self.unit, &target_unit) {
            // the same unit, no convertion needed
            (Degree::Celsius, Degree::Celsius) => self.value,
            (Degree::Fahrenheit, Degree::Fahrenheit) => self.value,
            (Degree::Kelvin, Degree::Kelvin) => self.value,

            // Celcius convertion
            (Degree::Celsius, Degree::Fahrenheit) => (self.value * 9.0 / 5.0) + 32.0,
            (Degree::Celsius, Degree::Kelvin) => self.value + 273.15,

            // Fahrenheit convertion
            (Degree::Fahrenheit, Degree::Celsius) => (self.value - 32.0) * 5.0 / 9.0,
            (Degree::Fahrenheit, Degree::Kelvin) => (self.value + 459.67) * 5.0 / 9.0,

            // Kelvin convertion
            (Degree::Kelvin, Degree::Celsius) => self.value - 273.15,
            (Degree::Kelvin, Degree::Fahrenheit) => (self.value * 9.0 / 5.0) - 459.67,
        }
    }

    pub fn convert_to_all(&self) -> TemperatureResults {
        TemperatureResults {
            celsius: self.convert_to(Degree::Celsius),
            fahrenheit: self.convert_to(Degree::Fahrenheit),
            kelvin: self.convert_to(Degree::Kelvin),
        }
    }
}

impl Display for TemperatureResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Celsius: {:.2}°C, Fahrenheit: {:.2}°F, Kelvin: {:.2}K",
            self.celsius, self.fahrenheit, self.kelvin
        )
    }
}

pub fn celcius_to_fahrenheit(celsius: f64) -> f64 {
    TemperatureConverter::new(celsius, Degree::Celsius).convert_to(Degree::Fahrenheit)
}

pub fn celcius_to_kelvin(celcius: f64) -> f64 {
    TemperatureConverter::new(celcius, Degree::Celsius).convert_to(Degree::Kelvin)
}

pub fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    TemperatureConverter::new(fahrenheit, Degree::Fahrenheit).convert_to(Degree::Celsius)
}

pub fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    TemperatureConverter::new(fahrenheit, Degree::Fahrenheit).convert_to(Degree::Kelvin)
}

pub fn kelvin_to_celcius(kelvin: f64) -> f64 {
    TemperatureConverter::new(kelvin, Degree::Kelvin).convert_to(Degree::Celsius)
}

pub fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    TemperatureConverter::new(kelvin, Degree::Kelvin).convert_to(Degree::Fahrenheit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celcius_convertions() {
        let converter = TemperatureConverter::new(0.0, Degree::Celsius);

        assert_eq!(converter.convert_to(Degree::Fahrenheit), 32.0);
        assert_eq!(converter.convert_to(Degree::Kelvin), 273.15);
    }

    #[test]
    fn test_fahrenheit_conversions() {
        let converter = TemperatureConverter::new(32.0, Degree::Fahrenheit);

        assert_eq!(converter.convert_to(Degree::Celsius), 0.0);
        assert_eq!(converter.convert_to(Degree::Kelvin), 273.15)
    }

    #[test]
    fn test_kelvin_conversions() {
        let converter = TemperatureConverter::new(273.15, Degree::Kelvin);

        assert_eq!(converter.convert_to(Degree::Celsius), 0.0);
        assert_eq!(converter.convert_to(Degree::Fahrenheit), 31.999999999999943);
    }

    #[test]
    fn test_functions() {
        assert_eq!(celcius_to_fahrenheit(100.0), 212.0);
        assert_eq!(fahrenheit_to_celcius(212.0), 100.0);
        assert_eq!(celcius_to_kelvin(0.0), 273.15);
    }
}
