use std::fmt;

#[derive(Debug)]
pub enum ValidationError {
    InvalidNumber(String),
    PhysicallyInvalid(String),
    ValueTooLarge(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidNumber(msg) => write!(f, "Invalid number: {}", msg),
            ValidationError::PhysicallyInvalid(msg) => {
                write!(f, "Physically invalid value: {}", msg)
            }
            ValidationError::ValueTooLarge(msg) => write!(f, "Value too large: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug)]
pub enum ConversionError {
    Overflow(String),
    Underflow(String),
    PrecisionLoss(String),
    InvalidResult(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::Overflow(msg) => write!(f, "Conversion overflow: {}", msg),
            ConversionError::Underflow(msg) => write!(f, "Conversion underflow: {}", msg),
            ConversionError::PrecisionLoss(msg) => write!(f, "Precision loss: {}", msg),
            ConversionError::InvalidResult(msg) => write!(f, "Invalid conversion result: {}", msg),
        }
    }
}

impl std::error::Error for ConversionError {}

#[derive(Debug)]
pub enum UnitParseError {
    UnknownUnit(String),
    AmbiguousUnit(String),
    EmptyInput,
}

impl fmt::Display for UnitParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitParseError::UnknownUnit(unit) => write!(f, "Unknown unit: '{}'", unit),
            UnitParseError::AmbiguousUnit(unit) => write!(f, "Ambiguous unit: '{}'", unit),
            UnitParseError::EmptyInput => write!(f, "Empty unit input"),
        }
    }
}

impl std::error::Error for UnitParseError {}

// Helper function to format suggestions
pub fn format_suggestions(suggestions: &[&str]) -> String {
    match suggestions.len() {
        0 => String::new(),
        1 => format!("Did you mean '{}'?", suggestions[0]),
        _ => {
            let mut result = String::from("Did you mean one of: ");
            for (i, suggestion) in suggestions.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push('\'');
                result.push_str(suggestion);
                result.push('\'');
            }
            result.push('?');
            result
        }
    }
}
