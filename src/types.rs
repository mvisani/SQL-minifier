use std::fmt::Display;

/// SQL Sqltypes that can be abbreviated.
pub enum Sqltypes {
    /// INTEGER can be abbreviated as INT
    Integer,
    /// INTEGER with comma can be abbreviated as INT,
    IntegerComma,
    /// CHARACTER can be abbreviated as CHAR
    Character,
    /// CHARACTER with comma can be abbreviated as CHAR,
    CharacterComma,
    /// BOOLEAN can be abbreviated as BOOL
    Boolean,
    /// BOOLEAN with comma can be abbreviated as BOOL,
    BooleanComma,
    /// DECIMAL can be abbreviated as DEC
    Decimal,
    /// DECIMAL with comma can be abbreviated as DEC,
    DecimalComma,
}

impl Display for Sqltypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sqltypes::Integer => write!(f, "INT"),
            Sqltypes::Character => write!(f, "CHAR"),
            Sqltypes::Boolean => write!(f, "BOOL"),
            Sqltypes::Decimal => write!(f, "DEC"),
            Sqltypes::IntegerComma => write!(f, "INT,"),
            Sqltypes::CharacterComma => write!(f, "CHAR,"),
            Sqltypes::BooleanComma => write!(f, "BOOL,"),
            Sqltypes::DecimalComma => write!(f, "DEC,"),
        }
    }
}

impl TryFrom<&str> for Sqltypes {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "INTEGER" => Ok(Sqltypes::Integer),
            "CHARACTER" => Ok(Sqltypes::Character),
            "BOOLEAN" => Ok(Sqltypes::Boolean),
            "DECIMAL" => Ok(Sqltypes::Decimal),
            "INTEGER," => Ok(Sqltypes::IntegerComma),
            "CHARACTER," => Ok(Sqltypes::CharacterComma),
            "BOOLEAN," => Ok(Sqltypes::BooleanComma),
            "DECIMAL," => Ok(Sqltypes::DecimalComma),
            _ => Err("Invalid type"),
        }
    }
}

impl TryFrom<String> for Sqltypes {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "INTEGER" => Ok(Sqltypes::Integer),
            "CHARACTER" => Ok(Sqltypes::Character),
            "BOOLEAN" => Ok(Sqltypes::Boolean),
            "DECIMAL" => Ok(Sqltypes::Decimal),
            "INTEGER," => Ok(Sqltypes::IntegerComma),
            "CHARACTER," => Ok(Sqltypes::CharacterComma),
            "BOOLEAN," => Ok(Sqltypes::BooleanComma),
            "DECIMAL," => Ok(Sqltypes::DecimalComma),
            _ => Err("Invalid type"),
        }
    }
}
