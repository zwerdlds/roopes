//! Provides an encapsulated unit of production.

pub mod iterator;
pub mod lambda;

pub use iterator::Iterator;
pub use lambda::Lambda;

/// The [`Emitter`] [`Delegate`] must be capable
/// of returning potentially unlimited owned
/// values.
pub trait Delegate<O> = Fn() -> O;

/// [`Emitter`]s are capable of continuously
/// returning an owned object to the
/// caller.  [`Iterator`] is a subset of this
/// interface in that [`Emitter<Option<T>>`] is
/// equivalent to [`Iterator<T>`].
pub trait Emitter<O>
{
    /// Produces a value.
    fn emit(&self) -> O;
}

/// Exposes the [`Emitter`] type at the library
/// level.
pub mod prelude
{
    pub use super::Emitter;
}

#[cfg(test)]
mod tests;
