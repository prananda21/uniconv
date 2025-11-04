# Contributing to UniConv

Thank you for your interest in contributing to UniConv! We welcome contributions from everyone, whether you're fixing bugs, adding features, improving documentation, or suggesting new ideas.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- Basic familiarity with Rust and command-line tools

### Setting up the Development Environment

1. **Fork and Clone**
   ```bash
   git clone https://github.com/yourusername/uniconv.git
   cd uniconv
   ```

2. **Build and Test**
   ```bash
   # Build the project
   cargo build
   
   # Run tests
   cargo test
   
   # Run the application
   cargo run -- --help
   
   # Test the smart convert feature
   cargo run -- convert --from celsius --to fahrenheit --value 25
   ```

3. **Install Development Tools** (Optional but recommended)
   ```bash
   # Install rustfmt for code formatting
   rustup component add rustfmt
   
   # Install clippy for linting
   rustup component add clippy
   ```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 2. Make Your Changes

- Follow Rust naming conventions and idioms
- Write clear, self-documenting code
- Add tests for new functionality
- Update documentation as needed

### 3. Code Quality Checks

Before submitting, ensure your code passes all quality checks:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test

# Check for compilation errors
cargo check
```

### 4. Commit Your Changes

Use clear, descriptive commit messages:

```bash
git add .
git commit -m "Add volume conversion support

- Implement liter/gallon/cubic meter conversions
- Add VolumeConverter struct and tests
- Update CLI to support volume subcommand
- Integrate with smart convert command for auto-detection"
```

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:
- Clear description of changes
- Reference to related issues (if any)
- Screenshots or examples (if applicable)

## Types of Contributions

### 🐛 Bug Fixes

- Look for issues labeled `bug`
- Reproduce the issue locally
- Write a test that demonstrates the bug
- Fix the bug and ensure the test passes
- Update relevant documentation

### ✨ New Features

- Check existing issues or create a new one to discuss the feature
- Implement the feature following existing patterns
- Add comprehensive tests
- Update documentation and help text
- Add examples to README/USAGE

### 📚 Documentation

- Fix typos or unclear explanations
- Add more examples
- Improve code comments
- Update README or USAGE files

### 🧪 Testing

- Add tests for uncovered code paths
- Improve existing test cases
- Add integration tests
- Performance benchmarks

## Code Guidelines

### Rust Style

- Follow standard Rust formatting (`cargo fmt`)
- Use `clippy` suggestions (`cargo clippy`)
- Prefer explicit types when it improves clarity
- Use descriptive variable and function names
- Add documentation comments for public APIs

### Project Structure

```
src/
├── main.rs              # CLI interface and main logic
└── conv/
    ├── mod.rs           # Module definitions and shared enums
    ├── temperature.rs   # Temperature conversions
    ├── length.rs        # Length conversions
    └── new_unit.rs      # New unit type (follow this pattern)
```

### Adding New Unit Types

When adding a new unit type (e.g., Volume, Weight), follow this pattern:

1. **Create the conversion module** (`src/conv/volume.rs`):
   ```rust
   use super::Volume;
   
   pub struct VolumeConverter {
       pub value: f64,
       pub unit: Volume,
   }
   
   impl VolumeConverter {
       pub fn new(value: f64, unit: Volume) -> Self {
           Self { value, unit }
       }
   
       pub fn convert_to(&self, target_unit: Volume) -> f64 {
           // Conversion logic
       }
   }
   
   #[cfg(test)]
   mod tests {
       // Comprehensive tests
   }
   ```

2. **Add enum to `mod.rs`**:
   ```rust
   #[derive(Clone, Debug, PartialEq)]
   pub enum Volume {
       Liter,
       Gallon,
       CubicMeter,
   }
   ```

3. **Update CLI in `main.rs`**:
   - Add CLI enum variant
   - Add conversion function
   - Update help text
   - Add unit parsing to `detect_and_convert` function

### Testing Guidelines

- Write unit tests for all conversion functions
- Test edge cases (zero, negative values, very large numbers)
- Test error conditions
- Use descriptive test names:
  ```rust
  #[test]
  fn test_celsius_to_fahrenheit_boiling_point() {
      assert_eq!(convert_celsius_to_fahrenheit(100.0), 212.0);
  }
  ```

### Documentation Guidelines

- Use clear, concise language
- Include examples for all features
- Document error conditions
- Keep README and USAGE files up to date

## Specific Contribution Areas

### Priority Features (Good First Issues)

- **Volume conversions** (liters, gallons, cubic meters)
- **Weight conversions** (grams, pounds, kilograms)
- **Area conversions** (square meters, square feet, acres)
- **Speed conversions** (mph, km/h, m/s)

### Advanced Features

- **Interactive mode** for multiple conversions
- **Configuration file** support (JSON/YAML)
- **Batch processing** from file input
- **Custom units** with user-defined conversion factors
- **Precision control** for output formatting
- **Fuzzy unit matching** for better typo detection

### Infrastructure Improvements

- **Performance benchmarks**
- **Integration tests**
- **GitHub Actions CI/CD**
- **Package distribution** (Homebrew, Cargo, etc.)

## Pull Request Guidelines

### Before Submitting

- [ ] Code builds without warnings (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (for significant changes)

### Pull Request Description

Include:
- **What**: Brief description of changes
- **Why**: Reason for the change
- **How**: Technical approach (if complex)
- **Testing**: How the change was tested
- **Screenshots**: For UI/CLI changes

### Review Process

1. Automated checks run (formatting, tests, clippy)
2. Code review by maintainers
3. Address feedback and make revisions
4. Final approval and merge

## Command Examples for Testing

When contributing, test your changes with these example commands:

```bash
# Smart convert (recommended usage)
cargo run -- convert --from celsius --to fahrenheit --value 25
cargo run -- convert --from c --to f --value 0
cargo run -- convert --from cm --to inch --value 100
cargo run -- convert --from kilometers --to miles --value 5

# Test error handling
cargo run -- convert --from celsius --to kilometers --value 25  # Mixed types
cargo run -- convert --from celcius --to fahrenheit --value 25  # Typo
cargo run -- convert --from meter --to inch --value 100         # Invalid unit

# Dedicated commands (still supported)
cargo run -- temperature --from celsius --to fahrenheit --value 25
cargo run -- length --from cm --to inch --value 100
```

## Getting Help

- **Questions**: Open a GitHub issue with the `question` label
- **Bugs**: Open a GitHub issue with the `bug` label
- **Feature Requests**: Open a GitHub issue with the `enhancement` label
- **Discussion**: Use GitHub Discussions for general topics

## Recognition

Contributors will be:
- Listed in the project's contributor list
- Mentioned in release notes for significant contributions
- Given credit in relevant documentation

Thank you for contributing to UniConv! 🦀✨