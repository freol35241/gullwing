use gullwing::{Formatter, Parser, Value};
use proptest::prelude::*;
use std::collections::HashMap;

#[cfg(test)]
mod roundtrip_tests {
    use super::*;

    proptest! {
        /// Test that integers can be formatted and parsed back to the same value
        #[test]
        fn roundtrip_decimal_int(n in -1000000i64..1000000i64) {
            let formatter = Formatter::new("{value:d}").unwrap();
            let parser = Parser::new("{value:d}").unwrap();

            let mut values = HashMap::new();
            values.insert("value".to_string(), Value::from(n));

            let formatted = formatter.format_map(&values).unwrap();
            let parsed = parser.parse(&formatted).unwrap().unwrap();

            prop_assert_eq!(parsed.get("value").unwrap().as_int().unwrap(), n);
        }

        /// Test that floats can be formatted and parsed back (with precision loss consideration)
        #[test]
        fn roundtrip_fixed_float(n in -1000.0f64..1000.0f64) {
            // Skip NaN and infinity
            if !n.is_finite() {
                return Ok(());
            }

            let formatter = Formatter::new("{value:.2f}").unwrap();
            let parser = Parser::new("{value:f}").unwrap();

            let mut values = HashMap::new();
            values.insert("value".to_string(), Value::from(n));

            let formatted = formatter.format_map(&values).unwrap();
            let parsed = parser.parse(&formatted).unwrap().unwrap();
            let parsed_value = parsed.get("value").unwrap().as_float().unwrap();

            // Allow small difference due to precision
            prop_assert!((parsed_value - n).abs() < 0.01, "Expected ~{}, got {}", n, parsed_value);
        }

        /// Test that hex integers can be formatted and parsed back
        #[test]
        fn roundtrip_hex(n in 0u64..1000000u64) {
            let formatter = Formatter::new("{value:x}").unwrap();
            let parser = Parser::new("{value:x}").unwrap();

            let mut values = HashMap::new();
            values.insert("value".to_string(), Value::from(n));

            let formatted = formatter.format_map(&values).unwrap();
            let parsed = parser.parse(&formatted).unwrap().unwrap();

            prop_assert_eq!(parsed.get("value").unwrap().as_uint().unwrap(), n);
        }

        /// Test that binary integers can be formatted and parsed back
        #[test]
        fn roundtrip_binary(n in 0u64..10000u64) {
            let formatter = Formatter::new("{value:b}").unwrap();
            let parser = Parser::new("{value:b}").unwrap();

            let mut values = HashMap::new();
            values.insert("value".to_string(), Value::from(n));

            let formatted = formatter.format_map(&values).unwrap();
            let parsed = parser.parse(&formatted).unwrap().unwrap();

            prop_assert_eq!(parsed.get("value").unwrap().as_uint().unwrap(), n);
        }

        /// Test that octal integers can be formatted and parsed back
        #[test]
        fn roundtrip_octal(n in 0u64..10000u64) {
            let formatter = Formatter::new("{value:o}").unwrap();
            let parser = Parser::new("{value:o}").unwrap();

            let mut values = HashMap::new();
            values.insert("value".to_string(), Value::from(n));

            let formatted = formatter.format_map(&values).unwrap();
            let parsed = parser.parse(&formatted).unwrap().unwrap();

            prop_assert_eq!(parsed.get("value").unwrap().as_uint().unwrap(), n);
        }
    }

    #[test]
    fn roundtrip_string() {
        let test_strings = vec![
            "hello",
            "world",
            "test123",
            "with-dashes",
            "with_underscores",
        ];

        for s in test_strings {
            let formatter = Formatter::new("{value}").unwrap();
            let parser = Parser::new("{value}").unwrap();

            let mut values = HashMap::new();
            values.insert("value".to_string(), Value::from(s));

            let formatted = formatter.format_map(&values).unwrap();
            let parsed = parser.parse(&formatted).unwrap().unwrap();

            assert_eq!(parsed.get("value").unwrap().as_str().unwrap(), s);
        }
    }

    #[test]
    fn roundtrip_multiple_fields() {
        let formatter = Formatter::new("{name} {age:d} {score:.1f}").unwrap();
        let parser = Parser::new("{name} {age:d} {score:f}").unwrap();

        let mut values = HashMap::new();
        values.insert("name".to_string(), Value::from("Alice"));
        values.insert("age".to_string(), Value::from(30));
        values.insert("score".to_string(), Value::from(95.7));

        let formatted = formatter.format_map(&values).unwrap();
        let parsed = parser.parse(&formatted).unwrap().unwrap();

        assert_eq!(parsed.get("name").unwrap().as_str().unwrap(), "Alice");
        assert_eq!(parsed.get("age").unwrap().as_int().unwrap(), 30);
        assert!((parsed.get("score").unwrap().as_float().unwrap() - 95.7).abs() < 0.01);
    }

    #[test]
    fn roundtrip_with_alignment() {
        // Right-aligned numbers
        let formatter = Formatter::new("{value:>10d}").unwrap();
        let parser = Parser::new("{value:d}").unwrap();

        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::from(42));

        let formatted = formatter.format_map(&values).unwrap();
        assert_eq!(formatted, "        42");

        let parsed = parser.parse(formatted.trim()).unwrap().unwrap();
        assert_eq!(parsed.get("value").unwrap().as_int().unwrap(), 42);
    }

    #[test]
    fn roundtrip_with_padding() {
        // Zero-padded numbers
        let formatter = Formatter::new("{value:05d}").unwrap();
        let parser = Parser::new("{value:d}").unwrap();

        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::from(42));

        let formatted = formatter.format_map(&values).unwrap();
        assert_eq!(formatted, "00042");

        let parsed = parser.parse(&formatted).unwrap().unwrap();
        assert_eq!(parsed.get("value").unwrap().as_int().unwrap(), 42);
    }

    #[test]
    fn roundtrip_scientific_notation() {
        let formatter = Formatter::new("{value:e}").unwrap();
        let parser = Parser::new("{value:e}").unwrap();

        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::from(1234.0));

        let formatted = formatter.format_map(&values).unwrap();
        let parsed = parser.parse(&formatted).unwrap().unwrap();
        let parsed_value = parsed.get("value").unwrap().as_float().unwrap();

        assert!((parsed_value - 1234.0).abs() < 0.01);
    }
}
