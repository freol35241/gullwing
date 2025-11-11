//! Types for format specification components.

/// Alignment option for format specifications.
///
/// See: <https://docs.python.org/3/library/string.html#formatspec>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Left-aligned: `<`
    Left,
    /// Right-aligned: `>`
    Right,
    /// Center-aligned: `^`
    Center,
    /// Sign-aware padding (numeric types only): `=`
    AfterSign,
}

impl Alignment {
    /// Parse an alignment character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '<' => Some(Alignment::Left),
            '>' => Some(Alignment::Right),
            '^' => Some(Alignment::Center),
            '=' => Some(Alignment::AfterSign),
            _ => None,
        }
    }

    /// Convert to character representation.
    pub fn to_char(self) -> char {
        match self {
            Alignment::Left => '<',
            Alignment::Right => '>',
            Alignment::Center => '^',
            Alignment::AfterSign => '=',
        }
    }
}

/// Sign option for numeric format specifications.
///
/// See: <https://docs.python.org/3/library/string.html#formatspec>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    /// Only show sign for negative numbers: `-` (default)
    Minus,
    /// Always show sign: `+`
    Plus,
    /// Use space for positive, minus for negative: ` `
    Space,
}

impl Sign {
    /// Parse a sign character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '-' => Some(Sign::Minus),
            '+' => Some(Sign::Plus),
            ' ' => Some(Sign::Space),
            _ => None,
        }
    }

    /// Convert to character representation.
    pub fn to_char(self) -> char {
        match self {
            Sign::Minus => '-',
            Sign::Plus => '+',
            Sign::Space => ' ',
        }
    }
}

/// Grouping option for numeric format specifications.
///
/// See: <https://docs.python.org/3/library/string.html#formatspec>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grouping {
    /// Comma separator (every 3 digits): `,`
    Comma,
    /// Underscore separator (every 3 for decimal, 4 for binary/octal/hex): `_`
    Underscore,
}

impl Grouping {
    /// Parse a grouping character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            ',' => Some(Grouping::Comma),
            '_' => Some(Grouping::Underscore),
            _ => None,
        }
    }

    /// Convert to character representation.
    pub fn to_char(self) -> char {
        match self {
            Grouping::Comma => ',',
            Grouping::Underscore => '_',
        }
    }
}

/// Type specifier for format specifications.
///
/// See: <https://docs.python.org/3/library/string.html#formatspec>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeSpec {
    /// String (default): `s`
    String,
    /// Binary integer: `b`
    Binary,
    /// Character (from integer code): `c`
    Character,
    /// Decimal integer: `d`
    Decimal,
    /// Octal integer: `o`
    Octal,
    /// Hexadecimal integer (lowercase): `x`
    HexLower,
    /// Hexadecimal integer (uppercase): `X`
    HexUpper,
    /// Integer with locale-aware formatting: `n`
    Number,
    /// Scientific notation (lowercase e): `e`
    ExponentLower,
    /// Scientific notation (uppercase E): `E`
    ExponentUpper,
    /// Fixed-point (lowercase): `f`
    FixedLower,
    /// Fixed-point (uppercase): `F`
    FixedUpper,
    /// General format (lowercase): `g`
    GeneralLower,
    /// General format (uppercase): `G`
    GeneralUpper,
    /// Percentage: `%`
    Percentage,
}

impl TypeSpec {
    /// Parse a type specifier character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            's' => Some(TypeSpec::String),
            'b' => Some(TypeSpec::Binary),
            'c' => Some(TypeSpec::Character),
            'd' => Some(TypeSpec::Decimal),
            'o' => Some(TypeSpec::Octal),
            'x' => Some(TypeSpec::HexLower),
            'X' => Some(TypeSpec::HexUpper),
            'n' => Some(TypeSpec::Number),
            'e' => Some(TypeSpec::ExponentLower),
            'E' => Some(TypeSpec::ExponentUpper),
            'f' => Some(TypeSpec::FixedLower),
            'F' => Some(TypeSpec::FixedUpper),
            'g' => Some(TypeSpec::GeneralLower),
            'G' => Some(TypeSpec::GeneralUpper),
            '%' => Some(TypeSpec::Percentage),
            _ => None,
        }
    }

    /// Convert to character representation.
    pub fn to_char(self) -> char {
        match self {
            TypeSpec::String => 's',
            TypeSpec::Binary => 'b',
            TypeSpec::Character => 'c',
            TypeSpec::Decimal => 'd',
            TypeSpec::Octal => 'o',
            TypeSpec::HexLower => 'x',
            TypeSpec::HexUpper => 'X',
            TypeSpec::Number => 'n',
            TypeSpec::ExponentLower => 'e',
            TypeSpec::ExponentUpper => 'E',
            TypeSpec::FixedLower => 'f',
            TypeSpec::FixedUpper => 'F',
            TypeSpec::GeneralLower => 'g',
            TypeSpec::GeneralUpper => 'G',
            TypeSpec::Percentage => '%',
        }
    }

    /// Check if this is a numeric type specifier.
    pub fn is_numeric(self) -> bool {
        matches!(
            self,
            TypeSpec::Binary
                | TypeSpec::Decimal
                | TypeSpec::Octal
                | TypeSpec::HexLower
                | TypeSpec::HexUpper
                | TypeSpec::Number
                | TypeSpec::ExponentLower
                | TypeSpec::ExponentUpper
                | TypeSpec::FixedLower
                | TypeSpec::FixedUpper
                | TypeSpec::GeneralLower
                | TypeSpec::GeneralUpper
                | TypeSpec::Percentage
        )
    }

    /// Check if this is an integer type specifier.
    pub fn is_integer(self) -> bool {
        matches!(
            self,
            TypeSpec::Binary
                | TypeSpec::Character
                | TypeSpec::Decimal
                | TypeSpec::Octal
                | TypeSpec::HexLower
                | TypeSpec::HexUpper
                | TypeSpec::Number
        )
    }

    /// Check if this is a float type specifier.
    pub fn is_float(self) -> bool {
        matches!(
            self,
            TypeSpec::ExponentLower
                | TypeSpec::ExponentUpper
                | TypeSpec::FixedLower
                | TypeSpec::FixedUpper
                | TypeSpec::GeneralLower
                | TypeSpec::GeneralUpper
                | TypeSpec::Percentage
        )
    }
}
