use crate::prelude::*;
use delegate::delegate;

pub struct Heap
{
    delegate: Box<dyn Command>,
}

impl Heap
{
    #[must_use]
    pub fn new(delegate: Box<dyn Command>) -> Heap
    {
        Self { delegate }
    }
}

impl Command for Heap
{
    delegate! {
        to self.delegate {
           fn execute(&self);
        }
    }
}
