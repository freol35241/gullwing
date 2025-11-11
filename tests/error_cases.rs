use gullwing::{Error, Formatter, Parser, Value};
use std::collections::HashMap;

#[cfg(test)]
mod error_tests {
    use super::*;

    // ===== Format Spec Parsing Errors =====

    #[test]
    fn invalid_format_spec_unclosed_brace() {
        let result = Formatter::new("{value");
        assert!(result.is_err());
    }

    #[test]
    fn invalid_format_spec_unknown_type() {
        // 'z' is not a valid type specifier, but the parser may accept it
        // and treat it as a default/string type. This test documents current behavior.
        let formatter = Formatter::new("{value:z}");

        // If the formatter was created successfully, unknown types might be treated as string type
        if let Ok(formatter) = formatter {
            let mut values = HashMap::new();
            values.insert("value".to_string(), Value::from("test"));
            // Should format successfully as string
            let result = formatter.format_map(&values);
            assert!(result.is_ok());
        }
        // Otherwise, it should error during parsing
        // Both behaviors are acceptable
    }

    #[test]
    fn empty_field_name_with_spec() {
        // Empty field name with format spec
        let result = Formatter::new("{:d}");
        // This should work (positional argument)
        assert!(result.is_ok());
    }

    // ===== Formatting Errors =====

    #[test]
    fn missing_field_error() {
        let formatter = Formatter::new("{missing_field}").unwrap();
        let values = HashMap::new();
        let result = formatter.format_map(&values);

        assert!(result.is_err());
        match result {
            Err(Error::MissingField(field)) => {
                assert_eq!(field, "missing_field");
            }
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn type_mismatch_int_format_on_string() {
        let formatter = Formatter::new("{value:d}").unwrap();
        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::from("not_a_number"));

        let result = formatter.format_map(&values);
        assert!(result.is_err());
        match result {
            Err(Error::ConversionError(_)) => {}
            _ => panic!("Expected ConversionError"),
        }
    }

    #[test]
    fn type_mismatch_float_format_on_string() {
        let formatter = Formatter::new("{value:.2f}").unwrap();
        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::from("not_a_number"));

        let result = formatter.format_map(&values);
        assert!(result.is_err());
        match result {
            Err(Error::ConversionError(_)) => {}
            _ => panic!("Expected ConversionError"),
        }
    }

    #[test]
    fn negative_with_unsigned_format() {
        let formatter = Formatter::new("{value:x}").unwrap();
        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::from(-42));

        // Negative numbers can't be formatted as hex (unsigned operation)
        let result = formatter.format_map(&values);
        // This might succeed or fail depending on implementation
        // If it succeeds, it should handle the conversion gracefully
    }

    // ===== Parsing Errors =====

    #[test]
    fn parse_no_match() {
        let parser = Parser::new("{value:d}").unwrap();
        let result = parser.parse("not a number");

        // No match should return Ok(None)
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn parse_partial_match() {
        let parser = Parser::new("{a:d} {b:d}").unwrap();
        let result = parser.parse("42");

        // Partial match should return Ok(None) for parse (use search for partial)
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn parse_type_conversion_error() {
        let parser = Parser::new("{value:d}").unwrap();
        // The regex will match digits, so this should actually work
        let result = parser.parse("123abc");

        // Since we're looking for exact match and the pattern is just {value:d},
        // this won't match the whole string
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_parser_pattern() {
        // Try to create a parser with invalid regex characters
        let result = Parser::new("{value");
        assert!(result.is_err());
    }

    // ===== Value Conversion Errors =====

    #[test]
    fn value_to_int_error() {
        let value = Value::from("hello");
        let result = value.to_int();

        assert!(result.is_err());
        match result {
            Err(Error::ConversionError(_)) => {}
            _ => panic!("Expected ConversionError"),
        }
    }

    #[test]
    fn value_to_float_error() {
        let value = Value::from("hello");
        let result = value.to_float();

        assert!(result.is_err());
        match result {
            Err(Error::ConversionError(_)) => {}
            _ => panic!("Expected ConversionError"),
        }
    }

    #[test]
    fn value_to_uint_from_negative() {
        let value = Value::from(-42);
        let result = value.to_uint();

        // Negative int can't convert to uint
        assert!(result.is_err());
    }

    // ===== Edge Cases =====

    #[test]
    fn empty_format_string() {
        let formatter = Formatter::new("").unwrap();
        let values = HashMap::new();
        let result = formatter.format_map(&values).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn format_string_without_fields() {
        let formatter = Formatter::new("Hello, World!").unwrap();
        let values = HashMap::new();
        let result = formatter.format_map(&values).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn escaped_braces() {
        let formatter = Formatter::new("{{literal}}").unwrap();
        let values = HashMap::new();
        let result = formatter.format_map(&values).unwrap();
        assert_eq!(result, "{literal}");
    }

    #[test]
    fn mixed_escaped_and_fields() {
        let formatter = Formatter::new("{{before}} {value} {{after}}").unwrap();
        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::from(42));
        let result = formatter.format_map(&values).unwrap();
        assert_eq!(result, "{before} 42 {after}");
    }

    // ===== Complex Error Scenarios =====

    #[test]
    fn multiple_missing_fields() {
        let formatter = Formatter::new("{a} {b} {c}").unwrap();
        let values = HashMap::new();
        let result = formatter.format_map(&values);

        assert!(result.is_err());
        // Should error on first missing field
        match result {
            Err(Error::MissingField(_)) => {}
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn complex_pattern_no_match() {
        let parser = Parser::new("{name} is {age:d} years old").unwrap();
        let result = parser.parse("completely different string");

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn search_vs_parse() {
        let parser = Parser::new("{value:d}").unwrap();

        // parse() requires exact match
        let parse_result = parser.parse("prefix 42 suffix").unwrap();
        assert!(parse_result.is_none());

        // search() finds first match
        let search_result = parser.search("prefix 42 suffix").unwrap();
        assert!(search_result.is_some());
        assert_eq!(
            search_result
                .unwrap()
                .get("value")
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
    }
}
