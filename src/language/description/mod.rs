//! Descriptions.

use language::Unit;

/// A description.
pub trait Description: Unit {
}

mod column;

pub use self::column::Column;
