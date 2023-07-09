//! This module enables conversion between [`Observer`] and [`Executable`].
use crate::prelude::*;

/// Wraps an [`Observer`] such that calls to [`Executable::execute`] are
/// forwarded to [`Observer::notify`].
pub struct ExecutableObserver<O>
where
    O: Observer,
{
    delegate: O,
}

impl<O> Observer for ExecutableObserver<O>
where
    O: Observer,
{
    fn notify(&self)
    {
        self.delegate.notify();
    }
}

impl<O> Executable for ExecutableObserver<O>
where
    O: Observer,
{
    fn execute(&self)
    {
        self.delegate.notify();
    }
}

/// Provides types which are exposed at the
/// library level.
pub mod prelude
{
    pub use super::ExecutableObserver;
}
