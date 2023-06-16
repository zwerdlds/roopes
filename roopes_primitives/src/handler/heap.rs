use crate::prelude::*;
use delegate::delegate;

pub struct Heap<M>
{
    delegate: Box<dyn Handler<M>>,
}

impl<M> Heap<M>
{
    #[must_use]
    pub fn new(delegate: Box<dyn Handler<M>>) -> Heap<M>
    {
        Heap { delegate }
    }
}

impl<M> Handler<M> for Heap<M>
{
    delegate! {
        to self.delegate {
           fn handle(&self, message: &M);
        }
    }
}
