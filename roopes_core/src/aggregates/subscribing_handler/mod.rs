//!
#![cfg_attr(feature = "doc-images",
  cfg_attr(
    all(),
    doc = ::embed_doc_image::embed_image!(
        "subscribing-handler-diagram",
        "src/aggregates/subscribing_handler/subscribing_handler.svg"
)))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and \
           Rust version >= 1.54 to enable."
)]
//! The [`subscribing_handler`] module creates
//! [`Subscriber`]s from arbitrary [`Handler`]s.
//!
//! ![subscribing handler diagram][subscribing-handler-diagram]

use crate::prelude::{
    handler::lambda::Delegate,
    *,
};
use delegate::delegate;
use std::{
    hash::Hash,
    marker::PhantomData,
};

#[cfg(test)]
mod test;

/// Exposes the [`SubscribingHandler`] type at the
/// library level.
pub mod prelude
{
    pub use super::SubscribingHandler;
}

/// Provides the [`Subscriber`] and [`Handler`]
/// traits for a wrapped [`Handler`] delegate.
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
    /// Creates a [`SubscribingHandler`] from a
    /// given [`Handler`].
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

#[allow(clippy::inline_always)]
impl<H, M> Handler<M> for SubscribingHandler<H, M>
where
    H: Handler<M>,
{
    delegate! {
        to self.handler{
            fn handle(
                &self,
                message: &M,
            );
        }
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

/// Provides the ability to convert a [`Handler`] into a
/// [`SubscribingHandler`], for use as a [`Subscriber`].
pub trait IntoSubscriber<H, M>
where
    H: Handler<M>,
{
    /// Wraps the [`Handler`] in a [`SubscribingHandler`], for use as a
    /// [`Subscriber`].
    fn into_subscriber(self) -> SubscribingHandler<H, M>;
}

impl<H, M> IntoSubscriber<H, M> for H
where
    H: Handler<M>,
{
    fn into_subscriber(self) -> SubscribingHandler<H, M>
    {
        SubscribingHandler::new(self)
    }
}
