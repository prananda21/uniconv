use clap::{Parser, Subcommand};

mod conv;

use conv::{Degree, Length, LengthConverter, TemperatureConverter};

fn main() {
    println!("Temperature Converter");
    println!("========================");

    // example 1: convert 25 C to all units
    let temp_c = TemperatureConverter::new(25.0, Degree::Celsius);
    let res = temp_c.convert_to_all();
    println!("25 C convert to: {}", res);

    // example 2: conver 100 cm to all units
    let length_cm = LengthConverter::new(10.0, Length::Centimeter);
    let res = length_cm.convert_to_all();
    println!("100 cm convert to: {}", res);
}
