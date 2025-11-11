//! Parser for extracting structured data from strings.

use super::builder::{build_regex_pattern, CaptureInfo};
use crate::error::{Error, Result};
use crate::spec::TypeSpec;
use crate::types::Value;
use regex::Regex;
use std::collections::HashMap;

/// A parser that extracts structured data from strings using a format pattern.
///
/// # Examples
///
/// ```
/// use gullwing::Parser;
///
/// let parser = Parser::new("{name} is {age:d} years old").unwrap();
/// let result = parser.parse("Alice is 30 years old").unwrap().unwrap();
///
/// assert_eq!(result.get("name").unwrap().as_str(), Some("Alice"));
/// assert_eq!(result.get("age").unwrap().as_int(), Some(30));
/// ```
#[derive(Debug, Clone)]
pub struct Parser {
    pattern: String,
    regex: Regex,
    captures: Vec<CaptureInfo>,
}

impl Parser {
    /// Create a new parser from a format pattern.
    ///
    /// The pattern uses the same syntax as formatting, with named or positional fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use gullwing::Parser;
    ///
    /// // Named fields
    /// let parser = Parser::new("{name} = {value:d}").unwrap();
    ///
    /// // With format specifications
    /// let parser = Parser::new("{date} {time} {level}").unwrap();
    /// ```
    pub fn new(pattern: &str) -> Result<Self> {
        let (regex_pattern, captures) = build_regex_pattern(pattern)?;

        let regex = Regex::new(&regex_pattern)
            .map_err(|e| Error::RegexError(format!("failed to compile regex: {}", e)))?;

        Ok(Parser {
            pattern: pattern.to_string(),
            regex,
            captures,
        })
    }

