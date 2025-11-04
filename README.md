# UniConv - Universal Unit Converter

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast, accurate, and easy-to-use command-line unit converter built with Rust. UniConv supports temperature and length conversions with automatic unit detection, multiple input formats, and comprehensive error handling.

## Features

- 🌡️ **Temperature Conversion**: Celsius, Fahrenheit, and Kelvin with proper symbols (°C, °F, K)
- 📏 **Length Conversion**: Centimeters, Inches, Kilometers, and Miles
- 🤖 **Automatic Unit Detection**: No need to specify conversion type - automatically detects temperature vs length
- 🚀 **Multiple Command Formats**: Dedicated subcommands and intelligent generic converter
- ✅ **Advanced Input Validation**: Prevents impossible values and detects edge cases
- 🛡️ **Comprehensive Error Handling**: Validates NaN, infinity, and extreme values
- 🎯 **Accurate Conversions**: High-precision formulas with overflow protection
- 📝 **Short Unit Names**: Support for abbreviations (cm, in, km, mi, c, f, k)
- 🔍 **Intelligent Error Messages**: Clear guidance with suggested fixes and typo detection
- 📖 **Built-in Help**: Comprehensive help system with examples
- ⚡ **Robust Operation**: Graceful handling of all error conditions

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from Source

```bash
# Build the project
cargo build --release

# The binary will be available at ./target/release/uniconv
```

### Development Build

```bash
# For development/testing
cargo build

# Run tests
cargo test

# Run with cargo
cargo run -- --help
```

## Quick Start

```bash
# Temperature conversion (automatically detected)
./target/release/uniconv convert --from celsius --to fahrenheit --value 100

# Length conversion (automatically detected)
./target/release/uniconv convert --from centimeter --to inch --value 100

# Or use dedicated commands
./target/release/uniconv temperature --from celsius --to fahrenheit --value 100
```

## Usage

### Command Structure

UniConv provides three ways to perform conversions:

1. **Smart Convert Command** (Recommended) - Automatically detects unit type
2. **Dedicated Temperature Command**
3. **Dedicated Length Command**

### Smart Convert Command (Recommended)

The `convert` command automatically detects whether you're converting temperature or length units:

```bash
# No need to specify --type, just provide units!
uniconv convert --from <UNIT> --to <UNIT> --value <NUMBER>
```

**Examples:**
```bash
# Temperature conversions (auto-detected)
$ uniconv convert --from celsius --to fahrenheit --value 25
25 °C = 77 °F

$ uniconv convert --from c --to k --value 100
100 °C = 373 K

# Length conversions (auto-detected)
$ uniconv convert --from cm --to inch --value 180
180 cm = 71 in

$ uniconv convert --from kilometers --to miles --value 42
42 km = 26 mi
```

### Temperature Conversions

```bash
# Using temperature subcommand
uniconv temperature --from <UNIT> --to <UNIT> --value <NUMBER>

# Using smart convert command (recommended)
uniconv convert --from <UNIT> --to <UNIT> --value <NUMBER>
```

**Supported Temperature Units:**
- `celsius` or `c` → Degrees Celsius (°C)
- `fahrenheit` or `f` → Degrees Fahrenheit (°F)
- `kelvin` or `k` → Kelvin (K)

**Examples:**
```bash
# Boiling point of water
$ uniconv convert --from celsius --to fahrenheit --value 100
100 °C = 212 °F

# Absolute zero
$ uniconv convert --from kelvin --to celsius --value 0
0 K = -273 °C

# Room temperature (using abbreviations)
$ uniconv convert --from c --to f --value 20
20 °C = 68 °F
```

### Length Conversions

```bash
# Using length subcommand
uniconv length --from <UNIT> --to <UNIT> --value <NUMBER>

# Using smart convert command (recommended)
uniconv convert --from <UNIT> --to <UNIT> --value <NUMBER>
```

**Supported Length Units:**
- `centimeter` or `cm` → Centimeters (cm)
- `inch` or `in` → Inches (in)
- `kilometer` or `km` → Kilometers (km)
- `miles` or `mi` → Miles (mi)

**Examples:**
```bash
# Convert height
$ uniconv convert --from centimeter --to inch --value 180
180 cm = 71 in

# Convert distance
$ uniconv convert --from kilometer --to miles --value 42.195
42 km = 26 mi

# Convert using abbreviations
$ uniconv convert --from cm --to km --value 100000
100000 cm = 1 km
```

### Help System

```bash
# General help
uniconv --help

# Command-specific help
uniconv temperature --help
uniconv length --help
uniconv convert --help
```

## Conversion Formulas

### Temperature Conversions

| From | To | Formula |
|------|----|----|
| Celsius | Fahrenheit | (C × 9/5) + 32 |
| Celsius | Kelvin | C + 273.15 |
| Fahrenheit | Celsius | (F - 32) × 5/9 |
| Fahrenheit | Kelvin | (F - 32) × 5/9 + 273.15 |
| Kelvin | Celsius | K - 273.15 |
| Kelvin | Fahrenheit | (K - 273.15) × 9/5 + 32 |

### Length Conversions

| From | To | Multiplier |
|------|----|----|
| Centimeter | Inch | × 0.393701 |
| Centimeter | Kilometer | × 0.00001 |
| Centimeter | Miles | × 0.0000062137 |
| Inch | Centimeter | × 2.54 |
| Kilometer | Miles | × 0.621371 |
| Miles | Kilometer | × 1.609344 |

## Error Handling

