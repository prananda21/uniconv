# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project setup with Rust and Cargo
- Temperature conversion support (Celsius, Fahrenheit, Kelvin)
- Length conversion support (Centimeter, Inch, Kilometer, Miles)
- CLI interface with clap derive macros
- Multiple command formats:
  - Dedicated `temperature` subcommand
  - Dedicated `length` subcommand  
  - Generic `convert` command with unit type specification
- Input validation and error handling
- Support for unit abbreviations (c, f, k, cm, in, km, mi)
- Comprehensive test suite
- Documentation (README, USAGE, CHANGELOG)
- MIT License

### Features
- **Temperature Conversions**: All combinations between Celsius, Fahrenheit, and Kelvin
- **Length Conversions**: All combinations between Centimeter, Inch, Kilometer, and Miles
- **Input Validation**: Prevents impossible temperatures below absolute zero
- **Length Validation**: Prevents negative length values
- **Accurate Formulas**: High-precision conversion calculations
- **Error Messages**: Clear, helpful error descriptions
- **Help System**: Built-in help for all commands and options
- **Flexible Input**: Multiple unit name formats and abbreviations

### Technical Details
- Built with Rust 1.70+
- Uses clap 4.5+ for CLI argument parsing
- Uses anyhow for error handling
- Modular code structure with separate modules for different unit types
- Comprehensive unit tests covering all conversion scenarios
- Zero-dependency core conversion logic

## [1.0.0] - 2024-01-XX

### Added
- Initial release of UniConv
- Core temperature and length conversion functionality
- Command-line interface
- Documentation and examples

---

## Version History Summary

- **v1.0.0** (Planned) - Initial release with temperature and length conversions
- **Future versions** - Additional unit types (volume, weight, speed, etc.)