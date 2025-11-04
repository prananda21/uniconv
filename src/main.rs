use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};

mod conv;
mod errors;

use conv::{Degree, Length, LengthConverter, TemperatureConverter};

#[derive(Parser)]
#[command(name = "uniconv")]
#[command(about = "A universal unit converter (based on temperature and length")]
#[command(version = "1.0")]
#[command(long_about = r#"
Examples:
  Smart conversion (automatically detects unit type):
    uniconv convert --from celsius --to fahrenheit --value 25
    uniconv convert --from c --to f --value 25
    uniconv convert --from cm --to inch --value 188
    uniconv convert --from centimeter --to miles --value 1000

  Dedicated commands:
    uniconv temperature --from celsius --to fahrenheit --value 25
    uniconv length --from cm --to inch --value 188
"#)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Convert between temperature units")]
    Temperature {
        #[arg(long, help = "Source temperature unit")]
        from: Degree,
        #[arg(long, help = "Target temperature unit")]
        to: Degree,
        #[arg(long, help = "Temperature value to convert")]
        value: f64,
    },
    #[command(about = "Convert between length units")]
    Length {
        #[arg(long, help = "Source length unit")]
        from: Length,
        #[arg(long, help = "Target length unit")]
        to: Length,
        #[arg(long, help = "Length value to convert")]
        value: f64,
    },
    #[command(about = "Convert between units (automatically detects unit type)")]
    Convert {
        #[arg(long, help = "Source unit (e.g., 'celsius', 'cm', 'c', 'centimeter')")]
        from: String,
        #[arg(long, help = "Target unit (e.g., 'fahrenheit', 'inch', 'f', 'in')")]
        to: String,
        #[arg(long, help = "Value to convert")]
        value: f64,
    },
}

fn format_number(value: f64) -> String {
    println!("{}", value);
    format!("{}", value.round_ties_even() as i64)
}

fn validate_numeric_input(value: f64, context: &str) -> Result<()> {
    if value.is_nan() {
        return Err(anyhow!("{} cannot be NaN (Not a Number)", context));
    }

    if value.is_infinite() {
        return Err(anyhow!("{} cannot be infinite", context));
    }

    // Check for extremely large numbers that might cause precision issues
    if value.abs() > 1e15 {
        return Err(anyhow!(
            "{} is too large (absolute value exceeds 1e15). Please use a smaller number.",
            context
        ));
    }

    Ok(())
}

fn convert_temperature(value: f64, from: Degree, to: Degree) -> Result<f64> {
    // Validate input
    validate_numeric_input(value, "Temperature value")?;

    match from {
        Degree::Kelvin if value < 0.0 => {
            return Err(anyhow!(
                "Kelvin temperature cannot be negative ({}K). Minimum is 0 K (absolute zero).",
                value
            ));
        }
        Degree::Celsius if value < -273.15 => {
            return Err(anyhow!(
                "Celsius temperature cannot be below absolute zero ({}°C < -273.15°C).",
                value
            ));
        }
        Degree::Fahrenheit if value < -459.67 => {
            return Err(anyhow!(
                "Fahrenheit temperature cannot be below absolute zero ({}°F < -459.67°F).",
                value
            ));
        }
        _ => {}
    }

    let converter = TemperatureConverter::new(value, from.into());
    let result = converter
        .convert_to(to.into())
        .context("Failed to perform temperature conversion")?;

    // Validate result
    validate_numeric_input(result, "Conversion result")?;

    Ok(result)
}

fn convert_length(value: f64, from: Length, to: Length) -> Result<f64> {
    // Validate input
    validate_numeric_input(value, "Length value")?;

    if value < 0.0 {
        return Err(anyhow!(
            "Length cannot be negative ({}). Please provide a positive value.",
            value
        ));
    }

    if value > 1e12 {
        return Err(anyhow!(
            "Length value {} is unrealistically large. Please check your input.",
            value
        ));
    }

    let converter = LengthConverter::new(value, from.into());
    let result = converter
        .convert_to(to.into())
        .context("Failed to perform length conversion")?;

    // Validate result
    validate_numeric_input(result, "Conversion result")?;

    Ok(result)
}