UniConv validates all inputs and provides intelligent error messages with automatic unit type detection:

### Mixed Unit Type Detection
```bash
# Trying to convert between different unit types
$ uniconv convert --from celsius --to kilometers --value 25
Error: Cannot convert between different unit types.
 'celsius' and 'kilometers' are from different categories (temperature vs length).
```

### Invalid Unit Detection with Suggestions
```bash
# Typo in temperature unit
$ uniconv convert --from celcius --to fahrenheit --value 25
Error: Invalid source unit: 'celcius'

Supported units:
Temperature: celsius (c), fahrenheit (f), kelvin (k)
Length: centimeter (cm), inch (in), kilometer (km), miles (mi)

Did you mean 'celsius' for the source unit?

# Invalid length unit
$ uniconv convert --from meter --to inch --value 1
Error: Invalid source unit: 'meter'

Supported units:
Temperature: celsius (c), fahrenheit (f), kelvin (k)
Length: centimeter (cm), inch (in), kilometer (km), miles (mi)

Did you mean 'centimeter' for the source unit?
```

### Numeric Input Validation
```bash
# NaN (Not a Number) detection
$ uniconv convert --from celsius --to fahrenheit --value nan
Error: Temperature value cannot be NaN (Not a Number)

# Infinity detection
$ uniconv convert --from celsius --to fahrenheit --value inf
Error: Temperature value cannot be infinite

# Extremely large numbers
$ uniconv convert --from celsius --to fahrenheit --value 1e16
Error: Temperature value is too large (absolute value exceeds 1e15). Please use a smaller number.
```

### Physical Constraint Validation
#### Temperature Constraints
```bash
# Below absolute zero in Kelvin
$ uniconv convert --from kelvin --to celsius --value -10
Error: Kelvin temperature cannot be negative (-10K). Minimum is 0 K (absolute zero).

# Below absolute zero in Celsius
$ uniconv convert --from celsius --to fahrenheit --value -300
Error: Celsius temperature cannot be below absolute zero (-300°C < -273.15°C).

# Below absolute zero in Fahrenheit
$ uniconv convert --from fahrenheit --to celsius --value -500
Error: Fahrenheit temperature cannot be below absolute zero (-500°F < -459.67°F).
```

#### Length Constraints
```bash
# Negative length values
$ uniconv convert --from centimeter --to inch --value -10
Error: Length cannot be negative (-10). Please provide a positive value.

# Unrealistically large lengths
$ uniconv convert --from centimeter --to inch --value 1e13
Error: Length value 1e+13 is unrealistically large. Please check your input.
```

## Examples & Use Cases

### Smart Conversions (Recommended Usage)

```bash
# Weather conversions
uniconv convert --from celsius --to fahrenheit --value 25    # 25 °C = 77 °F
uniconv convert --from f --to c --value 98.6                 # 99 °F = 37 °C
uniconv convert --from celsius --to kelvin --value 0         # 0 °C = 273 K

# Height and distance conversions  
uniconv convert --from cm --to inch --value 175             # 175 cm = 69 in
uniconv convert --from kilometers --to miles --value 5       # 5 km = 3 mi
uniconv convert --from miles --to km --value 26.2           # 26 mi = 42 km

# Cooking temperatures
uniconv convert --from f --to c --value 350                 # 350 °F = 177 °C
uniconv convert --from celsius --to fahrenheit --value 100   # 100 °C = 212 °F

# Scientific applications
uniconv convert --from kelvin --to celsius --value 300       # 300 K = 27 °C
```

### Using Dedicated Commands

```bash
# If you prefer explicit commands
uniconv temperature --from celsius --to fahrenheit --value 25
uniconv length --from centimeter --to inch --value 175
```

## Key Advantages of Smart Convert

1. **Simpler Commands**: No need to specify `--type degree` or `--type length`
2. **Automatic Detection**: Intelligently determines unit type from context
3. **Mixed Type Protection**: Prevents converting between incompatible units
4. **Better Error Messages**: Suggests corrections for typos and invalid units
5. **Proper Symbols**: Temperature outputs show proper symbols (°C, °F, K)

## Development

### Project Structure

```
uniconv/
├── src/
│   ├── main.rs              # CLI interface with smart unit detection
│   ├── conv/
│   │   ├── mod.rs           # Unit enums and display formatting
│   │   ├── temperature.rs   # Temperature conversion logic
│   │   └── length.rs        # Length conversion logic
│   └── errors/
│       └── mod.rs           # Error handling modules
├── Cargo.toml              # Dependencies and project metadata
├── README.md               # This file
└── CHANGELOG.md            # Version history
```

### Dependencies

- **clap** - Command-line argument parsing with derive macros
- **anyhow** - Error handling and context
- **serde** - Serialization framework (for future JSON/config support)

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Test specific functionality
cargo test temperature
cargo test length
cargo test error_handling
```

## Migration from Previous Versions

If you were using the old format with `--type`, simply remove that parameter:

```bash
# Old format
uniconv convert --type degree --from celsius --to fahrenheit --value 25
uniconv convert --type length --from cm --to inch --value 100

# New format (simpler!)
uniconv convert --from celsius --to fahrenheit --value 25
uniconv convert --from cm --to inch --value 100
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgments

- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- CLI parsing powered by [clap](https://github.com/clap-rs/clap)
- Error handling enhanced with [anyhow](https://github.com/dtolnay/anyhow)
- Inspired by the need for a fast, accurate, and intelligent unit converter

---

**Made with 🦀 Rust** | **Fast • Accurate • Intelligent**