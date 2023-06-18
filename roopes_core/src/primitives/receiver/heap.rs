use crate::prelude::*;
use delegate::delegate;

pub struct Heap<M>
{
    delegate: Box<dyn Receiver<M>>,
}

impl<M> Heap<M>
{
    #[must_use]
    pub fn new(delegate: Box<dyn Receiver<M>>) -> Heap<M>
    {
        Heap { delegate }
    }
}

impl<M> Receiver<M> for Heap<M>
{
    delegate! {
        to self.delegate {
           fn receive(&self, message: M);
        }
    }
}
