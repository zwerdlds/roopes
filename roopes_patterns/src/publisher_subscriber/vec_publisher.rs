use crate::prelude::*;
use core::marker::PhantomData;
use roopes_primitives::prelude::*;

/// Implements a [`Publisher`] based on a [`Vec`] of [`Subscriber`]s.
/// # Example
/// ``` rust
/// ```
pub struct VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    listeners: Vec<S>,
    _retain_types: PhantomData<M>,
}

impl<M, S> VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    #[must_use]
    pub fn new(listeners: Vec<S>) -> VecPublisher<M, S>
    {
        VecPublisher {
            listeners,
            _retain_types: PhantomData::default(),
        }
    }
}

impl<M, S> Default for VecPublisher<M, S>
where
    S: Subscriber<M>,
{
    fn default() -> Self
    {
        Self::new(Vec::new())
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

impl<M, S> Attachable<S> for VecPublisher<M, S>
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
        &mut self,
        detach_subscriber: S,
    ) -> Result<(), DetachError>
    {
        let (i, _) = self
            .listeners
            .iter()
            .enumerate()
            .find(|(_, o)| o.eq(&&detach_subscriber))
            .ok_or(DetachError::SubscriberNotFound)?;

        self.listeners.swap_remove(i);

        Ok(())
    }
}
