//! Error types for the gullwing library.

use thiserror::Error;

/// Errors that can occur when working with format specifications.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum Error {
    /// Invalid format specification syntax.
    #[error("invalid format specification: {0}")]
    InvalidFormatSpec(String),

    /// Unsupported type specifier.
    #[error("unsupported type specifier: {0}")]
    UnsupportedType(String),

    /// Parse error when matching a string against a format pattern.
    #[error("parse error: {0}")]
    ParseError(String),

    /// Type conversion error when converting parsed strings to typed values.
    #[error("type conversion error: {0}")]
    ConversionError(String),

    /// Regex compilation or matching error.
    #[error("regex error: {0}")]
    RegexError(String),

    /// Missing required field in format operation.
    #[error("missing field: {0}")]
    MissingField(String),

    /// Field name used is invalid.
    #[error("invalid field name: {0}")]
    InvalidFieldName(String),

    /// Width or precision value is invalid.
    #[error("invalid width or precision: {0}")]
    InvalidWidth(String),

    /// No match found when parsing.
    #[error("no match found")]
    NoMatch,
}

/// Result type alias for gullwing operations.
pub type Result<T> = std::result::Result<T, Error>;
