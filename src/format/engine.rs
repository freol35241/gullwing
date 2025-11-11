//! Core formatting engine.

use crate::error::{Error, Result};
use crate::spec::{Alignment, FormatSpec, TypeSpec};
use crate::types::Value;
use std::collections::HashMap;

/// A formatter that can format values according to a format string.
///
/// # Examples
///
/// ```
/// use gullwing::{Formatter, Value};
///
/// let formatter = Formatter::new("{name:>10}").unwrap();
/// let mut values = std::collections::HashMap::new();
/// values.insert("name".to_string(), Value::from("Alice"));
/// let result = formatter.format_map(&values).unwrap();
/// assert_eq!(result, "     Alice");
/// ```
#[derive(Debug, Clone)]
pub struct Formatter {
    #[allow(dead_code)]
    pattern: String,
    fields: Vec<Field>,
}

#[derive(Debug, Clone)]
struct Field {
    prefix: String,       // Text before the field
    name: Option<String>, // Field name (None for positional)
    index: Option<usize>, // Positional index
    spec: FormatSpec,     // Format specification
}

impl Formatter {
    /// Create a new formatter from a format pattern.
    ///
    /// The pattern may contain:
    /// - Named fields: `{name}` or `{name:spec}`
    /// - Positional fields: `{}` or `{:spec}` or `{0:spec}`
    /// - Literal braces: `{{` and `}}`
    ///
    /// # Examples
    ///
    /// ```
    /// use gullwing::Formatter;
    ///
    /// let f = Formatter::new("{name} is {age:d} years old").unwrap();
    /// ```
    pub fn new(pattern: &str) -> Result<Self> {
        let fields = parse_format_string(pattern)?;
        Ok(Formatter {
            pattern: pattern.to_string(),
            fields,
        })
    }

    /// Format values from a HashMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use gullwing::{Formatter, Value};
    /// use std::collections::HashMap;
    ///
    /// let formatter = Formatter::new("{name:>10}").unwrap();
    /// let mut values = HashMap::new();
    /// values.insert("name".to_string(), Value::from("Alice"));
    /// let result = formatter.format_map(&values).unwrap();
    /// assert_eq!(result, "     Alice");
    /// ```
    pub fn format_map(&self, values: &HashMap<String, Value>) -> Result<String> {
        let mut result = String::new();

        for field in &self.fields {
            // Append prefix text
            result.push_str(&field.prefix);

            // Skip if this is the trailing field (no name or index)
            if field.name.is_none() && field.index.is_none() {
                continue;
            }

            // Get the value
            let value = if let Some(name) = &field.name {
                values
                    .get(name)
                    .ok_or_else(|| Error::MissingField(name.clone()))?
            } else {
                return Err(Error::InvalidFormatSpec(
                    "positional fields not supported with format_map".to_string(),
                ));
            };

            // Format the value
            let formatted = format_value(value, &field.spec)?;
            result.push_str(&formatted);
        }

        Ok(result)
    }

    /// Format values from a closure that provides values by field name.
    ///
    /// # Examples
    ///
    /// ```
    /// use gullwing::{Formatter, Value};
    ///
    /// let formatter = Formatter::new("{x} + {y} = {z}").unwrap();
    /// let result = formatter.format_fn(|name| {
    ///     match name {
    ///         "x" => Some(Value::from(1)),
    ///         "y" => Some(Value::from(2)),
    ///         "z" => Some(Value::from(3)),
    ///         _ => None,
    ///     }
    /// }).unwrap();
    /// assert_eq!(result, "1 + 2 = 3");
    /// ```
    pub fn format_fn<F>(&self, mut f: F) -> Result<String>
    where
        F: FnMut(&str) -> Option<Value>,
    {
        let mut result = String::new();

        for field in &self.fields {
            result.push_str(&field.prefix);

            // Skip if this is the trailing field (no name or index)
            if field.name.is_none() && field.index.is_none() {
                continue;
            }

            let value = if let Some(name) = &field.name {
                f(name).ok_or_else(|| Error::MissingField(name.clone()))?
            } else {
                return Err(Error::InvalidFormatSpec(
                    "positional fields not supported with format_fn".to_string(),
                ));
            };

            let formatted = format_value(&value, &field.spec)?;
            result.push_str(&formatted);
        }

        Ok(result)
    }

