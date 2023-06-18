use super::Receiver;
use std::marker::PhantomData;

/// Defines an encapsulated [`Receiver`] as a struct, which just delegates its
/// execution to the attached [`Delegate`].
///
/// # Examples
/// ``` rust
/// use roopes::prelude::*;
/// let receiver = receiver::Lambda::new(|msg| {
///     println!("{msg}");
/// });
/// receiver.receive(&"Hello world!".to_string());
/// ```
#[derive(Clone)]
pub struct Lambda<C, M>
where
    C: super::Delegate<M>,
{
    delegate: C,
    _t: PhantomData<M>,
}

impl<C, M> Lambda<C, M>
where
    C: super::Delegate<M>,
{
    pub fn new(delegate: C) -> Lambda<C, M>
    {
        Lambda {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<C, M> Receiver<M> for Lambda<C, M>
where
    C: super::Delegate<M>,
{
    fn receive(
        &self,
        message: M,
    )
    {
        (self.delegate)(message);
    }
}

impl<C, M> From<C> for Lambda<C, M>
where
    C: super::Delegate<M>,
{
    fn from(delegate: C) -> Self
    {
        Lambda::new(delegate)
    }
}
