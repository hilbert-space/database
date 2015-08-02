use std::default::Default;

use language::description::Description;
use {Result, Type};

/// A column description.
#[derive(Clone, Debug, Default)]
pub struct Column {
    name: Option<String>,
    kind: Option<Type>,
}

impl Column {
    /// Create a description.
    #[inline]
    pub fn new() -> Column {
        Default::default()
    }

    /// Set the name.
    pub fn name<T: ToString>(mut self, value: T) -> Self {
        self.name = Some(value.to_string());
        self
    }

    /// Set the type.
    pub fn kind(mut self, value: Type) -> Self {
        self.kind = Some(value);
        self
    }
}

impl Description for Column {
    fn compile(mut self) -> Result<String> {
        let kind = match take!(self, kind) {
            Type::Binary => "BLOB",
            Type::Float => "REAL",
            Type::Integer => "INTEGER",
            Type::String => "TEXT",
        };
        Ok(format!("`{}` {}", take!(self, name), kind))
    }
}
