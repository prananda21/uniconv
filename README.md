# UniConv - Universal Unit Converter

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast, accurate, and easy-to-use command-line unit converter built with Rust. UniConv supports temperature and length conversions with multiple input formats and comprehensive error handling.

## Features

- 🌡️ **Temperature Conversion**: Celsius, Fahrenheit, and Kelvin
- 📏 **Length Conversion**: Centimeters, Inches, Kilometers, and Miles
- 🚀 **Multiple Command Formats**: Dedicated subcommands and generic converter
- ✅ **Input Validation**: Prevents impossible values (negative lengths, sub-absolute-zero temperatures)
- 🎯 **Accurate Conversions**: High-precision formulas for all unit conversions
- 📝 **Short Unit Names**: Support for abbreviations (cm, in, km, mi, c, f, k)
- 🛡️ **Error Handling**: Clear error messages for invalid inputs
- 📖 **Built-in Help**: Comprehensive help system with examples

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/uniconv.git
cd uniconv

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
# Temperature conversion
./target/release/uniconv temperature --from celsius --to fahrenheit --value 100

# Length conversion
./target/release/uniconv length --from centimeter --to inch --value 100

# Generic conversion (your preferred format)
./target/release/uniconv convert --unit-type temperature --from celsius --to fahrenheit --value 100
```

## Usage

### Command Structure

UniConv provides three ways to perform conversions:

1. **Dedicated Temperature Command**
2. **Dedicated Length Command**
3. **Generic Convert Command**

### Temperature Conversions

```bash
# Using temperature subcommand
uniconv temperature --from <UNIT> --to <UNIT> --value <NUMBER>

# Using generic convert command
uniconv convert --unit-type temperature --from <UNIT> --to <UNIT> --value <NUMBER>
```

**Supported Temperature Units:**
- `celsius` or `c` → Degrees Celsius (°C)
- `fahrenheit` or `f` → Degrees Fahrenheit (°F)
- `kelvin` or `k` → Kelvin (K)

**Examples:**
```bash
# Boiling point of water
$ uniconv temperature --from celsius --to fahrenheit --value 100
100.00°C = 212.00°F

# Absolute zero
$ uniconv temperature --from kelvin --to celsius --value 0
0.00K = -273.15°C

# Room temperature (using abbreviations)
$ uniconv convert --unit-type temperature --from c --to f --value 20
20.00°C = 68.00°F
```

### Length Conversions

```bash
# Using length subcommand
uniconv length --from <UNIT> --to <UNIT> --value <NUMBER>

# Using generic convert command
uniconv convert --unit-type length --from <UNIT> --to <UNIT> --value <NUMBER>
```

**Supported Length Units:**
- `centimeter` or `cm` → Centimeters (cm)
- `inch` or `in` → Inches (in)
- `kilometer` or `km` → Kilometers (km)
- `miles` or `mi` → Miles (mi)

**Examples:**
```bash
# Convert height
$ uniconv length --from centimeter --to inch --value 180
180.00cm = 70.87in

# Convert distance
$ uniconv length --from kilometer --to miles --value 42.195
42.20km = 26.22mi

# Convert using abbreviations
$ uniconv convert --unit-type length --from ft --to cm --value 6
# Error: Invalid length unit: ft. Valid units are: centimeter, inch, kilometer, miles
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

UniConv validates all inputs and provides clear error messages:

### Temperature Validation
```bash
# Below absolute zero
$ uniconv temperature --from celsius --to fahrenheit --value -300
Error: Celsius temperature cannot be below -273.15°C (absolute zero).

$ uniconv temperature --from kelvin --to celsius --value -1
Error: Kelvin temperature cannot be negative. Minimum is 0K (absolute zero).
```

### Length Validation
```bash
# Negative length
$ uniconv length --from centimeter --to inch --value -10

Error: Length cannot be negative.
```

### Invalid Units
```bash
# Invalid unit name
$ uniconv convert --unit-type temperature --from invalid --to celsius --value 100
Error: Invalid temperature unit: invalid. Valid units are: celsius, fahrenheit, kelvin
```

## Examples & Use Cases

### Common Temperature Conversions

```bash
# Weather conversions
uniconv temperature --from celsius --to fahrenheit --value 25    # Room temp
uniconv temperature --from fahrenheit --to celsius --value 98.6  # Body temp
uniconv temperature --from celsius --to kelvin --value 0         # Freezing point

# Cooking temperatures
uniconv temperature --from fahrenheit --to celsius --value 350   # Oven temp
uniconv temperature --from celsius --to fahrenheit --value 100   # Boiling water
```

### Common Length Conversions

```bash
# Height conversions
uniconv length --from centimeter --to inch --value 175    # Person height
uniconv length --from inch --to centimeter --value 72     # 6 feet in inches

# Distance conversions
uniconv length --from kilometer --to miles --value 5      # 5K run
uniconv length --from miles --to kilometer --value 26.2   # Marathon
```

## Development

### Project Structure

```
uniconv/
├── src/
│   ├── main.rs              # CLI interface and argument parsing
│   └── conv/
│       ├── mod.rs           # Module definitions and enums
│       ├── temperature.rs   # Temperature conversion logic
│       └── length.rs        # Length conversion logic
├── Cargo.toml              # Dependencies and project metadata
├── README.md               # This file
└── USAGE.md               # Detailed usage examples
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

# Run specific test module
cargo test temperature
cargo test length
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Run tests (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## Roadmap

- [ ] **Volume conversions** (liters, gallons, cubic meters)
- [ ] **Weight conversions** (grams, pounds, kilograms)
- [ ] **Speed conversions** (mph, km/h, m/s)
- [ ] **JSON/YAML configuration** file support
- [ ] **Batch conversion** from file input
- [ ] **Interactive mode** for multiple conversions
- [ ] **Custom conversion factors** for specialized units
- [ ] **Web API** for programmatic access

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- CLI parsing powered by [clap](https://github.com/clap-rs/clap)
- Inspired by the need for a fast, accurate unit converter

---

**Made with 🦀 Rust** | **Fast • Accurate • Reliable**