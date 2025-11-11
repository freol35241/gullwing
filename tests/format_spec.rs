//! Comprehensive tests for format specification parsing.

use gullwing::spec::{Alignment, FormatSpec, Grouping, Sign, TypeSpec};

#[test]
fn test_empty_format_spec() {
    let spec = FormatSpec::parse("").unwrap();
    assert_eq!(spec.fill, None);
    assert_eq!(spec.align, None);
    assert_eq!(spec.width, None);
    assert_eq!(spec.precision, None);
    assert_eq!(spec.type_spec, None);
}

// Alignment tests
#[test]
fn test_left_align() {
    let spec = FormatSpec::parse("<").unwrap();
    assert_eq!(spec.align, Some(Alignment::Left));
}

#[test]
fn test_right_align() {
    let spec = FormatSpec::parse(">").unwrap();
    assert_eq!(spec.align, Some(Alignment::Right));
}

#[test]
fn test_center_align() {
    let spec = FormatSpec::parse("^").unwrap();
    assert_eq!(spec.align, Some(Alignment::Center));
}

#[test]
fn test_after_sign_align() {
    let spec = FormatSpec::parse("=").unwrap();
    assert_eq!(spec.align, Some(Alignment::AfterSign));
}

// Fill and align tests
#[test]
fn test_fill_with_align() {
    let spec = FormatSpec::parse("*<").unwrap();
    assert_eq!(spec.fill, Some('*'));
    assert_eq!(spec.align, Some(Alignment::Left));

    let spec = FormatSpec::parse("0>").unwrap();
    assert_eq!(spec.fill, Some('0'));
    assert_eq!(spec.align, Some(Alignment::Right));

    let spec = FormatSpec::parse("-^").unwrap();
    assert_eq!(spec.fill, Some('-'));
    assert_eq!(spec.align, Some(Alignment::Center));

    let spec = FormatSpec::parse("_=").unwrap();
    assert_eq!(spec.fill, Some('_'));
    assert_eq!(spec.align, Some(Alignment::AfterSign));
}

// Sign tests
#[test]
fn test_plus_sign() {
    let spec = FormatSpec::parse("+").unwrap();
    assert_eq!(spec.sign, Some(Sign::Plus));
}

#[test]
fn test_minus_sign() {
    let spec = FormatSpec::parse("-").unwrap();
    assert_eq!(spec.sign, Some(Sign::Minus));
}

#[test]
fn test_space_sign() {
    let spec = FormatSpec::parse(" ").unwrap();
    assert_eq!(spec.sign, Some(Sign::Space));
}

// Alternate form tests
#[test]
fn test_alternate_form() {
    let spec = FormatSpec::parse("#").unwrap();
    assert!(spec.alternate);

    let spec = FormatSpec::parse("#x").unwrap();
    assert!(spec.alternate);
    assert_eq!(spec.type_spec, Some(TypeSpec::HexLower));
}

// Zero padding tests
#[test]
fn test_zero_pad() {
    let spec = FormatSpec::parse("0").unwrap();
    assert!(spec.zero_pad);

    let spec = FormatSpec::parse("05d").unwrap();
    assert!(spec.zero_pad);
    assert_eq!(spec.width, Some(5));
    assert_eq!(spec.type_spec, Some(TypeSpec::Decimal));
}

// Width tests
#[test]
fn test_width() {
    let spec = FormatSpec::parse("10").unwrap();
    assert_eq!(spec.width, Some(10));

    let spec = FormatSpec::parse("5").unwrap();
    assert_eq!(spec.width, Some(5));

    let spec = FormatSpec::parse("100").unwrap();
    assert_eq!(spec.width, Some(100));
}

// Grouping tests
#[test]
fn test_comma_grouping() {
    let spec = FormatSpec::parse(",").unwrap();
    assert_eq!(spec.grouping, Some(Grouping::Comma));

    let spec = FormatSpec::parse("10,d").unwrap();
    assert_eq!(spec.width, Some(10));
    assert_eq!(spec.grouping, Some(Grouping::Comma));
    assert_eq!(spec.type_spec, Some(TypeSpec::Decimal));
}

#[test]
fn test_underscore_grouping() {
    let spec = FormatSpec::parse("_").unwrap();
    assert_eq!(spec.grouping, Some(Grouping::Underscore));

    let spec = FormatSpec::parse("10_d").unwrap();
    assert_eq!(spec.width, Some(10));
    assert_eq!(spec.grouping, Some(Grouping::Underscore));
    assert_eq!(spec.type_spec, Some(TypeSpec::Decimal));
}

// Precision tests
#[test]
fn test_precision() {
    let spec = FormatSpec::parse(".2").unwrap();
    assert_eq!(spec.precision, Some(2));

    let spec = FormatSpec::parse(".10").unwrap();
    assert_eq!(spec.precision, Some(10));

    let spec = FormatSpec::parse(".0").unwrap();
    assert_eq!(spec.precision, Some(0));
}

#[test]
fn test_precision_with_width() {
    let spec = FormatSpec::parse("10.2").unwrap();
    assert_eq!(spec.width, Some(10));
    assert_eq!(spec.precision, Some(2));
}

// Type specifier tests
#[test]
fn test_string_type() {
    let spec = FormatSpec::parse("s").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::String));
}

