//! Provides a heap-based [`Executable`] which
//! redirects [`Executable::execute`] calls to a
//! delegate [`Executable`].  Useful when
//! redirecting calls to unknown or mixed lists of
//! [`Executable`]s.

use super::Executable;

/// Stores an indirected [`Executable`] in a
/// [`Box`] for later delegation.
pub struct Heap
{
    delegate: Box<dyn super::Executable>,
}

impl Heap
{
    /// Creates a new [`Heap`] with a given
    /// [`Box`]ed [`Executable`]. # Examples
    /// ``` rust
    /// use roopes::prelude::*;
    /// let my_executable =
    ///     executable::Heap::new(Box::new(executable::Lambda::new(|| {
    ///         println!("Hello World.");
    ///     })));
    /// my_executable.execute();
    /// ```
    #[must_use]
    pub fn new(delegate: Box<dyn super::Executable>) -> Self
    {
        Self { delegate }
    }
}

impl Executable for Heap
{
    fn execute(&self)
    {
        self.delegate.execute();
    }
}
