//! Provides the abstraction for general object creation.

pub mod lambda;

pub use lambda::Lambda;

/// This trait enables the abstraction of the creation of objects.
pub trait AbstractFactory<T>
{
    /// Executes the creation logic and provides the type back to the caller.
    fn create(&self) -> T;
}

/// Exposes the [`AbstractFactory`] types at the library level.
pub mod prelude
{
    pub use super::AbstractFactory;
}
