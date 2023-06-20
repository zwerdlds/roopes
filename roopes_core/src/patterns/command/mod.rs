//! Contains types which encapsulate a repeatedly callable block of code.

pub mod executable;
pub mod hashable;
pub mod heap;

pub use executable::Executable;
pub use hashable::Hashable;
pub use heap::Heap;

/// Encapsulates a repeatedly callable block of code.
pub trait Command
{
    /// Calls the encapsulated block of code.
    fn execute(&self);
}

/// Exposes the [`Command`] type at the library level.
pub mod prelude
{
    pub use super::Command;
}