    /// Parse a string, matching it exactly against the pattern.
    ///
    /// Returns `Ok(Some(result))` if the string matches, `Ok(None)` if it doesn't match.
    ///
    /// # Examples
    ///
    /// ```
    /// use gullwing::Parser;
    ///
    /// let parser = Parser::new("{x:d} + {y:d}").unwrap();
    /// let result = parser.parse("2 + 3").unwrap().unwrap();
    ///
    /// assert_eq!(result.get("x").unwrap().as_int(), Some(2));
    /// assert_eq!(result.get("y").unwrap().as_int(), Some(3));
    /// ```
    pub fn parse(&self, text: &str) -> Result<Option<ParseResult>> {
        let full_regex = format!("^{}$", self.regex.as_str());
        let full_regex = Regex::new(&full_regex)
            .map_err(|e| Error::RegexError(format!("failed to compile regex: {}", e)))?;

        if let Some(cap) = full_regex.captures(text) {
            let values = self.extract_values(&cap)?;
            Ok(Some(ParseResult {
                values,
                text: text.to_string(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Search for the pattern within a string.
    ///
    /// Returns the first match found, or `None` if no match is found.
    ///
    /// # Examples
    ///
    /// ```
    /// use gullwing::Parser;
    ///
    /// let parser = Parser::new("{number:d}").unwrap();
    /// let result = parser.search("The answer is 42!").unwrap().unwrap();
    ///
    /// assert_eq!(result.get("number").unwrap().as_int(), Some(42));
    /// ```
    pub fn search(&self, text: &str) -> Result<Option<ParseResult>> {
        if let Some(cap) = self.regex.captures(text) {
            let values = self.extract_values(&cap)?;
            Ok(Some(ParseResult {
                values,
                text: text.to_string(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Find all occurrences of the pattern in a string.
    ///
    /// Returns an iterator over all matches.
    ///
    /// # Examples
    ///
    /// ```
    /// use gullwing::Parser;
    ///
    /// let parser = Parser::new("{number:d}").unwrap();
    /// let results: Vec<_> = parser.findall("Numbers: 1, 2, 3").unwrap().collect();
    ///
    /// assert_eq!(results.len(), 3);
    /// assert_eq!(results[0].get("number").unwrap().as_int(), Some(1));
    /// assert_eq!(results[1].get("number").unwrap().as_int(), Some(2));
    /// assert_eq!(results[2].get("number").unwrap().as_int(), Some(3));
    /// ```
    pub fn findall(&self, text: &str) -> Result<impl Iterator<Item = ParseResult> + '_> {
        let captures: Vec<_> = self.regex.captures_iter(text).collect();

        let results: Result<Vec<_>> = captures
            .into_iter()
            .map(|cap| {
                let values = self.extract_values(&cap)?;
                Ok(ParseResult {
                    values,
                    text: text.to_string(),
                })
            })
            .collect();

        Ok(results?.into_iter())
    }

    /// Extract and convert captured values.
    fn extract_values(&self, cap: &regex::Captures) -> Result<HashMap<String, Value>> {
        let mut values = HashMap::new();

        for info in &self.captures {
            if let Some(matched) = cap.name(&info.name) {
                let text = matched.as_str();
                let value = convert_value(text, &info.spec)?;
                values.insert(info.name.clone(), value);
            }
        }

        Ok(values)
    }
}

/// Result of parsing a string.
///
/// Contains the extracted values as a map from field names to values.
#[derive(Debug, Clone)]
pub struct ParseResult {
    values: HashMap<String, Value>,
    text: String,
}

impl ParseResult {
    /// Get a value by field name.
    ///
    /// # Examples
    ///
    /// ```
    /// use gullwing::Parser;
    ///
    /// let parser = Parser::new("{name}").unwrap();
    /// let result = parser.parse("Alice").unwrap().unwrap();
    ///
    /// assert_eq!(result.get("name").unwrap().as_str(), Some("Alice"));
    /// ```
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    /// Get all values as a HashMap.
    pub fn values(&self) -> &HashMap<String, Value> {
        &self.values
    }

    /// Get the original text that was parsed.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Check if a field exists in the result.
    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}

/// Convert a captured string to a typed value based on the format spec.
fn convert_value(text: &str, spec: &crate::spec::FormatSpec) -> Result<Value> {
    let type_spec = spec.type_spec.unwrap_or(TypeSpec::String);

    match type_spec {
        TypeSpec::String => Ok(Value::Str(text.to_string())),

        TypeSpec::Decimal | TypeSpec::Number => {
            let cleaned = text.replace(',', "").replace('_', "");
            cleaned
                .parse::<i64>()
                .map(Value::Int)
                .map_err(|e| Error::ConversionError(format!("failed to parse integer: {}", e)))
        }

        TypeSpec::Binary => {
            let cleaned = text.trim_start_matches("0b").trim_start_matches("0B");
            i64::from_str_radix(cleaned, 2)
                .map(Value::Int)
                .map_err(|e| Error::ConversionError(format!("failed to parse binary: {}", e)))
        }

        TypeSpec::Octal => {
            let cleaned = text.trim_start_matches("0o").trim_start_matches("0O");
            i64::from_str_radix(cleaned, 8)
                .map(Value::Int)
                .map_err(|e| Error::ConversionError(format!("failed to parse octal: {}", e)))
        }

        TypeSpec::HexLower | TypeSpec::HexUpper => {
            let cleaned = text
                .trim_start_matches("0x")
                .trim_start_matches("0X")
                .replace('_', "");
            i64::from_str_radix(&cleaned, 16)
                .map(Value::Int)
                .map_err(|e| Error::ConversionError(format!("failed to parse hex: {}", e)))
        }

        TypeSpec::FixedLower
        | TypeSpec::FixedUpper
        | TypeSpec::ExponentLower
        | TypeSpec::ExponentUpper
        | TypeSpec::GeneralLower
        | TypeSpec::GeneralUpper => text
            .parse::<f64>()
            .map(Value::Float)
            .map_err(|e| Error::ConversionError(format!("failed to parse float: {}", e))),

        TypeSpec::Percentage => {
            let cleaned = text.trim_end_matches('%');
            cleaned
                .parse::<f64>()
                .map(|v| Value::Float(v / 100.0))
                .map_err(|e| Error::ConversionError(format!("failed to parse percentage: {}", e)))
        }

        TypeSpec::Character => {
            if text.len() == 1 {
                Ok(Value::Char(text.chars().next().unwrap()))
            } else {
                Err(Error::ConversionError(format!(
                    "expected single character, got: {}",
                    text
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let parser = Parser::new("{name}").unwrap();
        let result = parser.parse("Alice").unwrap().unwrap();
        assert_eq!(result.get("name").unwrap().as_str(), Some("Alice"));
    }

    #[test]
    fn test_parse_integers() {
        let parser = Parser::new("{x:d} + {y:d} = {z:d}").unwrap();
        let result = parser.parse("2 + 3 = 5").unwrap().unwrap();

        assert_eq!(result.get("x").unwrap().as_int(), Some(2));
        assert_eq!(result.get("y").unwrap().as_int(), Some(3));
        assert_eq!(result.get("z").unwrap().as_int(), Some(5));
    }

    #[test]
    fn test_parse_floats() {
        let parser = Parser::new("{value:f}").unwrap();
        let result = parser.parse("3.14").unwrap().unwrap();
        assert_eq!(result.get("value").unwrap().as_float(), Some(3.14));
    }

    #[test]
    fn test_parse_hex() {
        let parser = Parser::new("{value:x}").unwrap();
        let result = parser.parse("0xff").unwrap().unwrap();
        assert_eq!(result.get("value").unwrap().as_int(), Some(255));

        let result = parser.parse("ff").unwrap().unwrap();
        assert_eq!(result.get("value").unwrap().as_int(), Some(255));
    }

    #[test]
    fn test_search() {
        let parser = Parser::new("{number:d}").unwrap();
        let result = parser.search("The answer is 42!").unwrap().unwrap();
        assert_eq!(result.get("number").unwrap().as_int(), Some(42));
    }

    #[test]
    fn test_no_match() {
        let parser = Parser::new("{number:d}").unwrap();
        let result = parser.parse("no numbers here").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_findall() {
        let parser = Parser::new("{num:d}").unwrap();
        let results: Vec<_> = parser.findall("Numbers: 1, 2, 3").unwrap().collect();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].get("num").unwrap().as_int(), Some(1));
        assert_eq!(results[1].get("num").unwrap().as_int(), Some(2));
        assert_eq!(results[2].get("num").unwrap().as_int(), Some(3));
    }
}
