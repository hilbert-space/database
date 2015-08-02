//! Statements.

use Result;

/// A statement.
pub trait Statement {
    /// Compile the statement.
    fn compile(self) -> Result<String>;
}

impl<T: ToString> Statement for T {
    #[inline]
    fn compile(self) -> Result<String> {
        Ok(self.to_string())
    }
}

mod create_table;
mod insert_into;
mod select;

pub use self::create_table::{Column, CreateTable};
pub use self::insert_into::InsertInto;
pub use self::select::Select;

shortcut!(
    #[doc = "Create a `CREATE TABLE` statement."]
    create_table -> CreateTable,

    #[doc = "Create an `INSERT INTO` statement."]
    insert_into -> InsertInto,

    #[doc = "Create a `SELECT` statement."]
    select -> Select,
);
