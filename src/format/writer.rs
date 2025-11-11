//! Low-level formatting functions for different value types.

use crate::error::{Error, Result};
use crate::spec::{FormatSpec, Grouping, Sign, TypeSpec};
use crate::types::Value;

/// Format a value as a string.
pub fn format_string(value: &Value, spec: &FormatSpec) -> Result<String> {
    let s = match value {
        Value::Str(s) => s.clone(),
        Value::Char(c) => c.to_string(),
        _ => value.to_string(),
    };

    // Apply precision (max length for strings)
    let s = if let Some(precision) = spec.precision {
        s.chars().take(precision).collect()
    } else {
        s
    };

    Ok(s)
}

/// Format a value as a decimal integer.
pub fn format_decimal(value: &Value, spec: &FormatSpec) -> Result<String> {
    let num = value.to_int()?;

    let mut result = num.abs().to_string();

    // Apply grouping
    if let Some(grouping) = spec.grouping {
        result = apply_grouping(&result, grouping, 3);
    }

    // Add sign
    result = add_sign(&result, num, spec);

    // Apply zero padding (only if no explicit alignment)
    if spec.zero_pad && spec.align.is_none() {
        if let Some(width) = spec.width {
            result = apply_zero_padding(&result, width);
        }
    }

    Ok(result)
}

/// Format a value as a binary integer.
pub fn format_binary(value: &Value, spec: &FormatSpec) -> Result<String> {
    let num = value.to_uint()?;
    let mut result = format!("{:b}", num);

    // Apply grouping
    if let Some(grouping) = spec.grouping {
        result = apply_grouping(&result, grouping, 4);
    }

    // Add alternate form prefix
    if spec.alternate && num != 0 {
        result = format!("0b{}", result);
    }

    // Apply zero padding
    if spec.zero_pad && spec.align.is_none() {
        if let Some(width) = spec.width {
            let prefix_len = if spec.alternate && num != 0 { 2 } else { 0 };
            if result.len() < width {
                let zeros = width - result.len();
                if prefix_len > 0 {
                    result = format!("0b{:0>width$}", &result[2..], width = width - 2);
                } else {
                    result = format!("{:0>width$}", result, width = width);
                }
            }
        }
    }

    Ok(result)
}

/// Format a value as an octal integer.
pub fn format_octal(value: &Value, spec: &FormatSpec) -> Result<String> {
    let num = value.to_uint()?;
    let mut result = format!("{:o}", num);

    // Apply grouping
    if let Some(grouping) = spec.grouping {
        result = apply_grouping(&result, grouping, 4);
    }

    // Add alternate form prefix
    if spec.alternate && num != 0 {
        result = format!("0o{}", result);
    }

    // Apply zero padding
    if spec.zero_pad && spec.align.is_none() {
        if let Some(width) = spec.width {
            let prefix_len = if spec.alternate && num != 0 { 2 } else { 0 };
            if result.len() < width {
                if prefix_len > 0 {
                    result = format!("0o{:0>width$}", &result[2..], width = width - 2);
                } else {
                    result = format!("{:0>width$}", result, width = width);
                }
            }
        }
    }

    Ok(result)
}

/// Format a value as a hexadecimal integer.
pub fn format_hex(value: &Value, spec: &FormatSpec, uppercase: bool) -> Result<String> {
    let num = value.to_uint()?;
    let mut result = if uppercase {
        format!("{:X}", num)
    } else {
        format!("{:x}", num)
    };

    // Apply grouping
    if let Some(grouping) = spec.grouping {
        result = apply_grouping(&result, grouping, 4);
    }

    // Add alternate form prefix
    if spec.alternate && num != 0 {
        let prefix = if uppercase { "0X" } else { "0x" };
        result = format!("{}{}", prefix, result);
    }

    // Apply zero padding
    if spec.zero_pad && spec.align.is_none() {
        if let Some(width) = spec.width {
            let prefix_len = if spec.alternate && num != 0 { 2 } else { 0 };
            if result.len() < width {
                if prefix_len > 0 {
                    let prefix = if uppercase { "0X" } else { "0x" };
                    result = format!("{}{:0>width$}", prefix, &result[2..], width = width - 2);
                } else {
                    result = format!("{:0>width$}", result, width = width);
                }
            }
        }
    }

    Ok(result)
}

