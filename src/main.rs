use anyhow::{Ok, anyhow};
use clap::{Parser, Subcommand, ValueEnum};

mod conv;

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

fn convert_temperature(value: f64, from: Degree, to: Degree) -> Result<f64, anyhow::Error> {
    match from {
        Degree::Kelvin if value < 0.0 => {
            return Err(anyhow!(
                "Kelvin temperature cannot be negative. minimum is 0 K (absolute zero)."
            ));
        }
        Degree::Celsius if value < -273.15 => {
            return Err(anyhow!(
                "Celsius temperature cannot be below absolute zero (-273.15 °C)."
            ));
        }
        Degree::Fahrenheit if value < -459.67 => {
            return Err(anyhow!(
                "Fahrenheit temperature cannot be below absolute zero (-459.67 °F)."
            ));
        }
        _ => {}
    }

    let converter = TemperatureConverter::new(value, from.into());
    let result = converter.convert_to(to.into());

    Ok(result)
}

fn convert_length(value: f64, from: Length, to: Length) -> Result<f64, anyhow::Error> {
    if value < 0.0 {
        return Err(anyhow!("Length cannot be negative."));
    }

    let converter = LengthConverter::new(value, from.into());
    let result = converter.convert_to(to.into());

    Ok(result)
}

fn parse_temperature_unit(unit: &str) -> Result<Degree, anyhow::Error> {
    match unit.to_lowercase().as_str() {
        "celsius" | "c" => Ok(Degree::Celsius),
        "fahrenheit" | "f" => Ok(Degree::Fahrenheit),
        "kelvin" | "k" => Ok(Degree::Kelvin),
        _ => Err(anyhow!(
            "Invalid temperature unit: {}. Valid units are: celsius, fahrenheit, kelvin",
            unit
        )),
    }
}

fn parse_length_unit(unit: &str) -> Result<Length, anyhow::Error> {
    match unit.to_lowercase().as_str() {
        "centimeter" | "cm" => Ok(Length::Centimeter),
        "inch" | "in" => Ok(Length::Inch),
        "kilometer" | "km" => Ok(Length::Kilometer),
        "miles" | "mi" => Ok(Length::Miles),
        _ => Err(anyhow!(
            "Invalid length unit: {}. Valid units are: centimeter, inch, kilometer, miles",
            unit
        )),
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Temperature { from, to, value } => {
            let res = convert_temperature(value, from.clone(), to.clone());
            println!("{:.2} {:?} is {:?} {:?}", value, from, res, to);
            Ok(())
        }
        Commands::Length { from, to, value } => {
            let result = convert_length(value, from.clone(), to.clone())?;
            println!("{:.2}{:?} = {:.2}{:?}", value, from, result, to);
            Ok(())
        }
        Commands::Convert {
            r#type,
            from,
            to,
            value,
        } => {
            match r#type {
                ConversionType::Degree => {
                    let from_unit = parse_temperature_unit(&from)?;
                    let to_unit = parse_temperature_unit(&to)?;
                    let result = convert_temperature(value, from_unit.clone(), to_unit.clone())?;
                    println!("{:.2}{:?} = {:.2}{:?}", value, from_unit, result, to_unit);
                }
                ConversionType::Length => {
                    let from_unit = parse_length_unit(&from)?;
                    let to_unit = parse_length_unit(&to)?;
                    let result = convert_length(value, from_unit.clone(), to_unit.clone())?;
                    println!("{:.2}{:?} = {:.2}{:?}", value, from_unit, result, to_unit);
                }
            }
            Ok(())
        }
    }
}
