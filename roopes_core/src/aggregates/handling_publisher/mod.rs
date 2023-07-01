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
//! The [`publishing_handler`] module creates
//! [`Handler`]s from arbitrary [`Publisher`]s.
//!
//! ![publishing handler diagram][publishing-handler-diagram]

use crate::prelude::{
    handler::lambda::Delegate,
    *,
};
use std::{
    hash::Hash,
    marker::PhantomData,
};

#[cfg(test)]
mod test;

/// Exposes the [`HandlingPublisher`] type at the
/// library level.
pub mod prelude
{
    pub use super::HandlingPublisher;
}

/// Provides the [`Subscriber`] and [`Handler`]
/// traits for a wrapped [`Handler`] delegate.
pub struct HandlingPublisher<P, M>
where
    P: Publisher<M>,
{
    delegate: P,
    _retain_types: PhantomData<M>,
}

impl<P, M> HandlingPublisher<P, M>
where
    P: Publisher<M>,
{
    /// Creates a [`HandlingPublisher`] from a
    /// given [`Handler`].
    pub fn new(delegate: P) -> HandlingPublisher<P, M>
    {
        HandlingPublisher {
            delegate,
            _retain_types: PhantomData,
        }
    }
}

impl<P, M> Publisher<M> for HandlingPublisher<P, M>
where
    P: Publisher<M>,
{
    fn publish(
        &self,
        message: &M,
    )
    {
        self.delegate.publish(message);
    }
}

impl<P, M> Handler<M> for HandlingPublisher<P, M>
where
    P: Publisher<M>,
{
    fn handle(
        &self,
        message: &M,
    )
    {
        self.delegate.publish(message);
    }
}

impl<P, M> From<P> for HandlingPublisher<P, M>
where
    P: Publisher<M>,
{
    fn from(delegate: P) -> Self
    {
        HandlingPublisher::new(delegate)
    }
}

impl<P, M> PartialEq for HandlingPublisher<P, M>
where
    P: Publisher<M> + PartialEq,
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.delegate.eq(&other.delegate)
    }
}

impl<P, M> Eq for HandlingPublisher<P, M> where P: Publisher<M> + Eq {}

impl<P, M> Hash for HandlingPublisher<P, M>
where
    P: Publisher<M> + Hash,
{
    fn hash<S: std::hash::Hasher>(
        &self,
        state: &mut S,
    )
    {
        self.delegate.hash(state);
    }
}