/// Format a value as a fixed-point float.
pub fn format_fixed(value: &Value, spec: &FormatSpec) -> Result<String> {
    let mut num = value.to_float()?;

    // Handle zero flag (coerce -0.0 to 0.0)
    if spec.zero_flag && num == 0.0 && num.is_sign_negative() {
        num = 0.0;
    }

    let precision = spec.precision.unwrap_or(6);

    let abs_num = num.abs();
    let mut result = format!("{:.precision$}", abs_num, precision = precision);

    // Apply grouping to integer part
    if let Some(grouping) = spec.grouping {
        if let Some(dot_pos) = result.find('.') {
            let int_part = &result[..dot_pos];
            let frac_part = &result[dot_pos..];
            result = format!("{}{}", apply_grouping(int_part, grouping, 3), frac_part);
        }
    }

    // Add sign
    result = add_sign_float(&result, num, spec);

    // Apply zero padding
    if spec.zero_pad && spec.align.is_none() {
        if let Some(width) = spec.width {
            result = apply_zero_padding(&result, width);
        }
    }

    Ok(result)
}

/// Format a value in scientific notation.
pub fn format_exponent(value: &Value, spec: &FormatSpec) -> Result<String> {
    let mut num = value.to_float()?;

    // Handle zero flag
    if spec.zero_flag && num == 0.0 && num.is_sign_negative() {
        num = 0.0;
    }

    let precision = spec.precision.unwrap_or(6);
    let uppercase = matches!(spec.type_spec, Some(TypeSpec::ExponentUpper));

    let abs_num = num.abs();
    let mut result = if uppercase {
        format!("{:.precision$E}", abs_num, precision = precision)
    } else {
        format!("{:.precision$e}", abs_num, precision = precision)
    };

    // Add sign
    result = add_sign_float(&result, num, spec);

    // Apply zero padding
    if spec.zero_pad && spec.align.is_none() {
        if let Some(width) = spec.width {
            result = apply_zero_padding(&result, width);
        }
    }

    Ok(result)
}

/// Format a value using general format (automatically choose fixed or exponent).
pub fn format_general(value: &Value, spec: &FormatSpec) -> Result<String> {
    let mut num = value.to_float()?;

    // Handle zero flag
    if spec.zero_flag && num == 0.0 && num.is_sign_negative() {
        num = 0.0;
    }

    let precision = spec.precision.unwrap_or(6);
    let uppercase = matches!(spec.type_spec, Some(TypeSpec::GeneralUpper));

    // For general format, let Rust's formatting decide
    let abs_num = num.abs();
    let mut result = if uppercase {
        format!("{:.precision$}", abs_num, precision = precision)
    } else {
        format!("{:.precision$}", abs_num, precision = precision)
    };

    // Add sign
    result = add_sign_float(&result, num, spec);

    // Apply zero padding
    if spec.zero_pad && spec.align.is_none() {
        if let Some(width) = spec.width {
            result = apply_zero_padding(&result, width);
        }
    }

    Ok(result)
}

/// Format a value as a percentage.
pub fn format_percentage(value: &Value, spec: &FormatSpec) -> Result<String> {
    let num = value.to_float()? * 100.0;

    let precision = spec.precision.unwrap_or(6);
    let mut result = format!("{:.precision$}", num.abs(), precision = precision);

    // Add sign
    result = add_sign_float(&result, num, spec);

    // Add percentage symbol
    result.push('%');

    // Apply zero padding
    if spec.zero_pad && spec.align.is_none() {
        if let Some(width) = spec.width {
            // Remove % before padding, add back after
            result.pop();
            result = apply_zero_padding(&result, width - 1);
            result.push('%');
        }
    }

    Ok(result)
}

/// Format a value as a character.
pub fn format_character(value: &Value) -> Result<String> {
    match value {
        Value::Char(c) => Ok(c.to_string()),
        Value::Int(i) if *i >= 0 && *i <= 0x10FFFF => {
            let c = char::from_u32(*i as u32)
                .ok_or_else(|| Error::ConversionError(format!("invalid character code: {}", i)))?;
            Ok(c.to_string())
        }
        Value::Str(s) if s.len() == 1 => Ok(s.clone()),
        _ => Err(Error::ConversionError(format!(
            "cannot format {:?} as character",
            value
        ))),
    }
}

