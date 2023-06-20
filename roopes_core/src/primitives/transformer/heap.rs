//! This module contains types which store a [`Transformer`] on the heap.  This
//! is particularly useful to allow for non-uniform-sized [`Transformer`] types.
//! The type erasure provided by `dyn` also allows [`Transformer`]s to be
//! non-uniform in a collection so long as they implement for the same generic
//! types.

use super::Transformer;
use delegate::delegate;

/// Stores an indirected [`Transformer`] in a [`Box`] for later use.
pub struct Heap<I, O>
{
    delegate: Box<dyn Transformer<I, O>>,
}

impl<I, O> Heap<I, O>
{
    /// Creates a new [`Heap`] which contains a [`Box`]ed [`Transformer`].
    #[must_use]
    pub fn new(delegate: Box<dyn Transformer<I, O>>) -> Heap<I, O>
    {
        Heap { delegate }
    }
}

#[allow(clippy::inline_always)]
impl<I, O> Transformer<I, O> for Heap<I, O>
{
    delegate! {
        to self.delegate {
           fn transform(&self, input: &I) -> O;
        }
    }
}