    /// Format positional values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gullwing::{Formatter, Value};
    ///
    /// let formatter = Formatter::new("{0} + {1} = {2}").unwrap();
    /// let values = vec![Value::from(1), Value::from(2), Value::from(3)];
    /// let result = formatter.format_positional(&values).unwrap();
    /// assert_eq!(result, "1 + 2 = 3");
    /// ```
    pub fn format_positional(&self, values: &[Value]) -> Result<String> {
        let mut result = String::new();

        for field in &self.fields {
            result.push_str(&field.prefix);

            // Skip if this is the trailing field (no name or index)
            if field.name.is_none() && field.index.is_none() {
                continue;
            }

            let value = if let Some(index) = field.index {
                values
                    .get(index)
                    .ok_or_else(|| Error::MissingField(format!("position {}", index)))?
            } else if field.name.is_some() {
                return Err(Error::InvalidFormatSpec(
                    "named fields not supported with format_positional".to_string(),
                ));
            } else {
                return Err(Error::InvalidFormatSpec(
                    "cannot mix auto and manual indexing".to_string(),
                ));
            };

            let formatted = format_value(value, &field.spec)?;
            result.push_str(&formatted);
        }

        Ok(result)
    }
}

/// Parse a format string into fields.
fn parse_format_string(pattern: &str) -> Result<Vec<Field>> {
    let mut fields = Vec::new();
    let mut chars = pattern.chars().peekable();
    let mut prefix = String::new();
    let mut auto_index = 0;

    while let Some(ch) = chars.next() {
        match ch {
            '{' => {
                if chars.peek() == Some(&'{') {
                    // Escaped brace
                    chars.next();
                    prefix.push('{');
                } else {
                    // Parse field
                    let field_str = parse_until_closing_brace(&mut chars)?;
                    let field = parse_field(&field_str, &mut auto_index)?;
                    fields.push(Field {
                        prefix: prefix.clone(),
                        name: field.0,
                        index: field.1,
                        spec: field.2,
                    });
                    prefix.clear();
                }
            }
            '}' => {
                if chars.peek() == Some(&'}') {
                    // Escaped brace
                    chars.next();
                    prefix.push('}');
                } else {
                    return Err(Error::InvalidFormatSpec(
                        "unmatched '}' in format string".to_string(),
                    ));
                }
            }
            _ => prefix.push(ch),
        }
    }

    // Always add a trailing field to represent text after the last placeholder
    // (even if empty). This simplifies formatting logic.
    fields.push(Field {
        prefix,
        name: None,
        index: None,
        spec: FormatSpec::default(),
    });

    Ok(fields)
}

/// Parse until we find a closing brace.
fn parse_until_closing_brace(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String> {
    let mut result = String::new();
    let mut depth = 0;

    while let Some(&ch) = chars.peek() {
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            if depth == 0 {
                chars.next(); // consume the '}'
                return Ok(result);
            }
            depth -= 1;
        }
        result.push(ch);
        chars.next();
    }

    Err(Error::InvalidFormatSpec(
        "unclosed '{' in format string".to_string(),
    ))
}

/// Parse a field specification.
/// Returns (name, index, spec)
fn parse_field(
    field: &str,
    auto_index: &mut usize,
) -> Result<(Option<String>, Option<usize>, FormatSpec)> {
    // Split on ':'
    let parts: Vec<&str> = field.splitn(2, ':').collect();
    let name_part = parts[0];
    let spec_part = parts.get(1).copied().unwrap_or("");

    // Parse the name/index part
    let (name, index) = if name_part.is_empty() {
        // Auto-numbered positional field
        let idx = *auto_index;
        *auto_index += 1;
        (None, Some(idx))
    } else if let Ok(idx) = name_part.parse::<usize>() {
        // Explicit positional field
        (None, Some(idx))
    } else if name_part.chars().all(|c| c.is_alphanumeric() || c == '_') {
        // Named field
        (Some(name_part.to_string()), None)
    } else {
        return Err(Error::InvalidFieldName(name_part.to_string()));
    };

    // Parse the format spec
    let spec = FormatSpec::parse(spec_part)?;

    Ok((name, index, spec))
}