/// Apply grouping separators to a numeric string.
fn apply_grouping(s: &str, grouping: Grouping, group_size: usize) -> String {
    let sep = match grouping {
        Grouping::Comma => ',',
        Grouping::Underscore => '_',
    };

    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();

    for (i, &c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % group_size == 0 {
            result.push(sep);
        }
        result.push(c);
    }

    result
}

/// Add sign to a formatted integer.
fn add_sign(s: &str, num: i64, spec: &FormatSpec) -> String {
    let sign = match spec.sign {
        Some(Sign::Plus) => {
            if num >= 0 {
                "+"
            } else {
                "-"
            }
        }
        Some(Sign::Space) => {
            if num >= 0 {
                " "
            } else {
                "-"
            }
        }
        Some(Sign::Minus) | None => {
            if num < 0 {
                "-"
            } else {
                ""
            }
        }
    };

    format!("{}{}", sign, s)
}

/// Add sign to a formatted float.
fn add_sign_float(s: &str, num: f64, spec: &FormatSpec) -> String {
    let sign = match spec.sign {
        Some(Sign::Plus) => {
            if num >= 0.0 {
                "+"
            } else {
                "-"
            }
        }
        Some(Sign::Space) => {
            if num >= 0.0 {
                " "
            } else {
                "-"
            }
        }
        Some(Sign::Minus) | None => {
            if num < 0.0 {
                "-"
            } else {
                ""
            }
        }
    };

    format!("{}{}", sign, s)
}

/// Apply zero padding to a numeric string.
fn apply_zero_padding(s: &str, width: usize) -> String {
    if s.len() >= width {
        return s.to_string();
    }

    // Check if there's a sign or prefix
    let (prefix, rest) = if let Some(first) = s.chars().next() {
        if first == '+' || first == '-' || first == ' ' {
            (first.to_string(), &s[1..])
        } else if s.starts_with("0x") || s.starts_with("0X") {
            (s[..2].to_string(), &s[2..])
        } else if s.starts_with("0b") || s.starts_with("0B") {
            (s[..2].to_string(), &s[2..])
        } else if s.starts_with("0o") || s.starts_with("0O") {
            (s[..2].to_string(), &s[2..])
        } else {
            (String::new(), s)
        }
    } else {
        (String::new(), s)
    };

    let padding_needed = width.saturating_sub(s.len());
    format!("{}{:0>width$}", prefix, rest, width = rest.len() + padding_needed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_string() {
        let value = Value::from("hello");
        let spec = FormatSpec::default();
        assert_eq!(format_string(&value, &spec).unwrap(), "hello");

        let mut spec = FormatSpec::default();
        spec.precision = Some(3);
        assert_eq!(format_string(&value, &spec).unwrap(), "hel");
    }

    #[test]
    fn test_format_decimal() {
        let value = Value::from(42);
        let spec = FormatSpec::default();
        assert_eq!(format_decimal(&value, &spec).unwrap(), "42");

        let mut spec = FormatSpec::default();
        spec.sign = Some(Sign::Plus);
        assert_eq!(format_decimal(&value, &spec).unwrap(), "+42");

        let value = Value::from(-42);
        assert_eq!(format_decimal(&value, &spec).unwrap(), "-42");
    }

    #[test]
    fn test_format_binary() {
        let value = Value::from(10);
        let spec = FormatSpec::default();
        assert_eq!(format_binary(&value, &spec).unwrap(), "1010");

        let mut spec = FormatSpec::default();
        spec.alternate = true;
        assert_eq!(format_binary(&value, &spec).unwrap(), "0b1010");
    }

    #[test]
    fn test_format_hex() {
        let value = Value::from(255);
        let spec = FormatSpec::default();
        assert_eq!(format_hex(&value, &spec, false).unwrap(), "ff");
        assert_eq!(format_hex(&value, &spec, true).unwrap(), "FF");

        let mut spec = FormatSpec::default();
        spec.alternate = true;
        assert_eq!(format_hex(&value, &spec, false).unwrap(), "0xff");
        assert_eq!(format_hex(&value, &spec, true).unwrap(), "0XFF");
    }

    #[test]
    fn test_grouping() {
        assert_eq!(apply_grouping("1000", Grouping::Comma, 3), "1,000");
        assert_eq!(apply_grouping("1000000", Grouping::Comma, 3), "1,000,000");
        assert_eq!(apply_grouping("1111", Grouping::Underscore, 4), "1111");
        assert_eq!(apply_grouping("11111", Grouping::Underscore, 4), "1_1111");
    }
}
