//! Provides a heap-based implementation of [`Command`].

use crate::prelude::*;
use delegate::delegate;

/// Stores a delegate [`Command`] in a [`Box`] for later use.
pub struct Heap
{
    delegate: Box<dyn Command>,
}

impl Heap
{
    /// Creates a new [`Heap`] with the supplied delegate.
    #[must_use]
    pub fn new(delegate: Box<dyn Command>) -> Heap
    {
        Self { delegate }
    }
}

#[allow(clippy::inline_always)]
impl Command for Heap
{
    delegate! {
        to self.delegate {
           fn execute(&self);
        }
    }
}
