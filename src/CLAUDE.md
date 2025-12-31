# src/CLAUDE.md - Source Code Guide

This document provides detailed guidance for working with the gullwing source code.

## Module Dependency Graph

```
lib.rs (exports)
    ├── error.rs (Error, Result)
    ├── types/mod.rs (Value)
    ├── spec/
    │   ├── mod.rs (exports)
    │   ├── types.rs (Alignment, Sign, Grouping, TypeSpec)
    │   └── parser.rs (FormatSpec, SpecParser) → uses types.rs, error.rs
    ├── format/
    │   ├── mod.rs (exports)
    │   ├── engine.rs (Formatter) → uses spec/, types/, error.rs
    │   └── writer.rs (format functions) → uses spec/, types/, error.rs
    └── parse/
        ├── mod.rs (exports)
        ├── builder.rs (regex generation) → uses spec/, error.rs
        └── matcher.rs (Parser, ParseResult) → uses builder, spec/, types/, error.rs
```

## Module Details

### `error.rs` - Error Types

**Error Enum Variants:**
| Variant | When Used | Example |
|---------|-----------|---------|
| `InvalidFormatSpec` | Malformed format spec syntax | `{:xyz}` |
| `UnsupportedType` | Unknown type specifier | rarely used |
| `ParseError` | General parse failures | malformed input |
| `ConversionError` | Type conversion fails | `"abc".to_int()` |
| `RegexError` | Regex compilation fails | invalid pattern |
| `MissingField` | Required field not provided | `format_map` without field |
| `InvalidFieldName` | Bad field name chars | `{foo!bar}` |
| `InvalidWidth` | Bad width/precision | non-numeric width |
| `NoMatch` | No regex match found | used internally |

### `spec/types.rs` - Format Spec Components

**Alignment:**
- `Left` (`<`) - Left-align
- `Right` (`>`) - Right-align
- `Center` (`^`) - Center
- `AfterSign` (`=`) - Pad after sign

**Sign:**
- `Plus` (`+`) - Always show sign
- `Minus` (`-`) - Negative only (default)
- `Space` (` `) - Space for positive

**Grouping:**
- `Comma` (`,`) - Thousands separator with comma
- `Underscore` (`_`) - Thousands separator with underscore

**TypeSpec:**
- String: `String` (`s`)
- Integer: `Decimal` (`d`), `Binary` (`b`), `Octal` (`o`), `HexLower` (`x`), `HexUpper` (`X`), `Number` (`n`)
- Float: `FixedLower` (`f`), `FixedUpper` (`F`), `ExponentLower` (`e`), `ExponentUpper` (`E`), `GeneralLower` (`g`), `GeneralUpper` (`G`), `Percentage` (`%`)
- Other: `Character` (`c`)

### `spec/parser.rs` - Format Spec Parser

**FormatSpec Fields:**
```rust
pub struct FormatSpec {
    pub fill: Option<char>,      // Fill character
    pub align: Option<Alignment>, // Alignment mode
    pub sign: Option<Sign>,      // Sign handling
    pub zero_flag: bool,         // 'z' flag (coerce -0.0)
    pub alternate: bool,         // '#' flag (0x prefix, etc.)
    pub zero_pad: bool,          // '0' flag (zero padding)
    pub width: Option<usize>,    // Minimum width
    pub grouping: Option<Grouping>, // Grouping separator
    pub precision: Option<usize>, // Decimal places / string max
    pub type_spec: Option<TypeSpec>, // Type specifier
}
```

**Parser Strategy:**
The `SpecParser` is a hand-written recursive descent parser that processes the spec string character by character:
1. Try to parse fill+align (lookahead for align char)
2. Parse sign (`+`, `-`, ` `)
3. Parse flags (`z`, `#`, `0`)
4. Parse width (digits)
5. Parse grouping (`,`, `_`)
6. Parse precision (`.` + digits)
7. Parse type specifier (single char)

### `types/mod.rs` - Value Type

```rust
pub enum Value {
    Str(String),
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    Char(char),
}
```

**Accessor Methods:**
- `as_*()` - Returns `Option<T>` for direct access (no conversion)
- `to_*()` - Returns `Result<T>` with conversion attempt

