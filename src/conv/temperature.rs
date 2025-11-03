use super::Degree;
use anyhow::{anyhow, Result};
use std::{
    f64,
    fmt::{Display, Result as FmtResult},
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

    pub fn convert_to(&self, target_unit: Degree) -> Result<f64> {
        let result = match (&self.unit, &target_unit) {
            // Same unit, no conversion needed
            (Degree::Celsius, Degree::Celsius) => self.value,
            (Degree::Fahrenheit, Degree::Fahrenheit) => self.value,
            (Degree::Kelvin, Degree::Kelvin) => self.value,

            // Celsius conversion
            (Degree::Celsius, Degree::Fahrenheit) => {
                let result = (self.value * 9.0 / 5.0) + 32.0;
                self.check_conversion_result(result, "Celsius to Fahrenheit")?
            }
            (Degree::Celsius, Degree::Kelvin) => {
                let result = self.value + 273.15;
                self.check_conversion_result(result, "Celsius to Kelvin")?
            }

            // Fahrenheit conversion
            (Degree::Fahrenheit, Degree::Celsius) => {
                let result = (self.value - 32.0) * 5.0 / 9.0;
                self.check_conversion_result(result, "Fahrenheit to Celsius")?
            }
            (Degree::Fahrenheit, Degree::Kelvin) => {
                let result = (self.value + 459.67) * 5.0 / 9.0;
                self.check_conversion_result(result, "Fahrenheit to Kelvin")?
            }

            // Kelvin conversion
            (Degree::Kelvin, Degree::Celsius) => {
                let result = self.value - 273.15;
                self.check_conversion_result(result, "Kelvin to Celsius")?
            }
            (Degree::Kelvin, Degree::Fahrenheit) => {
                let result = (self.value * 9.0 / 5.0) - 459.67;
                self.check_conversion_result(result, "Kelvin to Fahrenheit")?
            }
        };

        Ok(result)
    }

    fn check_conversion_result(&self, result: f64, conversion_type: &str) -> Result<f64> {
        if result.is_nan() {
            return Err(anyhow!(
                "{} conversion resulted in NaN. Input value: {} {:?}",
                conversion_type,
                self.value,
                self.unit
            ));
        }

        if result.is_infinite() {
            return Err(anyhow!(
                "{} conversion resulted in infinity. Input value: {} {:?}",
                conversion_type,
                self.value,
                self.unit
            ));
        }

        // Check for reasonable temperature ranges in the result
        if result.abs() > 1e12 {
            return Err(anyhow!(
                "{} conversion resulted in an unrealistic temperature: {:.2}. Please check your input.",
                conversion_type, result
            ));
        }

        Ok(result)
    }

    pub fn convert_to_all(&self) -> Result<TemperatureResults> {
        Ok(TemperatureResults {
            celsius: self.convert_to(Degree::Celsius)?,
            fahrenheit: self.convert_to(Degree::Fahrenheit)?,
            kelvin: self.convert_to(Degree::Kelvin)?,
        })
    }
}

impl Display for TemperatureResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Celsius: {:.2}°C, Fahrenheit: {:.2}°F, Kelvin: {:.2}K",
            self.celsius, self.fahrenheit, self.kelvin
        )
    }
}

// Helper functions with error handling
pub fn celsius_to_fahrenheit(celsius: f64) -> Result<f64> {
    TemperatureConverter::new(celsius, Degree::Celsius).convert_to(Degree::Fahrenheit)
}

pub fn celsius_to_kelvin(celsius: f64) -> Result<f64> {
    TemperatureConverter::new(celsius, Degree::Celsius).convert_to(Degree::Kelvin)
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> Result<f64> {
    TemperatureConverter::new(fahrenheit, Degree::Fahrenheit).convert_to(Degree::Celsius)
}

pub fn fahrenheit_to_kelvin(fahrenheit: f64) -> Result<f64> {
    TemperatureConverter::new(fahrenheit, Degree::Fahrenheit).convert_to(Degree::Kelvin)
}

pub fn kelvin_to_celsius(kelvin: f64) -> Result<f64> {
    TemperatureConverter::new(kelvin, Degree::Kelvin).convert_to(Degree::Celsius)
}

pub fn kelvin_to_fahrenheit(kelvin: f64) -> Result<f64> {
    TemperatureConverter::new(kelvin, Degree::Kelvin).convert_to(Degree::Fahrenheit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_conversions() -> Result<()> {
        let converter = TemperatureConverter::new(0.0, Degree::Celsius);

        assert_eq!(converter.convert_to(Degree::Fahrenheit)?, 32.0);
        assert_eq!(converter.convert_to(Degree::Kelvin)?, 273.15);
        Ok(())
    }

    #[test]
    fn test_fahrenheit_conversions() -> Result<()> {
        let converter = TemperatureConverter::new(32.0, Degree::Fahrenheit);

        assert_eq!(converter.convert_to(Degree::Celsius)?, 0.0);
        assert_eq!(converter.convert_to(Degree::Kelvin)?, 273.15);
        Ok(())
    }

    #[test]
    fn test_kelvin_conversions() -> Result<()> {
        let converter = TemperatureConverter::new(273.15, Degree::Kelvin);

        assert_eq!(converter.convert_to(Degree::Celsius)?, 0.0);
        // Allow for floating point precision
        let fahrenheit = converter.convert_to(Degree::Fahrenheit)?;
        assert!((fahrenheit - 32.0).abs() < 1e-10);
        Ok(())
    }

    #[test]
    fn test_helper_functions() -> Result<()> {
        assert_eq!(celsius_to_fahrenheit(100.0)?, 212.0);
        assert_eq!(fahrenheit_to_celsius(212.0)?, 100.0);
        assert_eq!(celsius_to_kelvin(0.0)?, 273.15);
        Ok(())
    }

    #[test]
    fn test_invalid_conversions() {
        // Test NaN handling
        let converter = TemperatureConverter::new(f64::NAN, Degree::Celsius);
        assert!(converter.convert_to(Degree::Fahrenheit).is_err());

        // Test infinity handling
        let converter = TemperatureConverter::new(f64::INFINITY, Degree::Celsius);
        assert!(converter.convert_to(Degree::Fahrenheit).is_err());
    }
}
