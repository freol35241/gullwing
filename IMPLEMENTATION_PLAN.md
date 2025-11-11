# Implementation Plan: Rust Format Specification Mini-Language Library

## Executive Summary

This document outlines a detailed plan for implementing a Rust library (`gullwing`) that provides runtime formatting and parsing capabilities based on Python's Format Specification Mini-Language. The library will enable developers to write ergonomic data transformation tools similar to the `shuffle` script from RISE-Maritime/porla.

## 1. Problem Domain Analysis

### 1.1 Current Use Case: The `shuffle` Script

The reference Python script demonstrates a common pattern:
- **Input**: Structured text lines (e.g., logs, CSV-like data)
- **Parse**: Extract named fields using format specifications (e.g., `{timestamp} {level} {message}`)
- **Transform**: Reformat/reorder extracted data (e.g., `{level}: {message}`)
- **Output**: Write transformed data to stdout

### 1.2 Python Format Specification Mini-Language

The format spec follows this grammar:
```
[[fill]align][sign][z][#][0][width][grouping][.precision][type]
```

**Key features:**
- **Alignment**: `<` (left), `>` (right), `^` (center), `=` (sign-aware)
- **Sign options**: `+`, `-`, ` ` (space)
- **Width and precision**: Numeric field sizing
- **Type specifiers**: `s` (string), `d` (decimal), `f` (float), `x/X` (hex), `b` (binary), `o` (octal), `e/E` (scientific), `%` (percentage)
- **Grouping**: `,` (comma), `_` (underscore)
- **Named and positional fields**: `{name:format}` or `{:format}`

### 1.3 Python `parse` Package Insights

The `parse` package converts format strings to regex patterns:
1. **Parser generation**: Format string → Regex with named groups
2. **Type conversion**: Extracted strings → Typed values (int, float, datetime, etc.)
3. **Custom types**: Extensible via pattern attachments
4. **Match modes**: `parse()` (exact), `search()` (partial), `findall()` (iterator)

**Key implementation details:**
- Uses `re.compile()` to generate matchers
- Supports repeated named fields (backreferences)
- Handles format spec details (width, precision, alignment) for parsing
- Provides type converters for common types

## 2. Rust Ecosystem Analysis

### 2.1 Existing Formatting Libraries

| Crate | Pros | Cons | Relevance |
|-------|------|------|-----------|
| **format_num** | - Implements Python-like format spec<br>- Good numeric formatting | - Numbers only (no strings)<br>- No parsing capability | Partial - formatting only |
| **strfmt** | - Named placeholder support<br>- HashMap-based API | - No positional fields (`{}`)<br>- Beta numeric support<br>- Maintenance mode | Partial - formatting only |
| **dynfmt** | - Runtime format strings<br>- Subset of std::fmt | - Limited format spec support<br>- No parsing | Partial - formatting only |
| **rt-format** | - Full std::fmt equivalent | - No fill character support<br>- No parsing | Partial - formatting only |

**Key Finding**: No existing crate provides **both** formatting and parsing with full format spec support.

### 2.2 Existing Parsing Libraries

| Crate | Pros | Cons | Relevance |
|-------|------|------|-----------|
| **sscanf** | - Inverse of format!()<br>- Macro-based<br>- Regex backend | - Compile-time patterns only<br>- Limited format spec | Low - compile-time only |
| **nom** | - Fast (3x faster than pest)<br>- Zero-copy capable<br>- Composable | - Steeper learning curve<br>- Verbose for simple cases | High - parsing engine |
| **pest** | - PEG grammar (readable)<br>- Good error messages | - Slower than nom<br>- Runtime overhead | Medium - parsing engine |
| **regex** | - Battle-tested<br>- Feature-complete | - Larger binary size<br>- Slower compile times | High - direct approach |

**Key Finding**: The Python `parse` package uses regex. For Rust, we have a choice between regex (simple, proven) and nom (faster, more complex).

## 3. Architecture Design

### 3.1 Core Components

```
┌─────────────────────────────────────────────────────┐
│                   Gullwing Library                   │
├─────────────────────────────────────────────────────┤
│                                                       │
│  ┌─────────────────────────────────────────────┐   │
│  │         Format Spec Parser                   │   │
│  │  (Parse format strings into AST)             │   │
│  └─────────────────────────────────────────────┘   │
│                     │                                │
│         ┌───────────┴───────────┐                   │
│         │                       │                   │
│  ┌──────▼──────┐       ┌───────▼────────┐          │
│  │  Formatter  │       │  Parser Builder │          │
│  │  Engine     │       │  (Regex Gen)    │          │
│  └─────────────┘       └────────────────┘          │
│         │                       │                   │
│         │                       │                   │
│  ┌──────▼──────┐       ┌───────▼────────┐          │
│  │  Format API │       │   Parse API     │          │
│  │  (Runtime   │       │   (Regex match  │          │
│  │   format)   │       │   + convert)    │          │
│  └─────────────┘       └────────────────┘          │
│                                                       │
└─────────────────────────────────────────────────────┘
```

