use sqlite;
use std::mem;
use std::path::Path;

use Result;
use column::ColumnValue;
use driver;

/// A SQLite connection.
pub struct Driver<'l> {
    backend: sqlite::Connection<'l>,
}

/// An SQLite prepared statement.
pub struct Statement<'l> {
    backend: sqlite::Statement<'l>,
}

impl<'l> driver::Driver for Driver<'l> {
    type Statement = Statement<'l>;

    fn connect<T: AsRef<Path>>(path: T) -> Result<Self> {
        Ok(Driver { backend: ok!(sqlite::open(path)) })
    }

    #[inline]
    fn execute<T: AsRef<str>>(&self, query: T) -> Result<()> {
        ok!(self.backend.execute(query));
        Ok(())
    }

    #[inline]
    fn prepare<T: AsRef<str>>(&self, query: T) -> Result<Statement<'l>> {
        Ok(Statement { backend: unsafe { mem::transmute(ok!(self.backend.prepare(query))) } })
    }
}

impl<'l> driver::Statement for Statement<'l> {
    fn execute(&mut self, values: &[ColumnValue]) -> Result<()> {
        use sqlite::State;

        ok!(self.backend.reset());
        for (mut i, value) in values.iter().enumerate() {
            i += 1;
            match value {
                &ColumnValue::Float(value) => ok!(self.backend.bind(i, value)),
                &ColumnValue::Integer(value) => ok!(self.backend.bind(i, value)),
                &ColumnValue::Text(ref value) => ok!(self.backend.bind(i, &value[..])),
            }
        }

        match ok!(self.backend.step()) {
            State::Done => Ok(()),
            _ => raise!("failed to write into the database"),
        }
    }
}
