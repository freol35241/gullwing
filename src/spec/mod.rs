//! Format specification parsing and types.
//!
//! This module provides types and parsing for Python's Format Specification Mini-Language.
//!
//! # Format Specification Syntax
//!
//! ```text
//! [[fill]align][sign][z][#][0][width][grouping][.precision][type]
//! ```
//!
//! See: <https://docs.python.org/3/library/string.html#formatspec>

pub mod parser;
pub mod types;

pub use parser::FormatSpec;
pub use types::{Alignment, Grouping, Sign, TypeSpec};
