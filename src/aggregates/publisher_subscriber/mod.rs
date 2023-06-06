pub mod vec_publisher;
use crate::prelude::*;
pub use vec_publisher::VecPublisher;

pub trait Publisher<M>
{
    fn publish(
        &self,
        message: &M,
    );
}

pub trait Subscriber<M>
{
    fn receive(
        &self,
        message: &M,
    );
}

/// An Error which occurs during detachment.
pub enum DetachError
{
    /// The specified observer couldn't be found.
    SubscriberNotFound,
}

pub trait MutablePubllisher<M>:
    Attachable<M> + Detachable<M, DetachError> + Subscriber<M>
where
    M: Observer,
{
}
