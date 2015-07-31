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
    /// Floating-point data.
    Float,
    /// Integer data.
    Integer,
    /// Textual data.
    Text,
}