### 3.2 Module Structure

```
gullwing/
├── src/
│   ├── lib.rs                 # Public API exports
│   ├── spec/
│   │   ├── mod.rs             # Format spec types
│   │   ├── parser.rs          # Format string → AST
│   │   └── types.rs           # Alignment, Sign, Type enums
│   ├── format/
│   │   ├── mod.rs             # Formatting API
│   │   ├── engine.rs          # Core formatting logic
│   │   └── writer.rs          # Output writing helpers
│   ├── parse/
│   │   ├── mod.rs             # Parsing API
│   │   ├── builder.rs         # Regex pattern builder
│   │   ├── matcher.rs         # Regex matching
│   │   └── convert.rs         # Type conversions
│   ├── types/
│   │   ├── mod.rs             # Value types
│   │   └── conversions.rs     # From/Into implementations
│   └── error.rs               # Error types
├── examples/
│   ├── shuffle.rs             # CLI tool (like porla/bin/shuffle)
│   ├── format_demo.rs         # Formatting examples
│   └── parse_demo.rs          # Parsing examples
└── tests/
    ├── format_spec.rs         # Format spec parsing tests
    ├── formatting.rs          # Formatting tests
    └── parsing.rs             # Parsing tests
```

## 4. Technical Choices & Justifications

### 4.1 Format Specification Parser: Hand-Written vs Parser Library

**Decision: Hand-written recursive descent parser**

**Rationale:**
- Format spec grammar is simple and well-defined
- No need for parser generator overhead (pest) or complexity (nom)
- Better error messages (we control the logic)
- Faster compilation, smaller binary
- Precedent: Python's `parse` package uses simple string manipulation

**Implementation approach:**
```rust
pub struct FormatSpec {
    pub fill: Option<char>,
    pub align: Option<Alignment>,
    pub sign: Option<Sign>,
    pub alternate: bool,
    pub zero_pad: bool,
    pub width: Option<usize>,
    pub grouping: Option<Grouping>,
    pub precision: Option<usize>,
    pub type_spec: Option<TypeSpec>,
}
```

### 4.2 Parsing Engine: Regex vs Nom

**Decision: Regex (with nom as future optimization path)**

**Rationale:**

**Pros of regex:**
- Python's `parse` package uses regex successfully
- Simpler implementation (1:1 translation from Python)
- Mature, battle-tested crate
- Proven approach for this use case
- Named capture groups map directly to format fields

**Pros of nom:**
- 3x faster than alternatives
- Zero-copy parsing possible
- Composable parsers for complex formats
- Better for binary formats

**Why regex wins for v1.0:**
- Simpler to implement and maintain
- Format specs are textual, not binary
- Pattern generation is straightforward (format spec → regex)
- Performance is adequate for most use cases (line-by-line processing)

**Future optimization:** Provide optional nom backend via feature flag if profiling shows regex is a bottleneck.

### 4.3 Formatting Engine: Custom vs Leveraging std::fmt

**Decision: Custom formatting engine with std::fmt inspiration**

**Rationale:**
- std::fmt requires compile-time format strings
- Need runtime format string support
- Format spec mini-language differs slightly from std::fmt
- Need precise control over width, alignment, padding
- Can reuse formatting logic from `format_num` as reference

**Implementation approach:**
- Parse format spec → Apply to value → Write to buffer
- Support common types: String, integers, floats, bool
- Extensible via trait for custom types

### 4.4 API Design Philosophy

**Decision: Ergonomic, type-safe, builder-pattern APIs**

**Key principles:**
1. **Compile-time safety where possible**: Use enums for alignments, signs, types
2. **Runtime flexibility**: Accept format strings as `&str` or `String`
3. **No panics**: Return `Result<T, Error>` for all fallible operations
4. **Zero-copy where possible**: Use `&str` views instead of allocations
5. **Rust idioms**: Implement `Display`, `Debug`, `From`/`Into`, iterators

## 5. API Design

### 5.1 Formatting API

