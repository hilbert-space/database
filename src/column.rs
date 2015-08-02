//! Columns.

/// A column.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Column {
    /// The name.
    pub name: String,
    /// The type.
    pub kind: Type,
}

/// A column type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    /// The binary type.
    Binary,
    /// The floating-point type.
    Float,
    /// The integer type.
    Integer,
    /// The string type.
    String,
}

/// A column value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Binary data.
    Binary(Vec<u8>),
    /// A floating-point number.
    Float(f64),
    /// An integer.
    Integer(i64),
    /// A string.
    String(String),
}
