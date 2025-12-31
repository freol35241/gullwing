# CLAUDE.md - gullwing

This file provides context for AI agents (like Claude) working on this codebase.

## Project Overview

**gullwing** is a Rust library that implements Python's Format Specification Mini-Language for runtime string formatting and parsing. It enables bidirectional data transformation: format values to strings AND parse strings to values.

### Key Capabilities
- Runtime format strings (vs Rust's compile-time `format!` macro)
- Parse structured data from strings using format patterns
- Full Python format spec compatibility: alignment, padding, precision, type specifiers, etc.

### Inspiration
- Python's `str.format()` and Format Specification Mini-Language
- Python's [`parse`](https://github.com/r1chardj0n3s/parse) package for reverse formatting

## Quick Reference

### Build & Test Commands
```bash
cargo build                    # Build library
cargo test                     # Run all tests (116+ tests)
cargo test --doc               # Run doc tests only
cargo clippy --all-targets     # Lint (must pass with no warnings)
cargo fmt --check              # Check formatting
cargo doc --no-deps            # Build documentation
cargo bench                    # Run benchmarks
cargo build --example shuffle  # Build example CLI tool
```

### Project Structure
```
gullwing/
├── src/
│   ├── lib.rs              # Public API exports, crate docs
│   ├── error.rs            # Error types (9 variants)
│   ├── spec/               # Format specification parsing
│   │   ├── mod.rs          # Module exports
│   │   ├── parser.rs       # Hand-written recursive descent parser
│   │   └── types.rs        # Alignment, Sign, Grouping, TypeSpec enums
│   ├── types/
│   │   └── mod.rs          # Value enum (Str, Int, UInt, Float, Bool, Char)
│   ├── format/             # Formatting engine
│   │   ├── mod.rs          # Module exports
│   │   ├── engine.rs       # Formatter struct, format string parsing
│   │   └── writer.rs       # Type-specific formatting functions
│   └── parse/              # Parsing engine
│       ├── mod.rs          # Module exports
│       ├── matcher.rs      # Parser struct, regex matching
│       └── builder.rs      # Regex pattern generation from format strings
├── tests/                  # Integration tests
│   ├── format_spec.rs      # Format spec parsing tests
│   ├── formatting.rs       # Formatting operation tests
│   ├── roundtrip.rs        # Property-based roundtrip tests (proptest)
│   ├── error_cases.rs      # Error handling tests
│   └── shuffle_integration.rs  # End-to-end CLI tests
├── benches/                # Criterion benchmarks
├── examples/
│   └── shuffle.rs          # CLI tool for text transformation
└── docs/
    ├── CHANGELOG.md
    ├── CONTRIBUTING.md
    ├── SECURITY.md
    └── IMPLEMENTATION_PLAN.md
```

## Architecture

### Data Flow

**Formatting Pipeline:**
```
Format String → parse_format_string() → Vec<Field>
                                            ↓
Values (HashMap) → format_map() → for each field:
                                    → format_value() → type-specific formatter
                                    → apply_alignment()
                                            ↓
                                      Formatted String
```

**Parsing Pipeline:**
```
Format String → build_regex_pattern() → Regex + CaptureInfo
                                            ↓
Input Text → Parser::parse() → regex.captures()
                                    ↓
                              convert_value() for each capture
                                    ↓
                              ParseResult (HashMap<String, Value>)
```

### Key Design Decisions

1. **Hand-written spec parser** (`spec/parser.rs`): Not using a parser combinator library for simplicity and performance (~26-128ns per parse)

2. **Regex-based text parsing**: Uses `regex` crate for efficient pattern matching. Patterns are compiled once at `Parser::new()` time.

3. **Value enum**: Runtime type system since format strings are determined at runtime. Supports common Rust types with `From` implementations.

4. **Zero unsafe code**: Uses `#![forbid(unsafe_code)]` - all operations are safe Rust.

## Public API Summary

### Primary Types
- `Formatter` - Format values to strings
  - `new(pattern: &str) -> Result<Self>`
  - `format_map(&HashMap<String, Value>) -> Result<String>`
  - `format_fn(|&str| -> Option<Value>) -> Result<String>`
  - `format_positional(&[Value]) -> Result<String>`

- `Parser` - Parse strings to values
  - `new(pattern: &str) -> Result<Self>`
  - `parse(text: &str) -> Result<Option<ParseResult>>`
  - `search(text: &str) -> Result<Option<ParseResult>>`
  - `findall(text: &str) -> Result<impl Iterator<Item=ParseResult>>`

- `Value` - Universal value type
  - Variants: `Str`, `Int`, `UInt`, `Float`, `Bool`, `Char`
  - Accessors: `as_str()`, `as_int()`, `to_float()`, etc.

- `FormatSpec` - Parsed format specification
- `Error` - Error enum with 9 variants

## Code Conventions

### Error Handling
- All fallible operations return `Result<T, Error>`
- Use `?` operator for propagation
- Never use `unwrap()` without a guard check
- Map external errors to library error types

### Testing
- Unit tests in module files (`#[cfg(test)] mod tests`)
- Integration tests in `tests/` directory
- Property-based tests with `proptest` for roundtrip verification
- Doc examples on all public APIs (must compile and pass)

### Documentation
- All public items must have `///` doc comments
- Include `# Examples` section for complex APIs
- Use `#![warn(missing_docs)]` enforcement

## Common Development Tasks

### Adding a new type specifier
1. Add variant to `TypeSpec` enum in `src/spec/types.rs`
2. Update `TypeSpec::from_char()` and `to_char()`
3. Add formatting logic in `src/format/writer.rs`
4. Add regex pattern in `src/parse/builder.rs`
5. Add conversion logic in `convert_value()` in `src/parse/matcher.rs`
6. Add tests in `tests/format_spec.rs` and `tests/formatting.rs`

### Adding a new error type
1. Add variant to `Error` enum in `src/error.rs`
2. Add appropriate `#[error("...")]` message
3. Use the new error in relevant code paths
4. Add test in `tests/error_cases.rs`

### Running benchmarks
```bash
cargo bench                           # Run all benchmarks
cargo bench -- format_spec            # Run specific benchmark group
cargo bench -- --save-baseline before # Save baseline
cargo bench -- --baseline before      # Compare to baseline
```

## Dependencies

### Runtime
- `regex` (1.10) - Pattern matching for parsing
- `thiserror` (1.0) - Error derive macros
- `lazy_static` (1.4) - Lazy static initialization for cached patterns

### Development
- `proptest` (1.4) - Property-based testing
- `criterion` (0.5) - Benchmarking
- `pretty_assertions` (1.4) - Better test diffs

## CI/CD

GitHub Actions workflow (`.github/workflows/ci.yml`):
- Tests on Ubuntu, Windows, macOS
- Tests on Rust stable and beta
- MSRV check (Rust 1.70)
- Clippy linting (all targets, deny warnings)
- rustfmt check
- Documentation build

## Release Checklist

The release workflow (`.github/workflows/release.yml`) automates publishing to crates.io.

### Manual Steps
1. [ ] Update version in `Cargo.toml`
2. [ ] Update `CHANGELOG.md` with release notes
3. [ ] Run full test suite: `cargo test --all-features`
4. [ ] Run clippy: `cargo clippy --all-targets -- -D warnings`
5. [ ] Build docs: `cargo doc --no-deps`
6. [ ] Verify README examples work
7. [ ] Commit version bump: `git commit -am "chore: release v0.x.0"`
8. [ ] Create git tag: `git tag v0.x.0`
9. [ ] Push tag: `git push origin v0.x.0`
10. [ ] Create GitHub Release from the tag

### Automated Steps (triggered by GitHub Release)
- Full test suite runs
- Version tag is verified against Cargo.toml
- Crate is published to crates.io

### Required Setup
Add `CARGO_REGISTRY_TOKEN` secret to repository settings:
1. Go to Settings > Secrets and variables > Actions
2. Add new repository secret named `CARGO_REGISTRY_TOKEN`
3. Value: Your crates.io API token (from https://crates.io/settings/tokens)

## Known Limitations

- No nested field access (`{obj.field}` not supported)
- Locale-aware formatting (`n` type) falls back to decimal
- Some edge cases in float formatting may differ from Python
- No `no_std` support (despite feature flag presence)

## Getting Help

- README.md - Quick start and examples
- API docs: `cargo doc --open`
- CONTRIBUTING.md - Development guidelines
- GitHub Issues - Bug reports and feature requests
