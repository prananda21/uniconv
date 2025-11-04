# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Smart Convert Command**: Automatic unit type detection eliminates need for `--type` parameter
- **Enhanced Temperature Output**: Proper symbols (°C, °F, K) in conversion results
- **Mixed Unit Type Detection**: Prevents conversion between incompatible unit types
- **Intelligent Error Messages**: Typo detection and suggestions for invalid units
- **Fuzzy Unit Matching**: Levenshtein distance algorithm for unit name suggestions

### Changed
- **Simplified CLI**: `convert` command no longer requires `--type degree` or `--type length`
- **Automatic Detection**: Units are automatically categorized as temperature or length
- **Better Error Handling**: More descriptive error messages with actionable suggestions
- **Improved Help Text**: Updated examples to showcase smart conversion features

### Technical Improvements
- New `detect_and_convert()` function for intelligent unit type detection
- Enhanced unit parsing with fallback logic
- Improved error context with specific suggestions for typos
- Better separation of concerns between unit detection and conversion logic

### Migration
- Old format: `uniconv convert --type degree --from celsius --to fahrenheit --value 25`
- New format: `uniconv convert --from celsius --to fahrenheit --value 25` (simpler!)
- Dedicated commands (`temperature`, `length`) remain unchanged for backward compatibility

## [1.0.0] - Initial Release

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

### Core Features
- **Temperature Conversions**: All combinations between Celsius, Fahrenheit, and Kelvin with proper symbols
- **Length Conversions**: All combinations between Centimeter, Inch, Kilometer, and Miles
- **Smart Unit Detection**: Automatically determines conversion type from unit names
- **Input Validation**: Prevents impossible temperatures below absolute zero and negative lengths
- **Accurate Formulas**: High-precision conversion calculations with overflow protection
- **Intelligent Error Messages**: Typo detection, suggestions, and mixed-type prevention
- **Help System**: Built-in help for all commands with updated examples
- **Flexible Input**: Multiple unit name formats, abbreviations, and automatic detection

### Technical Details
- Built with Rust 1.70+
- Uses clap 4.5+ for CLI argument parsing
- Uses anyhow for error handling
- Modular code structure with separate modules for different unit types
- Comprehensive unit tests covering all conversion scenarios
- Zero-dependency core conversion logic

---

## Version History Summary

- **Unreleased** - Smart convert feature with automatic unit detection
- **v1.0.0** - Initial release with temperature and length conversions
- **Future versions** - Additional unit types (volume, weight, speed, area, etc.)

## Roadmap

### Planned Features
- **Volume conversions** (liters, gallons, cubic meters)
- **Weight conversions** (grams, pounds, kilograms)
- **Area conversions** (square meters, square feet, acres)
- **Speed conversions** (mph, km/h, m/s)
- **Interactive mode** for multiple conversions
- **Configuration file** support
- **Batch processing** from file input
- **Custom units** with user-defined conversion factors