use Result;
use column::{Column, ColumnValue};
use driver::{Driver, Statement};
use operation::{Insert, Operation};
use table::{self, Table};

/// A writer.
pub struct Writer<T: Driver> {
    statement: T::Statement,
}

impl<T: Driver> Writer<T> {
    /// Create a writer.
    #[inline]
    pub fn new(table: &Table<T>) -> Result<Self> {
        Writer::with_columns(table, table.columns())
    }

    /// Create a writer for a subset of columns.
    pub fn with_columns(table: &Table<T>, columns: &[Column]) -> Result<Self> {
        let driver = table::driver(table);
        let mut operation = Insert::new();
        operation.table(table.name());
        for &Column { ref name, .. } in columns {
            operation.column(name);
        }
        operation.multiplex(1);
        Ok(Writer { statement: try!(driver.prepare(try!(operation.compile()))) })
    }

    /// Write data.
    pub fn write(&mut self, values: &[ColumnValue]) -> Result<()> {
        self.statement.execute(values)
    }
}
