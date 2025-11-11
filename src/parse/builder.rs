//! Build regex patterns from format strings.

use crate::error::{Error, Result};
use crate::spec::{FormatSpec, TypeSpec};
use std::collections::HashMap;

/// Information about a capture group in a regex pattern.
#[derive(Debug, Clone)]
pub struct CaptureInfo {
    pub name: String,
    pub spec: FormatSpec,
    pub group_index: usize,
}

/// Build a regex pattern from a format string.
///
/// Returns the regex pattern and information about capture groups.
pub fn build_regex_pattern(format_str: &str) -> Result<(String, Vec<CaptureInfo>)> {
    let mut pattern = String::new();
    let mut captures = Vec::new();
    let mut chars = format_str.chars().peekable();
    let mut group_index = 1; // Regex group indices start at 1
    let mut auto_index = 0;

    while let Some(ch) = chars.next() {
        match ch {
            '{' => {
                if chars.peek() == Some(&'{') {
                    // Escaped brace
                    chars.next();
                    pattern.push_str(r"\{");
                } else {
                    // Parse field
                    let field_str = parse_until_closing_brace(&mut chars)?;
                    let (field_pattern, capture_info) =
                        build_field_pattern(&field_str, &mut group_index, &mut auto_index)?;
                    pattern.push_str(&field_pattern);
                    if let Some(info) = capture_info {
                        captures.push(info);
                    }
                }
            }
            '}' => {
                if chars.peek() == Some(&'}') {
                    // Escaped brace
                    chars.next();
                    pattern.push_str(r"\}");
                } else {
                    return Err(Error::InvalidFormatSpec(
                        "unmatched '}' in format string".to_string(),
                    ));
                }
            }
            // Escape regex special characters
            '.' | '*' | '+' | '?' | '|' | '(' | ')' | '[' | ']' | '^' | '$' | '\\' => {
                pattern.push('\\');
                pattern.push(ch);
            }
            _ => pattern.push(ch),
        }
    }

    Ok((pattern, captures))
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

/// Build a regex pattern for a field.
///
/// Returns the pattern and optional capture info.
fn build_field_pattern(
    field: &str,
    group_index: &mut usize,
    auto_index: &mut usize,
) -> Result<(String, Option<CaptureInfo>)> {
    // Split on ':'
    let parts: Vec<&str> = field.splitn(2, ':').collect();
    let name_part = parts[0];
    let spec_part = parts.get(1).copied().unwrap_or("");

    // Determine field name
    let name = if name_part.is_empty() {
        // Auto-numbered field
        let n = format!("_{}", auto_index);
        *auto_index += 1;
        n
    } else if name_part.chars().all(|c| c.is_alphanumeric() || c == '_') {
        name_part.to_string()
    } else {
        return Err(Error::InvalidFieldName(name_part.to_string()));
    };

    // Parse format spec
    let spec = FormatSpec::parse(spec_part)?;

    // Build regex pattern based on type
    let type_spec = spec.type_spec.unwrap_or(TypeSpec::String);
    let regex_pattern = match type_spec {
        TypeSpec::String => {
            if let Some(width) = spec.width {
                if let Some(precision) = spec.precision {
                    // Both width and precision: match between width and precision chars
                    format!(r".{{{},{}}}", width, precision)
                } else {
                    // Just width: match at least width chars
                    format!(r".{{{},}}", width)
                }
            } else if let Some(precision) = spec.precision {
                // Just precision: match up to precision chars
                format!(r".{{1,{}}}", precision)
            } else {
                // No constraints: match any non-empty string (non-greedy)
                r".+?".to_string()
            }
        }
        TypeSpec::Decimal | TypeSpec::Number => {
            // Match optional sign and digits
            r"[-+]?\d+".to_string()
        }
        TypeSpec::Binary => {
            // Match binary with optional 0b prefix
            r"(?:0[bB])?[01]+".to_string()
        }
        TypeSpec::Octal => {
            // Match octal with optional 0o prefix
            r"(?:0[oO])?[0-7]+".to_string()
        }
        TypeSpec::HexLower | TypeSpec::HexUpper => {
            // Match hex with optional 0x prefix
            r"(?:0[xX])?[0-9a-fA-F]+".to_string()
        }
        TypeSpec::FixedLower
        | TypeSpec::FixedUpper
        | TypeSpec::ExponentLower
        | TypeSpec::ExponentUpper
        | TypeSpec::GeneralLower
        | TypeSpec::GeneralUpper => {
            // Match floating point numbers (including scientific notation)
            r"[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?".to_string()
        }
        TypeSpec::Percentage => {
            // Match percentage
            r"[-+]?(?:\d+\.?\d*|\.\d+)%".to_string()
        }
        TypeSpec::Character => {
            // Match single character
            r".".to_string()
        }
    };

    // Wrap in named capture group
    let pattern = format!(r"(?P<{}>{})", name, regex_pattern);

    let capture_info = CaptureInfo {
        name: name.clone(),
        spec,
        group_index: *group_index,
    };

    *group_index += 1;

    Ok((pattern, Some(capture_info)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pattern() {
        let (pattern, captures) = build_regex_pattern("{name}").unwrap();
        assert_eq!(pattern, r"(?P<name>.+?)");
        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].name, "name");
    }

    #[test]
    fn test_multiple_fields() {
        let (pattern, captures) = build_regex_pattern("{first} {last}").unwrap();
        assert_eq!(pattern, r"(?P<first>.+?) (?P<last>.+?)");
        assert_eq!(captures.len(), 2);
    }

    #[test]
    fn test_decimal_field() {
        let (pattern, captures) = build_regex_pattern("{value:d}").unwrap();
        assert!(pattern.contains(r"[-+]?\d+"));
        assert_eq!(captures[0].spec.type_spec, Some(TypeSpec::Decimal));
    }

    #[test]
    fn test_float_field() {
        let (pattern, _) = build_regex_pattern("{value:f}").unwrap();
        assert!(pattern.contains(r"[-+]?"));
        assert!(pattern.contains(r"\d+"));
    }

    #[test]
    fn test_escaped_braces() {
        let (pattern, _) = build_regex_pattern("{{literal}}").unwrap();
        assert_eq!(pattern, r"\{literal\}");
    }

    #[test]
    fn test_regex_special_chars() {
        let (pattern, _) = build_regex_pattern("value = {x}").unwrap();
        assert!(pattern.contains("value = "));
    }
}
