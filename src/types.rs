use std::fmt::Display;

/// SQL Sqltypes that can be abbreviated.
pub enum Sqltypes {
    /// INTEGER can be abbreviated as INT
    Integer,
    /// CHARACTER can be abbreviated as CHAR
    Character,
    /// BOOLEAN can be abbreviated as BOOL
    Boolean,
    /// DECIMAL can be abbreviated as DEC
    Decimal,
}

impl Display for Sqltypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sqltypes::Integer => write!(f, "INT"),
            Sqltypes::Character => write!(f, "CHAR"),
            Sqltypes::Boolean => write!(f, "BOOL"),
            Sqltypes::Decimal => write!(f, "DEC"),
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
            _ => Err("Invalid type"),
        }
    }
}
