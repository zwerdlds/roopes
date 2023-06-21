//! Provides a simple wrapper struct around
//! [`Delegate`], `Fn(&I)->O` types.

/// [`Lambda`] [`Delegate`]s given input and
/// produce owned output.
pub trait Delegate<I, O> = Fn(&I) -> O;

use super::Transformer;
use std::marker::PhantomData;

/// Provides a type which redirects
/// transformations to its enclosed [`Delegate`].
#[derive(Clone)]
pub struct Lambda<C, I, O>
where
    C: Delegate<I, O>,
{
    delegate: C,
    _t: PhantomData<(I, O)>,
}

impl<C, I, O> Lambda<C, I, O>
where
    C: Delegate<I, O>,
{
    /// Creates a [`Lambda`] from a given
    /// [`Delegate`].
    pub fn new(delegate: C) -> Lambda<C, I, O>
    {
        Lambda {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<C, I, O> Transformer<I, O> for Lambda<C, I, O>
where
    C: Delegate<I, O>,
{
    fn transform(
        &self,
        input: &I,
    ) -> O
    {
        (self.delegate)(input)
    }
}

impl<C, I, O> From<C> for Lambda<C, I, O>
where
    C: Delegate<I, O>,
{
    fn from(delegate: C) -> Self
    {
        Lambda::new(delegate)
    }
}
