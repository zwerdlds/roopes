//! Provides a heap-based [`Handler`] which redirects [`Handler::handle`] calls
//! to a delegate [`Handler`].  Useful when redirecting calls to unknown or
//! mixed lists of [`Handler`]s.

use crate::prelude::*;
use delegate::delegate;

/// Stores an indirected [`Handler`] in a [`Box`] for later delegation.
pub struct Heap<M>
{
    delegate: Box<dyn Handler<M>>,
}

impl<M> Heap<M>
{
    /// Creates a new [`Heap`] with a given [`Box`]ed [`Handler`].
    /// # Examples
    /// ``` rust
    /// use roopes::prelude::*;
    /// let my_handler = handler::Heap::new(Box::new(
    ///     handler::Lambda::new(|message| {
    ///         println!("{message}");
    ///     }),
    /// ));
    /// my_handler.handle(&"Hello World.".to_string());
    /// ```
    #[must_use]
    pub fn new(delegate: Box<dyn Handler<M>>) -> Heap<M>
    {
        Heap { delegate }
    }
}

#[allow(clippy::inline_always)]
impl<M> Handler<M> for Heap<M>
{
    delegate! {
        to self.delegate {
           fn handle(&self, message: &M);
        }
    }
}
