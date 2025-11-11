//! Value types for formatting and parsing.

use crate::error::{Error, Result};
use std::fmt;

/// A value that can be formatted or parsed.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// String value
    Str(String),
    /// Signed integer value
    Int(i64),
    /// Unsigned integer value
    UInt(u64),
    /// Floating point value
    Float(f64),
    /// Boolean value
    Bool(bool),
    /// Character value
    Char(char),
}

impl Value {
    /// Get this value as a string slice, if possible.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Get this value as an integer, if possible.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            Value::UInt(u) if *u <= i64::MAX as u64 => Some(*u as i64),
            Value::Bool(true) => Some(1),
            Value::Bool(false) => Some(0),
            _ => None,
        }
    }

    /// Get this value as an unsigned integer, if possible.
    pub fn as_uint(&self) -> Option<u64> {
        match self {
            Value::UInt(u) => Some(*u),
            Value::Int(i) if *i >= 0 => Some(*i as u64),
            Value::Bool(true) => Some(1),
            Value::Bool(false) => Some(0),
            _ => None,
        }
    }

    /// Get this value as a float, if possible.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(i) => Some(*i as f64),
            Value::UInt(u) => Some(*u as f64),
            _ => None,
        }
    }

    /// Get this value as a boolean, if possible.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Get this value as a character, if possible.
    pub fn as_char(&self) -> Option<char> {
        match self {
            Value::Char(c) => Some(*c),
            Value::Str(s) if s.len() == 1 => s.chars().next(),
            _ => None,
        }
    }

    /// Try to convert this value to an integer for formatting.
    pub fn to_int(&self) -> Result<i64> {
        self.as_int()
            .ok_or_else(|| Error::ConversionError(format!("cannot convert {:?} to int", self)))
    }

    /// Try to convert this value to an unsigned integer for formatting.
    pub fn to_uint(&self) -> Result<u64> {
        self.as_uint()
            .ok_or_else(|| Error::ConversionError(format!("cannot convert {:?} to uint", self)))
    }

    /// Try to convert this value to a float for formatting.
    pub fn to_float(&self) -> Result<f64> {
        self.as_float()
            .ok_or_else(|| Error::ConversionError(format!("cannot convert {:?} to float", self)))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
            Value::UInt(u) => write!(f, "{}", u),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Char(c) => write!(f, "{}", c),
        }
    }
}

// Implement From for common types
impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Str(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Str(s.to_string())
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Int(i as i64)
    }
}

impl From<u64> for Value {
    fn from(u: u64) -> Self {
        Value::UInt(u)
    }
}

impl From<u32> for Value {
    fn from(u: u32) -> Self {
        Value::UInt(u as u64)
    }
}

impl From<usize> for Value {
    fn from(u: usize) -> Self {
        Value::UInt(u as u64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Float(f as f64)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<char> for Value {
    fn from(c: char) -> Self {
        Value::Char(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_conversions() {
        // String conversions
        let v = Value::from("hello");
        assert_eq!(v.as_str(), Some("hello"));

        // Integer conversions
        let v = Value::from(42i64);
        assert_eq!(v.as_int(), Some(42));
        assert_eq!(v.as_float(), Some(42.0));

        // Float conversions
        let v = Value::from(3.14);
        assert_eq!(v.as_float(), Some(3.14));

        // Bool conversions
        let v = Value::from(true);
        assert_eq!(v.as_bool(), Some(true));
        assert_eq!(v.as_int(), Some(1));

        // Char conversions
        let v = Value::from('a');
        assert_eq!(v.as_char(), Some('a'));
    }

    #[test]
    fn test_display() {
        assert_eq!(Value::from("hello").to_string(), "hello");
        assert_eq!(Value::from(42).to_string(), "42");
        assert_eq!(Value::from(3.14).to_string(), "3.14");
        assert_eq!(Value::from(true).to_string(), "true");
        assert_eq!(Value::from('a').to_string(), "a");
    }
}