```rust
use gullwing::{Formatter, Value};

// Simple formatting
let formatted = Formatter::new("{name:>10}").format(&[
    ("name", Value::Str("Alice"))
])?;

// Builder pattern for complex cases
let formatter = Formatter::builder()
    .pattern("{timestamp} {level:<5} {message}")
    .build()?;

let output = formatter.format_map(|field| {
    match field {
        "timestamp" => Some(Value::Str("2024-01-15")),
        "level" => Some(Value::Str("INFO")),
        "message" => Some(Value::Str("Hello")),
        _ => None,
    }
})?;

// Format with positional arguments
let formatted = Formatter::new("{0:05d} {1:.2f}").format_positional(&[
    Value::Int(42),
    Value::Float(3.14159),
])?; // "00042 3.14"
```

### 5.2 Parsing API

```rust
use gullwing::{Parser, ParseResult};

// Simple parsing
let parser = Parser::new("{name} is {age:d} years old")?;
let result = parser.parse("Alice is 30 years old")?;

assert_eq!(result.get("name"), Some(&Value::Str("Alice")));
assert_eq!(result.get("age"), Some(&Value::Int(30)));

// Search (partial match)
let result = parser.search("prefix: Alice is 30 years old :suffix")?;

// Find all matches
for result in parser.findall("Alice is 30 years old. Bob is 25 years old.")? {
    println!("{:?}", result);
}

// Typed extraction
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

let person: Person = parser.parse_into("Alice is 30 years old")?;
```

### 5.3 Shuffle-like CLI Tool API

```rust
use gullwing::{Parser, Formatter};
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_fmt = std::env::args().nth(1).expect("input format required");
    let output_fmt = std::env::args().nth(2).expect("output format required");

    let parser = Parser::new(&input_fmt)?;
    let formatter = Formatter::new(&output_fmt)?;

    let stdin = std::io::stdin();
    for line in BufReader::new(stdin.lock()).lines() {
        let line = line?;
        if let Some(parsed) = parser.parse(&line)? {
            let output = formatter.format_named(&parsed)?;
            println!("{}", output);
        }
    }

    Ok(())
}
```

### 5.4 Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid format specification: {0}")]
    InvalidFormatSpec(String),

    #[error("unsupported type specifier: {0}")]
    UnsupportedType(String),

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("type conversion error: {0}")]
    ConversionError(String),

    #[error("regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("missing field: {0}")]
    MissingField(String),
}
```

## 6. Implementation Phases

### Phase 1: Core Format Spec Parser (Week 1)
**Goal**: Parse format specifications into AST

**Deliverables:**
- [ ] `FormatSpec` struct with all fields
- [ ] Parser for format spec mini-language
- [ ] Comprehensive tests (100+ cases from Python spec)
- [ ] Error handling for invalid specs

**Validation:**
- All Python format spec examples parse correctly
- Invalid specs produce clear error messages

### Phase 2: Formatting Engine (Week 2-3)
**Goal**: Implement runtime formatting

**Deliverables:**
- [ ] `Formatter` struct and API
- [ ] Support for string, integer, float types
- [ ] Alignment, width, precision logic
- [ ] Sign handling, zero-padding
- [ ] Grouping (`,` and `_`)
- [ ] All type specifiers: `s`, `d`, `f`, `e`, `x`, `o`, `b`, `%`
- [ ] Named and positional field support

**Validation:**
- Matches Python's `format()` output for all test cases
- Performance: Format 1M strings in < 1 second

### Phase 3: Parsing Engine (Week 4-5)
**Goal**: Implement runtime parsing (inverse of formatting)

**Deliverables:**
- [ ] `Parser` struct and API
- [ ] Regex pattern generation from format strings
- [ ] Named capture group extraction
- [ ] Type conversion (string → int, float, etc.)
- [ ] Parse, search, findall methods
- [ ] Custom type support (extensibility)

**Validation:**
- Parses Python `parse` package test cases
- Round-trip: format → parse → original value

### Phase 4: Value System & Type Conversions (Week 6)
**Goal**: Ergonomic value handling

**Deliverables:**
- [ ] `Value` enum (String, Int, Float, Bool, etc.)
- [ ] `From`/`Into` implementations for common types
- [ ] Trait for custom value types
- [ ] ParseResult with ergonomic accessors

**Validation:**
- Easy conversion between Rust types and Value
- Type-safe extraction from parse results

### Phase 5: CLI Tool & Examples (Week 7)
**Goal**: Demonstrate library with shuffle tool

**Deliverables:**
- [ ] `shuffle` CLI binary (like porla version)
- [ ] Log transformation examples
- [ ] CSV reformatting examples
- [ ] Documentation with use cases

**Validation:**
- Can replicate porla/shuffle functionality
- Handles 100k lines/second

### Phase 6: Polish & Optimization (Week 8)
**Goal**: Production-ready release

**Deliverables:**
- [ ] Comprehensive documentation
- [ ] Performance benchmarks
- [ ] Optional nom backend (feature flag)
- [ ] Property-based testing (proptest)
- [ ] CI/CD setup
- [ ] Crates.io release prep

**Validation:**
- 90%+ code coverage
- Benchmarks vs Python parse package
- Clean cargo clippy, cargo audit

## 7. Performance Targets

| Operation | Target | Rationale |
|-----------|--------|-----------|
| Format spec parsing | < 1µs | One-time cost per pattern |
| Single format operation | < 100ns | Comparable to std::fmt |
| Single parse operation | < 10µs | Regex overhead + conversions |
| Shuffle 1M lines | < 10s | Real-world log processing |
| Binary size overhead | < 500KB | Including regex dependency |

## 8. Dependencies

```toml
[dependencies]
regex = "1.10"           # Parsing via regex
thiserror = "1.0"        # Error handling
lazy_static = "1.4"      # Cached regex patterns