fn find_closest_match(input: &str, valid_units: &[&str]) -> Option<String> {
    let input_lower = input.to_lowercase();

    // First, try exact matches or partial matches
    for unit in valid_units {
        if unit.to_lowercase().contains(&input_lower) || input_lower.contains(&unit.to_lowercase())
        {
            return Some(unit.to_string());
        }
    }

    // If no partial match, find the unit with minimum edit distance
    let mut best_match = None;
    let mut min_distance = usize::MAX;

    for unit in valid_units {
        let distance = levenshtein_distance(&input_lower, &unit.to_lowercase());
        if distance < min_distance && distance <= 3 {
            // Only suggest if distance is reasonable
            min_distance = distance;
            best_match = Some(unit.to_string());
        }
    }

    best_match
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[len1][len2]
}

fn parse_temperature_unit(unit: &str) -> Result<Degree> {
    match unit.to_lowercase().as_str() {
        "celsius" | "c" => Ok(Degree::Celsius),
        "fahrenheit" | "f" => Ok(Degree::Fahrenheit),
        "kelvin" | "k" => Ok(Degree::Kelvin),
        _ => {
            let valid_units = &["celsius", "c", "fahrenheit", "f", "kelvin", "k"];
            let mut error_msg = format!("Invalid temperature unit: '{}'.\n", unit);
            error_msg.push_str("Valid temperature units are:\n");
            error_msg.push_str("  • celsius (or 'c')\n");
            error_msg.push_str("  • fahrenheit (or 'f')\n");
            error_msg.push_str("  • kelvin (or 'k')\n");

            if let Some(suggestion) = find_closest_match(unit, valid_units) {
                error_msg.push_str(&format!("\nDid you mean '{}'?", suggestion));
            }

            Err(anyhow!(error_msg))
        }
    }
}

fn parse_length_unit(unit: &str) -> Result<Length> {
    match unit.to_lowercase().as_str() {
        "centimeter" | "cm" => Ok(Length::Centimeter),
        "inch" | "in" => Ok(Length::Inch),
        "kilometer" | "km" => Ok(Length::Kilometer),
        "miles" | "mi" => Ok(Length::Miles),
        _ => {
            let valid_units = &[
                "centimeter",
                "cm",
                "inch",
                "in",
                "kilometer",
                "km",
                "miles",
                "mi",
            ];
            let mut error_msg = format!("Invalid length unit: '{}'.\n", unit);
            error_msg.push_str("Valid length units are:\n");
            error_msg.push_str("  • centimeter (or 'cm')\n");
            error_msg.push_str("  • inch (or 'in')\n");
            error_msg.push_str("  • kilometer (or 'km')\n");
            error_msg.push_str("  • miles (or 'mi')\n");

            if let Some(suggestion) = find_closest_match(unit, valid_units) {
                error_msg.push_str(&format!("\nDid you mean '{}'?", suggestion));
            }

            Err(anyhow!(error_msg))
        }
    }
}

