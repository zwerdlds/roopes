//! Provides an encapsulated unit of execution.

pub mod heap;
pub mod lambda;

pub use heap::Heap;
pub use lambda::Lambda;

#[cfg(test)]
mod tests;

/// Encapsulates a block of execution which may be
/// run zero-or-more times.
pub trait Executable
{
    /// Run the unit of execution.
    fn execute(&self);
}

/// Exposes the [`Executable`] type at the library
/// level.
pub mod prelude
{
    pub use super::Executable;
}
