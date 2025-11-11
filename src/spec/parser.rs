//! Parser for format specification strings.

use super::types::{Alignment, Grouping, Sign, TypeSpec};
use crate::error::{Error, Result};

/// A parsed format specification.
///
/// Format: `[[fill]align][sign][z][#][0][width][grouping][.precision][type]`
///
/// See: <https://docs.python.org/3/library/string.html#formatspec>
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FormatSpec {
    /// Fill character (default: space)
    pub fill: Option<char>,
    /// Alignment option
    pub align: Option<Alignment>,
    /// Sign option for numeric types
    pub sign: Option<Sign>,
    /// Coerce negative zero to positive (floats only)
    pub zero_flag: bool,
    /// Alternate form (e.g., 0x prefix for hex)
    pub alternate: bool,
    /// Zero-padding for numeric types
    pub zero_pad: bool,
    /// Minimum field width
    pub width: Option<usize>,
    /// Grouping option for numeric types
    pub grouping: Option<Grouping>,
    /// Precision (digits after decimal for floats, max width for strings)
    pub precision: Option<usize>,
    /// Type specifier
    pub type_spec: Option<TypeSpec>,
}

impl FormatSpec {
    /// Parse a format specification string.
    ///
    /// # Examples
    ///
    /// ```
    /// use gullwing::spec::FormatSpec;
    ///
    /// let spec = FormatSpec::parse(">10.2f").unwrap();
    /// assert_eq!(spec.width, Some(10));
    /// assert_eq!(spec.precision, Some(2));
    /// ```
    pub fn parse(input: &str) -> Result<Self> {
        if input.is_empty() {
            return Ok(FormatSpec::default());
        }

        let mut parser = SpecParser::new(input);
        parser.parse()
    }

    /// Check if this spec is for a numeric type.
    pub fn is_numeric(&self) -> bool {
        self.type_spec.map(|t| t.is_numeric()).unwrap_or(false)
    }

    /// Get the effective fill character (default: space).
    pub fn fill_char(&self) -> char {
        self.fill.unwrap_or(' ')
    }
}

/// Internal parser state for format specifications.
struct SpecParser<'a> {
    input: &'a str,
    pos: usize,
    spec: FormatSpec,
}

impl<'a> SpecParser<'a> {
    fn new(input: &'a str) -> Self {
        SpecParser {
            input,
            pos: 0,
            spec: FormatSpec::default(),
        }
    }

    fn parse(&mut self) -> Result<FormatSpec> {
        // Parse [[fill]align]
        self.parse_fill_and_align()?;

        // Parse [sign]
        self.parse_sign()?;

        // Parse [z]
        if self.peek() == Some('z') {
            self.spec.zero_flag = true;
            self.advance();
        }

        // Parse [#]
        if self.peek() == Some('#') {
            self.spec.alternate = true;
            self.advance();
        }

        // Parse [0]
        if self.peek() == Some('0') {
            self.spec.zero_pad = true;
            self.advance();
        }

        // Parse [width]
        self.parse_width()?;

        // Parse [grouping]
        self.parse_grouping()?;

        // Parse [.precision]
        self.parse_precision()?;

        // Parse [type]
        self.parse_type()?;

        // Ensure we consumed all input
        if self.pos < self.input.len() {
            return Err(Error::InvalidFormatSpec(format!(
                "unexpected character at position {}: '{}'",
                self.pos,
                self.input.chars().nth(self.pos).unwrap()
            )));
        }

        Ok(self.spec.clone())
    }

    fn parse_fill_and_align(&mut self) -> Result<()> {
        // Look ahead for alignment characters
        if let Some(first) = self.peek() {
            if let Some(align) = Alignment::from_char(first) {
                // Single character alignment: "<", ">", "^", "="
                self.spec.align = Some(align);
                self.advance();
                return Ok(());
            }
        }

        // Check for fill character followed by alignment
        if self.remaining() >= 2 {
            let chars: Vec<char> = self.input[self.pos..].chars().take(2).collect();
            if chars.len() >= 2 {
                if let Some(align) = Alignment::from_char(chars[1]) {
                    self.spec.fill = Some(chars[0]);
                    self.spec.align = Some(align);
                    self.pos += 2;
                }
            }
        }

        Ok(())
    }

    fn parse_sign(&mut self) -> Result<()> {
        if let Some(c) = self.peek() {
            if let Some(sign) = Sign::from_char(c) {
                self.spec.sign = Some(sign);
                self.advance();
            }
        }
        Ok(())
    }

