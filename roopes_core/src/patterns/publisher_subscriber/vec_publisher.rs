use crate::prelude::*;
use core::marker::PhantomData;
use std::cell::RefCell;

/// Implements a [`Publisher`] based on a [`Vec`] of [`Subscriber`]s.
/// # Example
/// ``` rust
/// ```
pub struct VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    listeners: RefCell<Vec<S>>,
    _retain_types: PhantomData<M>,
}

impl<M, S> Default for VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    fn default() -> Self
    {
        Self::new(RefCell::default())
    }
}

impl<M, S> VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    #[must_use]
    pub fn new(listeners: RefCell<Vec<S>>) -> VecPublisher<M, S>
    {
        VecPublisher {
            listeners,
            _retain_types: PhantomData::default(),
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
        self.listeners
            .borrow()
            .iter()
            .for_each(|s| s.receive(message));
    }
}

impl<M, S> Attachable<S> for VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    fn attach(
        &self,
        attach_subscriber: S,
    )
    {
        self.listeners.borrow_mut().push(attach_subscriber);
    }
}

/// An Error which occurs during detachment.
pub enum DetachError
{
    /// The specified subscriber couldn't be found.
    SubscriberNotFound,
}

impl<M, S> Detachable<S, (), DetachError> for VecPublisher<M, S>
where
    S: Subscriber<M> + Eq,
{
    fn detach(
        &self,
        detach_subscriber: &S,
    ) -> Result<(), DetachError>
    {
        let (i, _) = self
            .listeners
            .borrow()
            .iter()
            .enumerate()
            .find(|(_, o)| o.eq(&detach_subscriber))
            .ok_or(DetachError::SubscriberNotFound)?;

        self.listeners.borrow_mut().swap_remove(i);

        Ok(())
    }
}
