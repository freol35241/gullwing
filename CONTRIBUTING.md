# Contributing to gullwing

Thank you for your interest in contributing to gullwing! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/gullwing.git
   cd gullwing
   ```
3. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building

```bash
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test --all-features

# Run specific test suite
cargo test --test roundtrip
cargo test --test error_cases
cargo test --test shuffle_integration

# Run with output
cargo test -- --nocapture
```

### Running Benchmarks

```bash
cargo bench
```

### Running Examples

```bash
# Build and run the shuffle example
cargo run --example shuffle "{name} {value:d}" "{value}: {name}"
```

## Code Quality

### Before Submitting

Please ensure your code passes all quality checks:

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy --all-features -- -D warnings

# Run all tests
cargo test --all-features

# Build documentation
cargo doc --no-deps
```

### Code Style

- Follow Rust standard formatting (use `cargo fmt`)
- Add documentation comments (`///`) for all public APIs
- Include examples in doc comments where appropriate
- Keep functions focused and well-named

### Documentation

- All public APIs must have documentation
- Include examples in doc comments for complex functionality
- Update README.md if adding major features
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format

## Testing

### Test Coverage

We aim for high test coverage. When adding new features:

1. **Unit tests**: Test individual functions and methods
2. **Integration tests**: Test feature interactions
3. **Property-based tests**: Use proptest for round-trip testing
4. **Error cases**: Test error handling and edge cases

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        let input = ...;

        // Act
        let result = ...;

        // Assert
        assert_eq!(result, expected);
    }
}
```

### Property-Based Testing

For round-trip operations (format → parse → original value):

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn roundtrip_property(n in -1000i64..1000i64) {
        // Test that format and parse are inverses
        let formatted = format_value(n)?;
        let parsed = parse_value(&formatted)?;
        prop_assert_eq!(n, parsed);
    }
}
```

## Pull Request Process

1. **Update documentation**: Ensure all changes are documented
2. **Add tests**: All new code should have corresponding tests
3. **Update CHANGELOG.md**: Add your changes under `[Unreleased]`
4. **Ensure CI passes**: All tests and checks must pass
5. **Write clear commit messages**: Follow conventional commits format
6. **Reference issues**: Link to relevant issues in PR description

### Commit Message Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>: <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

Examples:
```
feat: add support for custom type converters

fix: handle negative numbers in hex formatting

docs: improve README quick start section

test: add property-based tests for float parsing
```

## Feature Requests and Bug Reports

### Bug Reports

When filing a bug report, please include:

1. **Description**: Clear description of the bug
2. **Steps to reproduce**: Minimal code example
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Environment**: Rust version, OS, etc.

Example:
```markdown
## Bug Description
Parser fails to handle escaped braces in format strings

## Steps to Reproduce
```rust
let parser = Parser::new("{{literal}}");
// Fails here
```

## Expected Behavior
Should parse escaped braces as literal text

## Actual Behavior
Returns an error: "invalid format specification"

## Environment
- gullwing version: 0.9.0
- Rust version: 1.75.0
- OS: Linux
```

### Feature Requests

When requesting a feature:

1. **Use case**: Describe the problem you're trying to solve
2. **Proposed solution**: How you envision the feature working
3. **Alternatives**: Other approaches you've considered
4. **References**: Links to similar features in other libraries

## Areas for Contribution

Looking for ways to contribute? Check out:

1. **[Open Issues](https://github.com/freol35241/gullwing/issues)**: Browse issues tagged with `good first issue` or `help wanted`
2. **Feature Gap Analysis**: See [FEATURE_GAP_ANALYSIS.md](FEATURE_GAP_ANALYSIS.md) for features not yet implemented
3. **Performance**: Optimize hot paths identified in benchmarks
4. **Documentation**: Improve examples, guides, and API docs
5. **Testing**: Increase test coverage, add edge cases

### High-Priority Features

From the Feature Gap Analysis, high-priority items include:

- **Custom Type Converters**: Allow users to register custom parsers
- **Span Information**: Track character positions of matches
- **Case-Insensitive Parsing**: Add option for case-insensitive matching
- **Word/Letter Type Specifiers**: Implement `w`, `W`, `l` types

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors.

### Standards

- Be respectful and considerate
- Welcome newcomers and help them learn
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment, discrimination, or intimidation
- Trolling, insulting comments, or personal attacks
- Publishing others' private information
- Other conduct which could reasonably be considered inappropriate

## Questions?

If you have questions about contributing:

- Open a [GitHub Discussion](https://github.com/freol35241/gullwing/discussions)
- File an [Issue](https://github.com/freol35241/gullwing/issues) with the `question` label
- Check existing documentation and issues

## License

By contributing to gullwing, you agree that your contributions will be licensed under the Apache License, Version 2.0.

---

Thank you for contributing to gullwing! 🦀