#[test]
fn test_integer_types() {
    let spec = FormatSpec::parse("b").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::Binary));

    let spec = FormatSpec::parse("c").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::Character));

    let spec = FormatSpec::parse("d").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::Decimal));

    let spec = FormatSpec::parse("o").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::Octal));

    let spec = FormatSpec::parse("x").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::HexLower));

    let spec = FormatSpec::parse("X").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::HexUpper));

    let spec = FormatSpec::parse("n").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::Number));
}

#[test]
fn test_float_types() {
    let spec = FormatSpec::parse("e").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::ExponentLower));

    let spec = FormatSpec::parse("E").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::ExponentUpper));

    let spec = FormatSpec::parse("f").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::FixedLower));

    let spec = FormatSpec::parse("F").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::FixedUpper));

    let spec = FormatSpec::parse("g").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::GeneralLower));

    let spec = FormatSpec::parse("G").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::GeneralUpper));

    let spec = FormatSpec::parse("%").unwrap();
    assert_eq!(spec.type_spec, Some(TypeSpec::Percentage));
}

// Complex format specs
#[test]
fn test_complex_float_spec() {
    let spec = FormatSpec::parse(">10.2f").unwrap();
    assert_eq!(spec.align, Some(Alignment::Right));
    assert_eq!(spec.width, Some(10));
    assert_eq!(spec.precision, Some(2));
    assert_eq!(spec.type_spec, Some(TypeSpec::FixedLower));
}

#[test]
fn test_complex_int_spec() {
    let spec = FormatSpec::parse("0=+10,d").unwrap();
    assert_eq!(spec.fill, Some('0'));
    assert_eq!(spec.align, Some(Alignment::AfterSign));
    assert_eq!(spec.sign, Some(Sign::Plus));
    assert_eq!(spec.width, Some(10));
    assert_eq!(spec.grouping, Some(Grouping::Comma));
    assert_eq!(spec.type_spec, Some(TypeSpec::Decimal));
}

#[test]
fn test_complex_hex_spec() {
    let spec = FormatSpec::parse("#010x").unwrap();
    assert!(spec.alternate);
    assert!(spec.zero_pad);
    assert_eq!(spec.width, Some(10));
    assert_eq!(spec.type_spec, Some(TypeSpec::HexLower));
}

#[test]
fn test_padded_string() {
    let spec = FormatSpec::parse("*<20s").unwrap();
    assert_eq!(spec.fill, Some('*'));
    assert_eq!(spec.align, Some(Alignment::Left));
    assert_eq!(spec.width, Some(20));
    assert_eq!(spec.type_spec, Some(TypeSpec::String));
}

// Python compatibility tests
#[test]
fn test_python_examples() {
    // From Python docs: https://docs.python.org/3/library/string.html#formatspec

    // Basic alignment
    let spec = FormatSpec::parse("<10").unwrap();
    assert_eq!(spec.align, Some(Alignment::Left));
    assert_eq!(spec.width, Some(10));

    // Centered with fill
    let spec = FormatSpec::parse("^20").unwrap();
    assert_eq!(spec.align, Some(Alignment::Center));
    assert_eq!(spec.width, Some(20));

    // Signed numbers
    let spec = FormatSpec::parse("+d").unwrap();
    assert_eq!(spec.sign, Some(Sign::Plus));
    assert_eq!(spec.type_spec, Some(TypeSpec::Decimal));

    // Hex with prefix
    let spec = FormatSpec::parse("#x").unwrap();
    assert!(spec.alternate);
    assert_eq!(spec.type_spec, Some(TypeSpec::HexLower));

    // Thousands separator
    let spec = FormatSpec::parse(",d").unwrap();
    assert_eq!(spec.grouping, Some(Grouping::Comma));
    assert_eq!(spec.type_spec, Some(TypeSpec::Decimal));

    // Float with precision
    let spec = FormatSpec::parse(".2f").unwrap();
    assert_eq!(spec.precision, Some(2));
    assert_eq!(spec.type_spec, Some(TypeSpec::FixedLower));

    // Percentage
    let spec = FormatSpec::parse(".1%").unwrap();
    assert_eq!(spec.precision, Some(1));
    assert_eq!(spec.type_spec, Some(TypeSpec::Percentage));
}

#[test]
fn test_zero_flag() {
    let spec = FormatSpec::parse("z").unwrap();
    assert!(spec.zero_flag);

    let spec = FormatSpec::parse("z.2f").unwrap();
    assert!(spec.zero_flag);
    assert_eq!(spec.precision, Some(2));
    assert_eq!(spec.type_spec, Some(TypeSpec::FixedLower));
}

// Edge cases
#[test]
fn test_all_flags() {
    let spec = FormatSpec::parse("0<+z#010,.2f").unwrap();
    assert_eq!(spec.fill, Some('0'));
    assert_eq!(spec.align, Some(Alignment::Left));
    assert_eq!(spec.sign, Some(Sign::Plus));
    assert!(spec.zero_flag);
    assert!(spec.alternate);
    assert!(spec.zero_pad);
    assert_eq!(spec.width, Some(10));
    assert_eq!(spec.grouping, Some(Grouping::Comma));
    assert_eq!(spec.precision, Some(2));
    assert_eq!(spec.type_spec, Some(TypeSpec::FixedLower));
}

#[test]
fn test_large_width_and_precision() {
    let spec = FormatSpec::parse("1000.999f").unwrap();
    assert_eq!(spec.width, Some(1000));
    assert_eq!(spec.precision, Some(999));
    assert_eq!(spec.type_spec, Some(TypeSpec::FixedLower));
}
