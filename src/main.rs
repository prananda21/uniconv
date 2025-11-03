use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};

mod conv;
mod errors;

use conv::{Degree, Length, LengthConverter, TemperatureConverter};

#[derive(Parser)]
#[command(name = "uniconv")]
#[command(about = "A universal unit converter (based on temperature and length")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, ValueEnum)]
enum ConversionType {
    Degree,
    Length,
}

#[derive(Subcommand)]
enum Commands {
    Temperature {
        #[arg(long)]
        from: Degree,
        #[arg(long)]
        to: Degree,
        #[arg(long)]
        value: f64,
    },
    Length {
        #[arg(long)]
        from: Length,
        #[arg(long)]
        to: Length,
        #[arg(long)]
        value: f64,
    },
    Convert {
        #[arg(long)]
        r#type: ConversionType,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        value: f64,
    },
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

fn parse_temperature_unit(unit: &str) -> Result<Degree> {
    match unit.to_lowercase().as_str() {
        "celsius" | "c" => Ok(Degree::Celsius),
        "fahrenheit" | "f" => Ok(Degree::Fahrenheit),
        "kelvin" | "k" => Ok(Degree::Kelvin),
        _ => Err(anyhow!(
            "Invalid temperature unit: '{}'. Valid units are: celsius (c), fahrenheit (f), kelvin (k)",
            unit
        )),
    }
}

fn parse_length_unit(unit: &str) -> Result<Length> {
    match unit.to_lowercase().as_str() {
        "centimeter" | "cm" => Ok(Length::Centimeter),
        "inch" | "in" => Ok(Length::Inch),
        "kilometer" | "km" => Ok(Length::Kilometer),
        "miles" | "mi" => Ok(Length::Miles),
        _ => Err(anyhow!(
            "Invalid length unit: '{}'. Valid units are: centimeter (cm), inch (in), kilometer (km), miles (mi)",
            unit
        )),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Temperature { from, to, value } => {
            let conversion_result = convert_temperature(value, from.clone(), to.clone())
                .with_context(|| {
                    format!("Failed to convert {:.2} {:?} to {:?}", value, from, to)
                })?;

            println!(
                "{:.6} {:?} = {:.6} {:?}",
                value, from, conversion_result, to
            );
        }
        Commands::Length { from, to, value } => {
            let conversion_result =
                convert_length(value, from.clone(), to.clone()).with_context(|| {
                    format!("Failed to convert {:.2} {:?} to {:?}", value, from, to)
                })?;

            println!(
                "{:.6} {:?} = {:.6} {:?}",
                value, from, conversion_result, to
            );
        }
        Commands::Convert {
            r#type,
            from,
            to,
            value,
        } => match r#type {
            ConversionType::Degree => {
                let from_unit = parse_temperature_unit(&from)
                    .with_context(|| format!("Invalid source temperature unit: '{}'", from))?;
                let to_unit = parse_temperature_unit(&to)
                    .with_context(|| format!("Invalid target temperature unit: '{}'", to))?;

                let conversion_result = convert_temperature(value, from_unit, to_unit)
                    .with_context(|| {
                        format!("Failed to convert {:.2} {} to {}", value, from, to)
                    })?;

                println!("{:.6} {} = {:.6} {}", value, from, conversion_result, to);
            }
            ConversionType::Length => {
                let from_unit = parse_length_unit(&from)
                    .with_context(|| format!("Invalid source length unit: '{}'", from))?;
                let to_unit = parse_length_unit(&to)
                    .with_context(|| format!("Invalid target length unit: '{}'", to))?;

                let conversion_result =
                    convert_length(value, from_unit, to_unit).with_context(|| {
                        format!("Failed to convert {:.2} {} to {}", value, from, to)
                    })?;

                println!("{:.6} {} = {:.6} {}", value, from, conversion_result, to);
            }
        },
    }

    Ok(())
}
