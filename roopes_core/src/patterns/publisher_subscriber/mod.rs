//!
#![cfg_attr(feature = "doc-images",
  cfg_attr(
    all(),
    doc = ::embed_doc_image::embed_image!(
        "publisher-subscriber-diagram",
        "src/patterns/publisher_subscriber/publisher_subscriber.svg"
)))]
//! This module implements the
//! Publisher-Subscriber pattern.  In this model,
//! a [`Publisher`] contains a list of
//! [`Subscriber`]s.  When a message is
//! delivered to the [`Publisher`], it is then
//! delegated directly to all its [`Subscriber`]s,
//! one at a time.  Typically, implementations are
//! dynamic, in that [`Subscriber`]s are able to
//! be dded or removed at will by the client
//! code - these features are delineated
//! separately in the [`DetachablePublisher`] and
//! [`AttachablePublisher`] traits, but can be
//! implemented by any [`Publisher`], and together
//! as a [`MutablePublisher`].
//!
//! ![publisher subscriber diagram][publisher-subscriber-diagram]

pub mod heap;
pub mod vec_publisher;
use crate::prelude::*;
pub use vec_publisher::VecPublisher;

/// A [`Publisher`] distributes messages it
/// receives to the [`Subscriber`]s
/// which it currently holds.
pub trait Publisher<M>
{
    /// Broadcasts a message to the current
    /// [`Subscriber`]s of this [`Publisher`].
    fn publish(
        &self,
        message: &M,
    );
}

/// Allows [`Subscriber`]s to be added to the
/// implementing [`Publisher`].
pub trait AttachablePublisher<M, S>: Publisher<M>
where
    S: Subscriber<M>,
{
    /// Attaches the given [`Subscriber`] from the
    /// [`Publisher`] so it would now
    /// receive notifications.
    fn attach(
        &mut self,
        attach_subscriber: S,
    );
}

/// Allows [`Subscriber`]s to be removed from the
/// implementing [`Publisher`].
pub trait DetachablePublisher<M, S, E>
where
    S: Subscriber<M>,
{
    /// Detaches the given [`Subscriber`] from the
    /// [`Publisher`] so it would no
    /// longer receive notifications.
    ///
    /// # Errors
    /// E: An error that occurred during
    /// detachment.
    fn detach(
        &mut self,
        detach_subscriber: &S,
    ) -> Result<(), E>;
}

/// Convenience trait for [`Publisher`]s which
/// implement both [`AttachablePublisher`] and
/// [`DetachablePublisher`].
pub trait MutablePublisher<M, S, E>:
    DetachablePublisher<M, S, E> + AttachablePublisher<M, S>
where
    S: Subscriber<M>,
    Self: Publisher<M>,
{
}

/// [`Subscriber`]s can be attached to, and will
/// receive messages from, [`Publisher`]s.
pub trait Subscriber<M>
{
    /// Receives a borrowed message for the
    /// [`Subscriber`] to process.
    fn receive(
        &self,
        message: &M,
    );
}

/// Exposes the [`Publisher`], [`Subscriber`],
/// [`AttachablePublisher`] and
/// [`DetachablePublisher`] types at the library
/// level.
pub mod prelude
{
    pub use super::{
        AttachablePublisher,
        DetachablePublisher,
        Publisher,
        Subscriber,
    };
}

#[cfg(test)]
mod tests;
