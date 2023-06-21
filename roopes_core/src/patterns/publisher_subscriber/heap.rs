//! Provides a heap-based [`Subscriber`] which
//! redirects
//! [`publisher_subscriber::Subscriber::receive`]
//! calls to a delegate [`Subscriber`].

use crate::{
    patterns::publisher_subscriber,
    prelude::*,
};
use delegate::delegate;

/// Holds a reference to a delegate [`Subscriber`]
/// in a [`Box`]ed delegate for later calls to
/// [`publisher_subscriber::Subscriber::receive`].
pub struct Subscriber<M>
{
    delegate: Box<dyn publisher_subscriber::Subscriber<M>>,
}

impl<M> Subscriber<M>
{
    /// Creates a new, heap-based, [`Subscriber`].
    #[must_use]
    pub fn new(
        delegate: Box<dyn publisher_subscriber::Subscriber<M>>
    ) -> Subscriber<M>
    {
        Self { delegate }
    }
}

#[allow(clippy::inline_always)]
impl<M> publisher_subscriber::Subscriber<M> for Subscriber<M>
{
    delegate! {
        to self.delegate {
           fn receive(&self, message: &M);
        }
    }
}
