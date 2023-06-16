use super::Emitter;
use std::{
    cell::RefCell,
    iter,
};

pub struct Iterator<R>
{
    iter: RefCell<Box<dyn iter::Iterator<Item = R>>>,
}

impl<R> Iterator<R>
{
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
