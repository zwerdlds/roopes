use crate::prelude::*;
use std::marker::PhantomData;

pub struct SubscribingHandler<H, M>
where
    H: Handler<M>,
{
    handler: H,
    _retain_types: PhantomData<M>,
}

impl<H, M> SubscribingHandler<H, M>
where
    H: Handler<M>,
{
    pub fn new(handler: H) -> SubscribingHandler<H, M>
    {
        SubscribingHandler {
            handler,
            _retain_types: PhantomData::default(),
        }
    }
}

impl<H, M> Subscriber<M> for SubscribingHandler<H, M>
where
    H: Handler<M>,
{
    fn receive(
        &self,
        message: &M,
    )
    {
        self.handler.handle(message);
    }
}

impl<H, M> From<H> for SubscribingHandler<H, M>
where
    H: Handler<M>,
{
    fn from(handler: H) -> Self
    {
        SubscribingHandler::new(handler)
    }
}