/// Format a value according to a format specification.
fn format_value(value: &Value, spec: &FormatSpec) -> Result<String> {
    use super::writer::*;

    // Determine the type of formatting to perform
    let type_spec = spec.type_spec.unwrap_or({
        // Default type based on value
        match value {
            Value::Str(_) | Value::Char(_) => TypeSpec::String,
            Value::Int(_) | Value::UInt(_) | Value::Bool(_) => TypeSpec::Decimal,
            Value::Float(_) => TypeSpec::GeneralLower,
        }
    });

    // Format according to type
    let formatted = match type_spec {
        TypeSpec::String => format_string(value, spec)?,
        TypeSpec::Decimal => format_decimal(value, spec)?,
        TypeSpec::Binary => format_binary(value, spec)?,
        TypeSpec::Octal => format_octal(value, spec)?,
        TypeSpec::HexLower => format_hex(value, spec, false)?,
        TypeSpec::HexUpper => format_hex(value, spec, true)?,
        TypeSpec::FixedLower | TypeSpec::FixedUpper => format_fixed(value, spec)?,
        TypeSpec::ExponentLower | TypeSpec::ExponentUpper => format_exponent(value, spec)?,
        TypeSpec::GeneralLower | TypeSpec::GeneralUpper => format_general(value, spec)?,
        TypeSpec::Percentage => format_percentage(value, spec)?,
        TypeSpec::Character => format_character(value)?,
        TypeSpec::Number => format_decimal(value, spec)?, // TODO: locale-aware
    };

    // Apply alignment and padding
    let result = apply_alignment(&formatted, spec);

    Ok(result)
}

/// Apply alignment and padding to a formatted value.
fn apply_alignment(s: &str, spec: &FormatSpec) -> String {
    let width = match spec.width {
        Some(w) if w > s.len() => w,
        _ => return s.to_string(),
    };

    let fill = spec.fill_char();
    let padding_needed = width - s.len();

    let align = spec.align.unwrap_or(
        // Default alignment depends on type
        if spec.is_numeric() {
            Alignment::Right
        } else {
            Alignment::Left
        },
    );

    match align {
        Alignment::Left => {
            let mut result = s.to_string();
            result.push_str(&fill.to_string().repeat(padding_needed));
            result
        }
        Alignment::Right => {
            let mut result = fill.to_string().repeat(padding_needed);
            result.push_str(s);
            result
        }
        Alignment::Center => {
            let left_pad = padding_needed / 2;
            let right_pad = padding_needed - left_pad;
            let mut result = fill.to_string().repeat(left_pad);
            result.push_str(s);
            result.push_str(&fill.to_string().repeat(right_pad));
            result
        }
        Alignment::AfterSign => {
            // Insert padding after sign for numeric values
            if let Some(first_char) = s.chars().next() {
                if first_char == '+' || first_char == '-' || first_char == ' ' {
                    let mut result = String::new();
                    result.push(first_char);
                    result.push_str(&fill.to_string().repeat(padding_needed));
                    result.push_str(&s[1..]);
                    return result;
                }
            }
            // No sign, just right-align
            let mut result = fill.to_string().repeat(padding_needed);
            result.push_str(s);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_pattern() {
        let fields = parse_format_string("Hello {name}!").unwrap();
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].prefix, "Hello ");
        assert_eq!(fields[0].name, Some("name".to_string()));
        assert_eq!(fields[1].prefix, "!");
    }

    #[test]
    fn test_parse_positional() {
        let fields = parse_format_string("{0} + {1} = {2}").unwrap();
        assert_eq!(fields.len(), 4);
        assert_eq!(fields[0].index, Some(0));
        assert_eq!(fields[1].prefix, " + ");
        assert_eq!(fields[1].index, Some(1));
    }

    #[test]
    fn test_parse_with_spec() {
        let fields = parse_format_string("{value:05d}").unwrap();
        assert_eq!(fields[0].name, Some("value".to_string()));
        assert_eq!(fields[0].spec.width, Some(5));
        assert_eq!(fields[0].spec.zero_pad, true);
    }

    #[test]
    fn test_escaped_braces() {
        let fields = parse_format_string("{{escaped}}").unwrap();
        assert_eq!(fields[0].prefix, "{escaped}");
    }
}
