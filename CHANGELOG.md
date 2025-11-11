# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2025-11-11

### Added

#### Core Features
- **Format Specification Parser**: Full implementation of Python's Format Specification Mini-Language
  - Support for all alignment options (`<`, `>`, `^`, `=`)
  - Sign handling (`+`, `-`, ` `)
  - Zero-padding and custom fill characters
  - Width and precision specifications
  - Grouping separators (`,` and `_`)
  - Alternate form (`#`) for hex, binary, octal

- **Runtime Formatting**: Format values using runtime-determined format strings
  - Named field formatting with `Formatter::format_map()`
  - Positional field formatting with `Formatter::format_positional()`
  - Closure-based formatting with `Formatter::format_fn()`
  - Support for multiple value types: String, Int, UInt, Float, Bool, Char

- **Runtime Parsing**: Extract structured data from strings using format patterns
  - Exact matching with `Parser::parse()`
  - First-match search with `Parser::search()`
  - Multiple matches with `Parser::findall()`
  - Automatic type conversion based on format specifiers
  - Regex-based pattern matching for efficiency

#### Type Specifiers
- **String**: `s` (string, default)
- **Integer**: `d` (decimal), `b` (binary), `o` (octal), `x`/`X` (hexadecimal), `n` (number)
- **Float**: `f`/`F` (fixed-point), `e`/`E` (scientific), `g`/`G` (general), `%` (percentage)
- **Character**: `c` (character from integer code)

#### Value System
- `Value` enum supporting common Rust types
- Conversion traits for ergonomic value creation
- Type-safe accessors (`as_str()`, `as_int()`, `as_float()`, etc.)
- Fallible conversion methods (`to_int()`, `to_float()`, etc.)

#### Examples and Tools
- `shuffle` CLI tool for text transformation
- Comprehensive documentation with examples
- Benchmark suite measuring performance

#### Performance
- Format spec parsing: 26-128 ns (depending on complexity)
- Formatting operations: 104 ns - 1.22 µs (depending on operation)
- Parsing operations: 38.5 µs - 279 µs (includes regex matching)
- Zero-copy operations where possible

### Documentation
- Complete API documentation with examples
- README with quick start guide
- Implementation plan and feature gap analysis
- Performance benchmarks
- Comprehensive error messages

### Testing
- 116 tests covering core functionality
  - 36 unit tests in library modules
  - 27 format specification tests
  - 10 property-based round-trip tests
  - 21 error case tests
  - 10 shuffle integration tests
  - 12 documentation tests
- Unit tests for all major components
- Integration tests for format specifications and shuffle tool
- Property-based testing with proptest for round-trip verification
- Comprehensive error case coverage
- Doc tests for all public APIs

## [0.1.0] - Initial Development

### Added
- Initial project structure
- Basic format specification support
- Core formatting and parsing engines

[Unreleased]: https://github.com/freol35241/gullwing/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/freol35241/gullwing/releases/tag/v0.9.0
[0.1.0]: https://github.com/freol35241/gullwing/releases/tag/v0.1.0