fn detect_and_convert(from: &str, to: &str, value: f64) -> Result<()> {
    // First, try to parse both units as temperature units
    let temp_from = parse_temperature_unit(from);
    let temp_to = parse_temperature_unit(to);

    if let (Ok(from_unit), Ok(to_unit)) = (temp_from, temp_to) {
        // Both units are valid temperature units
        let conversion_result = convert_temperature(value, from_unit.clone(), to_unit.clone())
            .with_context(|| {
                format!(
                    "Failed to convert {} {} to {}",
                    format_number(value),
                    from_unit,
                    to_unit
                )
            })?;

        println!(
            "{} {} = {} {}",
            format_number(value),
            from_unit,
            format_number(conversion_result),
            to_unit
        );
        return Ok(());
    }

    // If temperature parsing failed, try length units
    let length_from = parse_length_unit(from);
    let length_to = parse_length_unit(to);

    if let (Ok(from_unit), Ok(to_unit)) = (length_from, length_to) {
        // Both units are valid length units
        let conversion_result = convert_length(value, from_unit.clone(), to_unit.clone())
            .with_context(|| {
                format!(
                    "Failed to convert {} {} to {}",
                    format_number(value),
                    from_unit,
                    to_unit
                )
            })?;

        println!(
            "{} {} = {} {}",
            format_number(value),
            from_unit,
            format_number(conversion_result),
            to_unit
        );
        return Ok(());
    }

    // If we get here, neither temperature nor length parsing worked for both units
    // Check if it's a mixed unit type error
    let temp_from_ok = parse_temperature_unit(from).is_ok();
    let temp_to_ok = parse_temperature_unit(to).is_ok();
    let length_from_ok = parse_length_unit(from).is_ok();
    let length_to_ok = parse_length_unit(to).is_ok();

    if (temp_from_ok && length_to_ok) || (length_from_ok && temp_to_ok) {
        return Err(anyhow!(
            "Cannot convert between different unit types. '{}' and '{}' are from different categories (temperature vs length).",
            from, to
        ));
    }

    // If both units are invalid, provide helpful error message
    let mut error_msg = String::new();

    if !temp_from_ok && !length_from_ok {
        error_msg.push_str(&format!("Invalid source unit: '{}'\n", from));
    }
    if !temp_to_ok && !length_to_ok {
        error_msg.push_str(&format!("Invalid target unit: '{}'\n", to));
    }

    error_msg.push_str("\nSupported units:\n");
    error_msg.push_str("Temperature: celsius (c), fahrenheit (f), kelvin (k)\n");
    error_msg.push_str("Length: centimeter (cm), inch (in), kilometer (km), miles (mi)\n");

    // Try to provide suggestions
    let temp_units = &["celsius", "c", "fahrenheit", "f", "kelvin", "k"];
    let length_units = &[
        "centimeter",
        "cm",
        "inch",
        "in",
        "kilometer",
        "km",
        "miles",
        "mi",
    ];

    if !temp_from_ok && !length_from_ok {
        if let Some(suggestion) = find_closest_match(from, temp_units) {
            error_msg.push_str(&format!(
                "\nDid you mean '{}' for the source unit?",
                suggestion
            ));
        } else if let Some(suggestion) = find_closest_match(from, length_units) {
            error_msg.push_str(&format!(
                "\nDid you mean '{}' for the source unit?",
                suggestion
            ));
        }
    }

    if !temp_to_ok && !length_to_ok {
        if let Some(suggestion) = find_closest_match(to, temp_units) {
            error_msg.push_str(&format!(
                "\nDid you mean '{}' for the target unit?",
                suggestion
            ));
        } else if let Some(suggestion) = find_closest_match(to, length_units) {
            error_msg.push_str(&format!(
                "\nDid you mean '{}' for the target unit?",
                suggestion
            ));
        }
    }

    Err(anyhow!(error_msg))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Temperature { from, to, value } => {
            let conversion_result = convert_temperature(value, from.clone(), to.clone())
                .with_context(|| {
                    format!(
                        "Failed to convert {} {} to {}",
                        format_number(value),
                        from,
                        to
                    )
                })?;

            println!(
                "{} {} = {} {}",
                format_number(value),
                from,
                format_number(conversion_result),
                to
            );
        }
        Commands::Length { from, to, value } => {
            let conversion_result =
                convert_length(value, from.clone(), to.clone()).with_context(|| {
                    format!(
                        "Failed to convert {} {} to {}",
                        format_number(value),
                        from,
                        to
                    )
                })?;

            println!(
                "{} {} = {} {}",
                format_number(value),
                from,
                format_number(conversion_result),
                to
            );
        }
        Commands::Convert { from, to, value } => {
            detect_and_convert(&from, &to, value)?;
        }
    }

    Ok(())
}