**From Implementations:**
- `&str`, `String` → `Str`
- `i32`, `i64` → `Int`
- `u32`, `u64`, `usize` → `UInt`
- `f32`, `f64` → `Float`
- `bool` → `Bool`
- `char` → `Char`

### `format/engine.rs` - Formatter

**Internal Field Struct:**
```rust
struct Field {
    prefix: String,        // Literal text before this field
    name: Option<String>,  // Named field (None for positional)
    index: Option<usize>,  // Positional index
    spec: FormatSpec,      // Format specification
}
```

**Format String Parsing:**
- `{name}` → Named field
- `{0}` → Positional by index
- `{}` → Auto-numbered positional
- `{{` → Escaped literal `{`
- `}}` → Escaped literal `}`

### `format/writer.rs` - Type-Specific Formatters

**Key Functions:**
| Function | Handles |
|----------|---------|
| `format_string` | String values, precision truncation |
| `format_decimal` | Integer as decimal |
| `format_binary` | Integer as binary (with optional 0b) |
| `format_octal` | Integer as octal (with optional 0o) |
| `format_hex` | Integer as hex (with optional 0x) |
| `format_fixed` | Float with fixed decimals |
| `format_exponent` | Float in scientific notation |
| `format_general` | Float choosing f or e |
| `format_percentage` | Float * 100 with % |
| `format_character` | Int as Unicode char |

**Helper Functions:**
- `apply_grouping()` - Insert comma/underscore separators
- `add_sign()` - Add +/- sign prefix
- `apply_zero_padding()` - Zero-pad numeric values

### `parse/builder.rs` - Regex Pattern Generation

**`build_regex_pattern(format_str: &str)`**
Converts a format string to a regex pattern:
- `{name}` → `(?P<name>.+?)`
- `{name:d}` → `(?P<name>[-+]?\d+)`
- `{name:f}` → `(?P<name>[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?)`
- Literal text → escaped for regex

**CaptureInfo:**
Tracks each field's name and TypeSpec for value conversion after matching.

### `parse/matcher.rs` - Parser and ParseResult

**Parser Fields:**
```rust
pub struct Parser {
    pattern: Regex,            // For search/findall
    anchored_pattern: Regex,   // For exact parse (^...$)
    captures: Vec<CaptureInfo>, // Field metadata
}
```

**ParseResult:**
```rust
pub struct ParseResult {
    values: HashMap<String, Value>,
    text: String,  // Matched text
}
```

**Value Conversion (`convert_value`):**
- Handles base prefixes (0x, 0b, 0o)
- Removes grouping separators before parsing
- Converts percentage (removes %, divides by 100)

## Testing Patterns

### Unit Test Location
Tests are in `#[cfg(test)] mod tests` blocks within each module.

### Integration Test Files
- `format_spec.rs` - Comprehensive spec parsing tests
- `formatting.rs` - Formatting output verification
- `roundtrip.rs` - proptest: format → parse → compare
- `error_cases.rs` - Error condition verification
- `shuffle_integration.rs` - CLI tool end-to-end tests

### Property-Based Testing (proptest)
```rust
proptest! {
    #[test]
    fn roundtrip_decimal_int(n in -1000000i64..1000000i64) {
        // Format value, parse it back, verify equality
    }
}
```

## Performance Notes

### Hot Paths
1. **`FormatSpec::parse()`** - Called for each field, keep fast (~26ns simple, ~128ns complex)
2. **`format_value()`** - Called for each value, type-switch is efficient
3. **Regex matching** - Pattern compiled once at `Parser::new()`, reused

### Caching
- `lazy_static!` used for commonly-needed patterns
- Formatter/Parser instances should be reused, not recreated

### Allocations
- Format output builds a single `String`
- Parsing returns `HashMap<String, Value>`
- Consider pre-sizing for known field counts

## Common Pitfalls

1. **String slicing** - Always use character iterators, not byte indices
2. **UTF-8 width** - Alignment width is in chars, not bytes
3. **Numeric edge cases** - Handle negative zero, infinity, NaN for floats
4. **Regex escaping** - Literal text in patterns must be `regex::escape()`d
