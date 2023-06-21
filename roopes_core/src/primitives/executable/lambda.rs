//! Provides a simple wrapper struct around
//! [`Delegate`], `Fn()` types.
use super::Executable;

/// An [`Executable`] [`Delegate`] takes no
/// parameters and returns nothing.
pub trait Delegate = Fn();

/// Defines an encapsulated [`Executable`] as a
/// struct, which just delegates its execution to
/// the attached [`Delegate`].
pub struct Lambda<D>
where
    D: Delegate,
{
    delegate: D,
}

impl<D> Lambda<D>
where
    D: Delegate,
{
    /// Creates a [`Lambda`] from a given
    /// [`Delegate`].
    pub fn new(delegate: D) -> Self
    {
        Self { delegate }
    }
}

impl<D> Executable for Lambda<D>
where
    D: Delegate,
{
    fn execute(&self)
    {
        (self.delegate)();
    }
}
