use crate::{
    patterns::publisher_subscriber,
    prelude::*,
};
use delegate::delegate;

pub struct Subscriber<M>
{
    delegate: Box<dyn publisher_subscriber::Subscriber<M>>,
}

impl<M> Subscriber<M>
{
    #[must_use]
    pub fn new(
        delegate: Box<dyn publisher_subscriber::Subscriber<M>>
    ) -> Subscriber<M>
    {
        Self { delegate }
    }
}

impl<M> publisher_subscriber::Subscriber<M> for Subscriber<M>
{
    delegate! {
        to self.delegate {
           fn receive(&self, message: &M);
        }
    }
}
