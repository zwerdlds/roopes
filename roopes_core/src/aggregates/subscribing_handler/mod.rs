//! The [`subscribing_handler`] module creates [`Subscriber`]s
//! from arbitrary [`Handler`]s.

use crate::prelude::*;
use std::{
    hash::Hash,
    marker::PhantomData,
};

#[cfg(test)]
mod test;

/// Exposes the [`SubscribingHandler`] type at the library level.
pub mod prelude
{
    pub use super::SubscribingHandler;
}

/// Provides the [`Subscriber`] and [`Handler`] traits for a wrapped [`Handler`]
/// delegate.
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
    /// Creates a [`SubscribingHandler`] from a given [`Handler`].
    pub fn new(handler: H) -> SubscribingHandler<H, M>
    {
        SubscribingHandler {
            handler,
            _retain_types: PhantomData,
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

impl<H, M> Handler<M> for SubscribingHandler<H, M>
where
    H: Handler<M>,
{
    fn handle(
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

impl<H, M> PartialEq for SubscribingHandler<H, M>
where
    H: PartialEq + Handler<M>,
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.handler.eq(&other.handler)
    }
}

impl<H, M> Eq for SubscribingHandler<H, M> where H: Eq + Handler<M> {}

impl<H, M> Hash for SubscribingHandler<H, M>
where
    H: Handler<M> + Hash,
{
    fn hash<S: std::hash::Hasher>(
        &self,
        state: &mut S,
    )
    {
        self.handler.hash(state);
    }
}
