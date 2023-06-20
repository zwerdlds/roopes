//! Provides types which receive a borrowed value, and return a new, owned
//! value.

pub mod heap;
pub mod lambda;

pub use heap::Heap;
pub use lambda::Lambda;

/// A [`Transformer`] receives a borrowed value and creates a new value of a
/// possibly different type, giving ownership to the caller.
pub trait Transformer<I, O>
{
    /// Performs the transformation to produce the output, giving
    /// ownership of the new value to the caller.
    fn transform(
        &self,
        input: &I,
    ) -> O;
}

/// Exposes the [`Transformer`] type at the library level.
pub mod prelude
{
    pub use super::Transformer;
}

#[cfg(test)]
mod tests;
