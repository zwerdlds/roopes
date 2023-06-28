//! Provides a heap-based implementation of
//! [`Command`].

use crate::{
    prelude::*,
    primitives,
};
use delegate::delegate;

/// Stores a delegate [`Command`] in a [`Box`] for
/// later use.
pub struct Heap
{
    delegate: Box<dyn Command>,
}

impl Heap
{
    /// Creates a new [`Heap`] with the supplied
    /// delegate.
    #[must_use]
    pub fn new(delegate: Box<dyn Command>) -> Heap
    {
        Heap { delegate }
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

impl From<Heap> for Box<dyn Command>
{
    fn from(value: Heap) -> Self
    {
        value.delegate
    }
}

impl From<Box<dyn Command>> for Heap
{
    fn from(delegate: Box<dyn Command>) -> Self
    {
        Heap { delegate }
    }
}

impl<D> From<D> for Heap
where
    D: primitives::executable::lambda::Delegate + 'static,
{
    fn from(delegate: D) -> Self
    {
        let delegate =
            Box::new(CommandExecutable::new(executable::Lambda::new(delegate)));

        Heap { delegate }
    }
}
