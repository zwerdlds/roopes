use super::Handler;
use std::marker::PhantomData;

pub trait Delegate<M> = Fn(&M);

/// Defines an encapsulated [`Handler`] as a struct, which just delegates its
/// execution to the attached [`Delegate`].
///
/// # Examples
/// ``` rust
/// use roopes::prelude::*;
/// let handler = handler::Lambda::new(|msg| {
///     println!("{msg}");
/// });
/// handler.handle(&"Hello world!".to_string());
/// ```
#[derive(Clone)]
pub struct Lambda<C, M>
where
    C: Delegate<M>,
{
    delegate: C,
    _t: PhantomData<M>,
}

impl<C, M> Lambda<C, M>
where
    C: Delegate<M>,
{
    pub fn new(delegate: C) -> Lambda<C, M>
    {
        Lambda {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<C, M> Handler<M> for Lambda<C, M>
where
    C: Delegate<M>,
{
    fn handle(
        &self,
        message: &M,
    )
    {
        (self.delegate)(message);
    }
}

impl<C, M> From<C> for Lambda<C, M>
where
    C: Delegate<M>,
{
    fn from(delegate: C) -> Self
    {
        Lambda::new(delegate)
    }
}
