/// A column.
#[derive(Clone, Debug)]
pub struct Column {
    /// The name.
    pub name: String,
    /// The type.
    pub kind: ColumnKind,
}

/// A column type.
#[derive(Clone, Copy, Debug)]
pub enum ColumnKind {
    /// The floating-point type.
    Float,
    /// The integer type.
    Integer,
    /// The textual type.
    Text,
}

/// A column value.
#[derive(Clone, Debug)]
pub enum ColumnValue {
    /// A floating-point value.
    Float(f64),
    /// An integer value.
    Integer(i64),
    /// A textual value.
    Text(String),
}