[dev-dependencies]
proptest = "1.4"         # Property-based testing
criterion = "0.5"        # Benchmarking
pretty_assertions = "1.4" # Better test output

[features]
default = ["std"]
std = []
nom-backend = ["nom"]    # Optional nom parsing backend
```

## 9. Testing Strategy

### 9.1 Unit Tests
- **Format spec parser**: Every spec variant, error cases
- **Formatter**: Each type specifier, alignment, padding
- **Parser**: Regex generation, field extraction, type conversion

### 9.2 Integration Tests
- **Round-trip**: Format → Parse → Original value
- **Python compatibility**: Port Python `parse` test suite
- **Shuffle tool**: End-to-end CLI tests

### 9.3 Property-Based Tests
```rust
proptest! {
    #[test]
    fn roundtrip_int(n: i64) {
        let formatted = format_value("{:d}", n)?;
        let parsed = parse_value("{:d}", &formatted)?;
        assert_eq!(Value::Int(n), parsed);
    }
}
```

### 9.4 Benchmarks
- Compare against Python's `parse` package
- Measure format/parse operations
- Profile regex compilation overhead

## 10. Documentation Plan

### 10.1 API Documentation
- Rustdoc for all public items
- Examples in doc comments
- Links to Python format spec

### 10.2 Guides
- **Getting Started**: Basic format/parse usage
- **Format Specification**: Complete spec reference
- **CLI Tools**: Building shuffle-like tools
- **Custom Types**: Extending with user types
- **Performance**: Optimization tips

### 10.3 Examples
- Log file transformation
- CSV/TSV reformatting
- Data extraction pipelines
- Config file parsing

## 11. Future Enhancements (Post-v1.0)

1. **Advanced Types**
   - DateTime parsing (like Python parse)
   - Custom pattern matchers
   - Complex nested structures

2. **Performance**
   - nom backend for high-throughput scenarios
   - Compiled pattern caching
   - SIMD optimizations for formatting

3. **Features**
   - Compiled format/parse (proc macro)
   - Async support for streams
   - Serde integration

4. **Tools**
   - Interactive format playground (WASM)
   - Format string linter/validator
   - Migration tool from Python parse

## 12. Risk Analysis

| Risk | Impact | Mitigation |
|------|--------|------------|
| Format spec complexity | High | Start with subset, expand iteratively |
| Python compatibility gaps | Medium | Extensive test porting, clear docs on differences |
| Regex performance | Medium | Profile early, nom fallback |
| API ergonomics | High | User testing, examples-first design |
| Type system mismatch | Medium | Value enum abstraction, conversion traits |

## 13. Success Criteria

**Version 1.0 is successful if:**
1. ✅ Supports 90%+ of Python format spec mini-language
2. ✅ Can replicate `porla/shuffle` functionality in Rust
3. ✅ API is ergonomic (< 10 lines for common use cases)
4. ✅ Performance within 2x of Python (faster expected)
5. ✅ Comprehensive docs + examples
6. ✅ 80%+ code coverage
7. ✅ Zero unsafe code (unless profiling demands it)

## 14. Conclusion

This implementation plan provides a roadmap for building a comprehensive Rust library for runtime format string handling. By learning from Python's `parse` package while leveraging Rust's type system and performance characteristics, `gullwing` will enable developers to write ergonomic data transformation tools with confidence.

The phased approach allows for iterative development, early validation, and course correction. The focus on API ergonomics, comprehensive testing, and clear documentation will ensure the library is both powerful and accessible.

**Next Steps:**
1. Review and approve this plan
2. Set up project structure (Cargo.toml, CI/CD)
3. Begin Phase 1: Format Spec Parser implementation
