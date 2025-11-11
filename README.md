# gullwing

[![Crates.io](https://img.shields.io/crates/v/gullwing.svg)](https://crates.io/crates/gullwing)
[![Documentation](https://docs.rs/gullwing/badge.svg)](https://docs.rs/gullwing)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Runtime formatting and parsing with Python's [Format Specification Mini-Language](https://docs.python.org/3/library/string.html#formatspec).

gullwing brings Python-style string formatting and parsing to Rust, enabling you to:
- **Format** values at runtime using format strings (like Python's `format()` and `str.format()`)
- **Parse** structured data from strings (like Python's [`parse`](https://github.com/r1chardj0n3s/parse) package)

## Features

- 🎯 **Runtime Format Strings**: Use format strings determined at runtime, not just compile-time
- 🔄 **Bidirectional**: Both format values to strings AND parse strings to values
- 📐 **Full Format Spec Support**: Alignment, padding, precision, type specifiers, and more
- 🦀 **Pure Rust**: Zero unsafe code, comprehensive error handling
- 🚀 **Fast**: Regex-based parsing, efficient formatting
- 📚 **Well Documented**: Extensive examples and API documentation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gullwing = "0.1"
```

## Quick Start

### Formatting

```rust
use gullwing::{Formatter, Value};
use std::collections::HashMap;

let formatter = Formatter::new("{name:>10} scored {score:05d} points")?;

let mut values = HashMap::new();
values.insert("name".to_string(), Value::from("Alice"));
values.insert("score".to_string(), Value::from(42));

let result = formatter.format_map(&values)?;
assert_eq!(result, "     Alice scored 00042 points");
```

### Parsing

```rust
use gullwing::Parser;

let parser = Parser::new("{name} is {age:d} years old")?;
let result = parser.parse("Alice is 30 years old")?.unwrap();

assert_eq!(result.get("name").unwrap().as_str(), Some("Alice"));
assert_eq!(result.get("age").unwrap().as_int(), Some(30));
```

## Format Specification Mini-Language

gullwing implements Python's format specification syntax:

```
[[fill]align][sign][z][#][0][width][grouping][.precision][type]
```

### Examples

```rust
use gullwing::{Formatter, Value};

// Right-align in 10 characters
let f = Formatter::new("{:>10}")?;
// ">     hello"

// Zero-pad integers to 5 digits
let f = Formatter::new("{:05d}")?;
// "00042"

// Format float with 2 decimal places
let f = Formatter::new("{:.2f}")?;
// "3.14"

// Hexadecimal with 0x prefix
let f = Formatter::new("{:#x}")?;
// "0xff"

// Thousands separator
let f = Formatter::new("{:,d}")?;
// "1,000,000"

// Center-align with custom fill
let f = Formatter::new("{:*^20}")?;
// "*******hello********"
```

### Supported Type Specifiers

| Type | Description | Example Input | Parsed As |
|------|-------------|---------------|-----------|
| `s` | String (default) | `"hello"` | String |
| `d` | Decimal integer | `"42"`, `"-17"` | i64 |
| `b` | Binary integer | `"1010"`, `"0b1010"` | i64 |
| `o` | Octal integer | `"755"`, `"0o755"` | i64 |
| `x`, `X` | Hexadecimal | `"ff"`, `"0xFF"` | i64 |
| `f`, `F` | Fixed-point float | `"3.14"`, `"-2.5"` | f64 |
| `e`, `E` | Scientific notation | `"1.5e10"` | f64 |
| `g`, `G` | General float | `"3.14"`, `"1e5"` | f64 |
| `%` | Percentage | `"50%"` | f64 (0.50) |
| `c` | Character | `"A"` | char |

## Use Cases

### Log File Transformation

```rust
use gullwing::{Parser, Formatter};

let parser = Parser::new("{timestamp} {level:>5} {message}")?;
let formatter = Formatter::new("[{level}] {message}")?;

let line = "2024-01-15T10:30:00 INFO  Server started";
if let Some(parsed) = parser.parse(line)? {
    let output = formatter.format_map(parsed.values())?;
    println!("{}", output);
    // Output: [INFO] Server started
}
```

### CSV/Data Reformatting

```rust
use gullwing::{Parser, Formatter};

let parser = Parser::new("{id:d},{name},{score:f}")?;
let formatter = Formatter::new("ID: {id:03d} | Name: {name:<20} | Score: {score:. 1f}")?;

let csv_line = "5,Alice,95.7";
if let Some(parsed) = parser.parse(csv_line)? {
    let output = formatter.format_map(parsed.values())?;
    println!("{}", output);
    // Output: ID: 005 | Name: Alice                | Score: 95.7
}
```

### The Shuffle Tool

gullwing includes a `shuffle` example that demonstrates text transformation capabilities:

```bash
# Build the example
cargo build --example shuffle --release

# Use it to transform log files
echo "2024-01-15 INFO Hello World" | \
  target/release/examples/shuffle "{date} {level} {message}" "{level}: {message}"
# Output: INFO: Hello World

# Extract and reformat CSV data
echo "Alice,30,Engineer" | \
  target/release/examples/shuffle "{name},{age:d},{job}" "{name} ({age}) - {job}"
# Output: Alice (30) - Engineer
```

## Advanced Features

### Search and FindAll

```rust
use gullwing::Parser;

let parser = Parser::new("{number:d}")?;

// Search finds the first match
let result = parser.search("The answer is 42!")?.unwrap();
assert_eq!(result.get("number").unwrap().as_int(), Some(42));

// FindAll finds all matches
let results: Vec<_> = parser.findall("Numbers: 1, 2, 3")?.collect();
assert_eq!(results.len(), 3);
```

### Functional Formatting

```rust
use gullwing::{Formatter, Value};

let formatter = Formatter::new("x={x}, y={y}, sum={z}")?;

let result = formatter.format_fn(|field| {
    match field {
        "x" => Some(Value::from(10)),
        "y" => Some(Value::from(20)),
        "z" => Some(Value::from(30)),
        _ => None,
    }
})?;

assert_eq!(result, "x=10, y=20, sum=30");
```

### Positional Arguments

```rust
use gullwing::{Formatter, Value};

let formatter = Formatter::new("{0} + {1} = {2}")?;
let values = vec![Value::from(2), Value::from(3), Value::from(5)];
let result = formatter.format_positional(&values)?;
assert_eq!(result, "2 + 3 = 5");
```

## Comparison with Python

### Formatting

**Python:**
```python
"{name:>10} {value:05d}".format(name="Alice", value=42)
```

**Rust with gullwing:**
```rust
let formatter = Formatter::new("{name:>10} {value:05d}")?;
let mut values = HashMap::new();
values.insert("name".to_string(), Value::from("Alice"));
values.insert("value".to_string(), Value::from(42));
formatter.format_map(&values)?
```

### Parsing

**Python:**
```python
import parse
result = parse.parse("{name} is {age:d} years old", "Alice is 30 years old")
print(result["name"], result["age"])
```

**Rust with gullwing:**
```rust
let parser = Parser::new("{name} is {age:d} years old")?;
let result = parser.parse("Alice is 30 years old")?.unwrap();
println!("{} {}",
    result.get("name").unwrap().as_str().unwrap(),
    result.get("age").unwrap().as_int().unwrap());
```

## Architecture

gullwing is built with:
- **Hand-written format spec parser** for fast, accurate parsing of format specifications
- **Regex-based text parsing** for efficient pattern matching (inspired by Python's parse package)
- **Type-safe value system** with `Value` enum for runtime value handling
- **Zero-copy operations** where possible for performance

## Performance

Benchmarks run on an Intel x86_64 processor using criterion.rs. All operations are measured at runtime (no compile-time optimizations):

### Format Specification Parsing

| Operation | Time |
|-----------|------|
| Simple spec (`>10`) | ~26 ns |
| Complex spec (`0<+#20,.2f`) | ~63 ns |
| All features (`*>+z#030_,.6e`) | ~128 ns |

### Formatting Operations

| Operation | Time |
|-----------|------|
| Simple string | ~104 ns |
| Integer | ~151 ns |
| Integer with grouping | ~337 ns |
| Float with precision | ~224 ns |
| Aligned/padded string | ~175 ns |
| Hex with prefix | ~160 ns |
| Complex pattern (3 fields) | ~895 ns |
| Multiple fields (10) | ~1.22 µs |

### Parsing Operations

| Operation | Time |
|-----------|------|
| Simple pattern | ~38.5 µs |
| Integer | ~63.6 µs |
| Float | ~151.7 µs |
| Integer with grouping | ~32.0 µs |
| Multiple fields (3) | ~197.6 µs |
| Complex pattern | ~279 µs |
| Search (first match) | ~489 ns |
| FindAll (4 matches) | ~2.74 µs |
| Pattern creation | ~61.5 µs |

**Key Takeaways:**
- Format specification parsing is **extremely fast** (sub-microsecond)
- Formatting operations are **comparable to `std::fmt`** (nanoseconds)
- Parsing includes regex compilation and matching, measured in microseconds
- Pattern creation is one-time cost; reuse Parser instances for best performance

Run benchmarks yourself:
```bash
cargo bench
```

## Limitations

- No support for nested field access (e.g., `{obj.field}`)
- Locale-aware formatting (`n` type) falls back to default formatting
- Some edge cases in floating-point formatting may differ slightly from Python

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

- Inspired by Python's [Format Specification Mini-Language](https://docs.python.org/3/library/string.html#formatspec)
- Parse functionality inspired by Richard Jones' [`parse`](https://github.com/r1chardj0n3s/parse) package
- Created to enable ergonomic data transformation tools in Rust

## See Also

- [Python Format Specification](https://docs.python.org/3/library/string.html#formatspec)
- [Python parse package](https://github.com/r1chardj0n3s/parse)
- [Implementation Plan](IMPLEMENTATION_PLAN.md) - detailed design decisions and architecture
