//! # Gullwing
//!
//! Runtime formatting and parsing with Python's Format Specification Mini-Language.
//!
//! This library provides two main capabilities:
//! - **Formatting**: Format values using runtime format strings (like Python's `format()`)
//! - **Parsing**: Extract structured data from strings using format patterns (like Python's `parse` package)
//!
//! ## Format Specification Mini-Language
//!
//! The format specification follows Python's syntax:
//!
//! ```text
//! [[fill]align][sign][z][#][0][width][grouping][.precision][type]
//! ```
//!
//! See the [Python documentation](https://docs.python.org/3/library/string.html#formatspec)
//! for complete details.
//!
//! ## Quick Start
//!
//! ### Formatting
//!
//! ```rust,ignore
//! use gullwing::{Formatter, Value};
//!
//! let formatter = Formatter::new("{name:>10} {value:05d}")?;
//! let output = formatter.format(&[
//!     ("name", Value::Str("Alice")),
//!     ("value", Value::Int(42))
//! ])?;
//! // Output: "     Alice 00042"
//! ```
//!
//! ### Parsing
//!
//! ```rust,ignore
//! use gullwing::Parser;
//!
//! let parser = Parser::new("{name} is {age:d} years old")?;
//! let result = parser.parse("Alice is 30 years old")?;
//!
//! assert_eq!(result.get("name"), Some("Alice"));
//! assert_eq!(result.get("age"), Some(30));
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub mod error;
pub mod format;
pub mod parse;
pub mod spec;
pub mod types;

// Re-export commonly used types
pub use error::{Error, Result};
pub use format::Formatter;
pub use parse::{ParseResult, Parser};
pub use spec::{Alignment, FormatSpec, Grouping, Sign, TypeSpec};
pub use types::Value;
