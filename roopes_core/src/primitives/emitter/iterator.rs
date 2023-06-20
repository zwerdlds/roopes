//! Provides a simple wrapper struct around [`iter::Iterator`].
use super::Emitter;
use std::{
    cell::RefCell,
    iter,
};

/// Wraps an [`iter::Iterator`] in an [`Emitter<Option>`].
pub struct Iterator<R>
{
    iter: RefCell<Box<dyn iter::Iterator<Item = R>>>,
}

impl<R> Iterator<R>
{
    /// Creates a new [`Iterator`] with a given [`Box`]ed [`iter::Iterator`].
    pub fn new(iter: RefCell<Box<dyn iter::Iterator<Item = R>>>)
        -> Iterator<R>
    {
        Iterator { iter }
    }
}

impl<R> Emitter<Option<R>> for Iterator<R>
{
    fn emit(&self) -> Option<R>
    {
        self.iter.borrow_mut().next()
    }
}
