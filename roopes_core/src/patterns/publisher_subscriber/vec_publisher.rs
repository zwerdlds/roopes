//! Contains an implementation of [`Publisher`]
//! which stores its [`Subscriber`]s in a [`Vec`].

use super::{
    AttachablePublisher,
    DetachablePublisher,
};
use crate::prelude::*;
use core::marker::PhantomData;
use std::cell::RefCell;

/// Implements a [`Publisher`] based on a [`Vec`]
/// of [`Subscriber`]s. # Example
/// ``` rust
/// use roopes::prelude::*;
/// use std::{
///     cell::RefCell,
///     rc::Rc,
/// };
///
/// let has_run = Rc::new(RefCell::new(false));
/// let mut publisher = publisher_subscriber::VecPublisher::default();
/// let has_run_ext = has_run.clone();
/// let heap_handler =
///     handler::Heap::new(Box::new(handler::Lambda::new(move |v| {
///         (*has_run_ext.borrow_mut()) = *v;
///     })));
/// let sub_handler: SubscribingHandler<_, _> = heap_handler.into();
/// let subscriber =
///     publisher_subscriber::heap::Subscriber::new(Box::new(sub_handler));
/// publisher.attach(subscriber);
/// assert!(!(*has_run.borrow()));
/// publisher.publish(&true);
/// assert!((*has_run.borrow()));
/// ```
pub struct VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    listeners: Vec<S>,
    _retain_types: PhantomData<M>,
}

/// An Error which occurs during detachment.
#[derive(Debug)]
pub enum DetachError
{
    /// The specified observer couldn't be found.
    SubscriberNotFound,
}

impl<M, S> Default for VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    fn default() -> Self
    {
        Self::new(Vec::default())
    }
}

impl<M, S> VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    /// Creates a new [`VecPublisher`] with the
    /// given [`Vec`] of starting
    /// [`Subscriber`]s.
    #[must_use]
    pub fn new(listeners: Vec<S>) -> VecPublisher<M, S>
    {
        VecPublisher {
            listeners,
            _retain_types: PhantomData,
        }
    }
}

impl<M, S> Publisher<M> for VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    fn publish(
        &self,
        message: &M,
    )
    {
        self.listeners.iter().for_each(|s| s.receive(message));
    }
}

impl<M, S> DetachablePublisher<M, S, DetachError> for VecPublisher<M, S>
where
    S: Subscriber<M> + Eq,
{
    fn detach(
        &mut self,
        detach_subscriber: &S,
    ) -> Result<(), DetachError>
    {
        let (i, _) = self
            .listeners
            .iter()
            .enumerate()
            .find(|(_, o)| o.eq(&detach_subscriber))
            .ok_or(DetachError::SubscriberNotFound)?;

        self.listeners.swap_remove(i);

        Ok(())
    }
}

impl<M, S> AttachablePublisher<M, S> for VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    fn attach(
        &mut self,
        attach_subscriber: S,
    )
    {
        self.listeners.push(attach_subscriber);
    }
}

impl<M, S> PartialEq for VecPublisher<M, S>
where
    S: Subscriber<M> + Eq,
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.listeners == other.listeners
    }
}
