pub mod heap;
pub mod vec_publisher;
use crate::prelude::*;
pub use vec_publisher::VecPublisher;

pub trait Publisher<M, S>
where
    S: Subscriber<M>,
{
    fn publish(
        &self,
        message: &M,
    );
}

pub trait AttachablePublisher<M, S>
where
    S: Subscriber<M>,
{
    /// Attaches the given [`Subscriber`] from the [`Publisher`] so it would now
    /// receive notifications.
    fn attach(
        &mut self,
        attach_subscriber: S,
    );
}

pub trait DetachablePublisher<M, S, E>
where
    S: Subscriber<M>,
{
    /// Detaches the given [`Subscriber`] from the [`Publisher`] so it would no
    /// longer receive notifications.
    ///
    /// # Errors
    /// E: An error that occurred during detachment.
    fn detach(
        &mut self,
        detach_subscriber: &S,
    ) -> Result<(), E>;
}

pub trait Subscriber<M>
{
    fn receive(
        &self,
        message: &M,
    );
}

/// Exposes the [`Publisher`], [`Subscriber`], [`AttachablePublisher`] and
/// [`DetachablePublisher`] types at the library level.
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