    fn parse_width(&mut self) -> Result<()> {
        if let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                let start = self.pos;
                while let Some(c) = self.peek() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    self.advance();
                }
                let width_str = &self.input[start..self.pos];
                self.spec.width = Some(
                    width_str
                        .parse()
                        .map_err(|_| Error::InvalidWidth(width_str.to_string()))?,
                );
            }
        }
        Ok(())
    }

    fn parse_grouping(&mut self) -> Result<()> {
        if let Some(c) = self.peek() {
            if let Some(grouping) = Grouping::from_char(c) {
                self.spec.grouping = Some(grouping);
                self.advance();
            }
        }
        Ok(())
    }

    fn parse_precision(&mut self) -> Result<()> {
        if self.peek() == Some('.') {
            self.advance(); // consume '.'

            if let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    let start = self.pos;
                    while let Some(c) = self.peek() {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        self.advance();
                    }
                    let precision_str = &self.input[start..self.pos];
                    self.spec.precision = Some(
                        precision_str
                            .parse()
                            .map_err(|_| Error::InvalidWidth(precision_str.to_string()))?,
                    );
                } else {
                    return Err(Error::InvalidFormatSpec(
                        "precision must be followed by a number".to_string(),
                    ));
                }
            } else {
                return Err(Error::InvalidFormatSpec(
                    "precision must be followed by a number".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn parse_type(&mut self) -> Result<()> {
        if let Some(c) = self.peek() {
            if let Some(type_spec) = TypeSpec::from_char(c) {
                self.spec.type_spec = Some(type_spec);
                self.advance();
            }
        }
        Ok(())
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
        }
    }

    fn remaining(&self) -> usize {
        self.input.len() - self.pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_spec() {
        let spec = FormatSpec::parse("").unwrap();
        assert_eq!(spec, FormatSpec::default());
    }

    #[test]
    fn test_align_only() {
        let spec = FormatSpec::parse("<").unwrap();
        assert_eq!(spec.align, Some(Alignment::Left));

        let spec = FormatSpec::parse(">").unwrap();
        assert_eq!(spec.align, Some(Alignment::Right));

        let spec = FormatSpec::parse("^").unwrap();
        assert_eq!(spec.align, Some(Alignment::Center));

        let spec = FormatSpec::parse("=").unwrap();
        assert_eq!(spec.align, Some(Alignment::AfterSign));
    }

    #[test]
    fn test_fill_and_align() {
        let spec = FormatSpec::parse("*<").unwrap();
        assert_eq!(spec.fill, Some('*'));
        assert_eq!(spec.align, Some(Alignment::Left));

        let spec = FormatSpec::parse("0>").unwrap();
        assert_eq!(spec.fill, Some('0'));
        assert_eq!(spec.align, Some(Alignment::Right));
    }

    #[test]
    fn test_sign() {
        let spec = FormatSpec::parse("+").unwrap();
        assert_eq!(spec.sign, Some(Sign::Plus));

        let spec = FormatSpec::parse("-").unwrap();
        assert_eq!(spec.sign, Some(Sign::Minus));

        let spec = FormatSpec::parse(" ").unwrap();
        assert_eq!(spec.sign, Some(Sign::Space));
    }

    #[test]
    fn test_alternate_and_zero_pad() {
        let spec = FormatSpec::parse("#").unwrap();
        assert_eq!(spec.alternate, true);

        let spec = FormatSpec::parse("0").unwrap();
        assert_eq!(spec.zero_pad, true);

        let spec = FormatSpec::parse("#0").unwrap();
        assert_eq!(spec.alternate, true);
        assert_eq!(spec.zero_pad, true);
    }

    #[test]
    fn test_width() {
        let spec = FormatSpec::parse("10").unwrap();
        assert_eq!(spec.width, Some(10));

        let spec = FormatSpec::parse("123").unwrap();
        assert_eq!(spec.width, Some(123));
    }

    #[test]
    fn test_grouping() {
        let spec = FormatSpec::parse(",").unwrap();
        assert_eq!(spec.grouping, Some(Grouping::Comma));

        let spec = FormatSpec::parse("_").unwrap();
        assert_eq!(spec.grouping, Some(Grouping::Underscore));
    }

    #[test]
    fn test_precision() {
        let spec = FormatSpec::parse(".2").unwrap();
        assert_eq!(spec.precision, Some(2));

        let spec = FormatSpec::parse(".10").unwrap();
        assert_eq!(spec.precision, Some(10));
    }

    #[test]
    fn test_type_spec() {
        let spec = FormatSpec::parse("d").unwrap();
        assert_eq!(spec.type_spec, Some(TypeSpec::Decimal));

        let spec = FormatSpec::parse("f").unwrap();
        assert_eq!(spec.type_spec, Some(TypeSpec::FixedLower));

        let spec = FormatSpec::parse("x").unwrap();
        assert_eq!(spec.type_spec, Some(TypeSpec::HexLower));
    }

    #[test]
    fn test_complex_spec() {
        let spec = FormatSpec::parse(">10.2f").unwrap();
        assert_eq!(spec.align, Some(Alignment::Right));
        assert_eq!(spec.width, Some(10));
        assert_eq!(spec.precision, Some(2));
        assert_eq!(spec.type_spec, Some(TypeSpec::FixedLower));

        let spec = FormatSpec::parse("0=+10,.2f").unwrap();
        assert_eq!(spec.fill, Some('0'));
        assert_eq!(spec.align, Some(Alignment::AfterSign));
        assert_eq!(spec.sign, Some(Sign::Plus));
        assert_eq!(spec.width, Some(10));
        assert_eq!(spec.grouping, Some(Grouping::Comma));
        assert_eq!(spec.precision, Some(2));
        assert_eq!(spec.type_spec, Some(TypeSpec::FixedLower));
    }

    #[test]
    fn test_zero_pad_width() {
        let spec = FormatSpec::parse("05d").unwrap();
        assert_eq!(spec.zero_pad, true);
        assert_eq!(spec.width, Some(5));
        assert_eq!(spec.type_spec, Some(TypeSpec::Decimal));
    }

    #[test]
    fn test_alternate_form() {
        let spec = FormatSpec::parse("#x").unwrap();
        assert_eq!(spec.alternate, true);
        assert_eq!(spec.type_spec, Some(TypeSpec::HexLower));
    }
}
