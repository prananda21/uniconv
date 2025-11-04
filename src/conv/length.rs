use super::Length;
use anyhow::{anyhow, Result};
use std::fmt::{Display, Result as FmtResult};

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

    pub fn convert_to(&self, target_unit: Length) -> Result<f64> {
        let result = match (&self.unit, &target_unit) {
            // Same unit, no conversion needed
            (Length::Centimeter, Length::Centimeter) => self.value,
            (Length::Inch, Length::Inch) => self.value,
            (Length::Kilometer, Length::Kilometer) => self.value,
            (Length::Miles, Length::Miles) => self.value,

            // Centimeter conversion
            (Length::Centimeter, Length::Inch) => {
                let result = self.value / 2.54;
                self.check_conversion_result(result, "Centimeter to Inch")?
            }
            (Length::Centimeter, Length::Kilometer) => {
                let result = self.value / 100000.0;
                self.check_conversion_result(result, "Centimeter to Kilometer")?
            }
            (Length::Centimeter, Length::Miles) => {
                let result = self.value / 160934.4;
                self.check_conversion_result(result, "Centimeter to Miles")?
            }

            // Inch conversion
            (Length::Inch, Length::Centimeter) => {
                let result = self.value * 2.54;
                self.check_conversion_result(result, "Inch to Centimeter")?
            }
            (Length::Inch, Length::Kilometer) => {
                let result = self.value * 0.0000254;
                self.check_conversion_result(result, "Inch to Kilometer")?
            }
            (Length::Inch, Length::Miles) => {
                let result = self.value / 63360.0;
                self.check_conversion_result(result, "Inch to Miles")?
            }

            // Kilometer conversion
            (Length::Kilometer, Length::Centimeter) => {
                let result = self.value * 100000.0;
                self.check_conversion_result(result, "Kilometer to Centimeter")?
            }
            (Length::Kilometer, Length::Inch) => {
                let result = self.value * 39370.08;
                self.check_conversion_result(result, "Kilometer to Inch")?
            }
            (Length::Kilometer, Length::Miles) => {
                let result = self.value / 1.609344;
                self.check_conversion_result(result, "Kilometer to Miles")?
            }

            // Miles conversion
            (Length::Miles, Length::Centimeter) => {
                let result = self.value * 160934.4;
                self.check_conversion_result(result, "Miles to Centimeter")?
            }
            (Length::Miles, Length::Inch) => {
                let result = self.value * 63360.0;
                self.check_conversion_result(result, "Miles to Inch")?
            }
            (Length::Miles, Length::Kilometer) => {
                let result = self.value * 1.609344;
                self.check_conversion_result(result, "Miles to Kilometer")?
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

        // Check for reasonable length ranges
        if result < 0.0 {
            return Err(anyhow!(
                "{} conversion resulted in negative length: {:.6}. This should not happen with positive input.",
                conversion_type, result
            ));
        }

        if result > 1e15 {
            return Err(anyhow!(
                "{} conversion resulted in an unrealistically large length: {:.2}. Please check your input.",
                conversion_type, result
            ));
        }

        Ok(result)
    }

    pub fn convert_to_all(&self) -> Result<LengthResults> {
        Ok(LengthResults {
            centimeter: self.convert_to(Length::Centimeter)?,
            inch: self.convert_to(Length::Inch)?,
            kilometer: self.convert_to(Length::Kilometer)?,
            miles: self.convert_to(Length::Miles)?,
        })
    }
}

impl Display for LengthResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Centimeter: {:.6} cm, Inch: {:.6} in, Kilometer: {:.6} km, Miles: {:.6} mi",
            self.centimeter, self.inch, self.kilometer, self.miles
        )
    }
}

// Helper functions with error handling
pub fn cm_to_km(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Centimeter).convert_to(Length::Kilometer)
}

pub fn cm_to_inch(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Centimeter).convert_to(Length::Inch)
}

pub fn cm_to_mile(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Centimeter).convert_to(Length::Miles)
}

pub fn inch_to_cm(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Inch).convert_to(Length::Centimeter)
}

pub fn inch_to_km(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Inch).convert_to(Length::Kilometer)
}

pub fn inch_to_mile(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Inch).convert_to(Length::Miles)
}

pub fn km_to_cm(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Kilometer).convert_to(Length::Centimeter)
}

pub fn km_to_inch(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Kilometer).convert_to(Length::Inch)
}

pub fn km_to_mile(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Kilometer).convert_to(Length::Miles)
}

pub fn mile_to_cm(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Miles).convert_to(Length::Centimeter)
}

pub fn mile_to_km(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Miles).convert_to(Length::Kilometer)
}

pub fn mile_to_inch(value: f64) -> Result<f64> {
    LengthConverter::new(value, Length::Miles).convert_to(Length::Inch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cm_conversions() -> Result<()> {
        let converter = LengthConverter::new(100.0, Length::Centimeter);

        let inch_result = converter.convert_to(Length::Inch)?;
        assert!((inch_result - 39.3701).abs() < 0.001);

        assert_eq!(converter.convert_to(Length::Kilometer)?, 0.001);

        let miles_result = converter.convert_to(Length::Miles)?;
        assert!((miles_result - 0.000621371).abs() < 0.000001);

        Ok(())
    }

    #[test]
    fn test_inch_conversions() -> Result<()> {
        let converter = LengthConverter::new(10.0, Length::Inch);

        assert_eq!(converter.convert_to(Length::Centimeter)?, 25.4);
        assert_eq!(converter.convert_to(Length::Kilometer)?, 0.000254);

        let miles_result = converter.convert_to(Length::Miles)?;
        assert!((miles_result - 0.000157828).abs() < 0.000001);

        Ok(())
    }

    #[test]
    fn test_km_conversions() -> Result<()> {
        let converter = LengthConverter::new(1.0, Length::Kilometer);

        assert_eq!(converter.convert_to(Length::Centimeter)?, 100000.0);

        let inch_result = converter.convert_to(Length::Inch)?;
        assert!((inch_result - 39370.08).abs() < 0.01);

        let miles_result = converter.convert_to(Length::Miles)?;
        assert!((miles_result - 0.621371).abs() < 0.001);

        Ok(())
    }

    #[test]
    fn test_mile_conversions() -> Result<()> {
        let converter = LengthConverter::new(1.0, Length::Miles);

        assert_eq!(converter.convert_to(Length::Centimeter)?, 160934.4);
        assert_eq!(converter.convert_to(Length::Inch)?, 63360.0);
        assert_eq!(converter.convert_to(Length::Kilometer)?, 1.609344);

        Ok(())
    }

    #[test]
    fn test_invalid_conversions() {
        // Test NaN handling
        let converter = LengthConverter::new(f64::NAN, Length::Centimeter);
        assert!(converter.convert_to(Length::Inch).is_err());

        // Test infinity handling
        let converter = LengthConverter::new(f64::INFINITY, Length::Centimeter);
        assert!(converter.convert_to(Length::Inch).is_err());
    }

    #[test]
    fn test_helper_functions() -> Result<()> {
        assert_eq!(cm_to_km(100000.0)?, 1.0);
        assert_eq!(inch_to_cm(1.0)?, 2.54);
        assert_eq!(km_to_mile(1.609344)?, 1.0);
        Ok(())
    }
}
