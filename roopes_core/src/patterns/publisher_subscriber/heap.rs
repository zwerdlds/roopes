//! Provides a heap-based [`Subscriber`] which
//! redirects
//! [`publisher_subscriber::Subscriber::receive`]
//! calls to a delegate [`Subscriber`].

use crate::{
    patterns::publisher_subscriber,
    prelude::{
        handler::lambda::Delegate,
        *,
    },
};
use delegate::delegate;
use std::fmt::{
    Debug,
    Write,
};

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

impl<M> Debug for Subscriber<M>
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    {
        f.debug_struct("Subscriber").finish()
    }
}
