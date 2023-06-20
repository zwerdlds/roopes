//! Provides a simple wrapper struct around [`super::Delegate`], `Fn() -> I`
//! types.

use super::Emitter;
use std::marker::PhantomData;

/// Provides a type which redirects transformations to its enclosed
/// [`super::Delegate`].
#[derive(Clone)]
pub struct Lambda<L, O>
where
    L: super::Delegate<O>,
{
    delegate: L,
    _t: PhantomData<O>,
}

impl<L, O> Lambda<L, O>
where
    L: super::Delegate<O>,
{
    /// Creates a [`Lambda`] from a given [`super::Delegate`].
    pub fn new(delegate: L) -> Lambda<L, O>
    {
        Lambda {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<L, O> Emitter<O> for Lambda<L, O>
where
    L: super::Delegate<O>,
{
    fn emit(&self) -> O
    {
        (self.delegate)()
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
