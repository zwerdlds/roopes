//! Provides a heap-based [`Head`] which redirects
//! [`Transformer::transform`] calls to a delegate
//! [`Transformer`], then enables additional
//! stages to be added arbitrarily.

use crate::primitives::transformer::{
    self,
    Transformer,
};
use delegate::delegate;

/// The first stage in the [`Transformer`] Chain.
/// Unique because it does not an
/// the prior [`Transformer`], but is instead
/// given input from the client.
pub struct Head<I, O>
{
    transformer: Box<dyn Transformer<I, O>>,
}

impl<I, O> Head<I, O>
{
    /// Creates a new [`Head`] with a given
    /// [`Box`]ed [`Transformer`].
    #[must_use]
    pub fn new(transformer: Box<dyn Transformer<I, O>>) -> Head<I, O>
    {
        Head { transformer }
    }

    /// Adds a stage of execution, giving a new
    /// [`Heap`] back.
    pub fn push<N, T>(
        self,
        transformer: T,
    ) -> Heap<I, N, O>
    where
        T: Transformer<O, N> + 'static,
        N: 'static,
        O: 'static,
        I: 'static,
    {
        let prev = Box::new(self);
        let transformer = Box::new(transformer);

        Heap { prev, transformer }
    }
}

#[allow(clippy::inline_always)]
impl<I, O> Transformer<I, O> for Head<I, O>
{
    delegate! {
            to self.transformer {
            fn transform(
                &self,
                input: &I,
            ) -> O;
        }
    }
}

/// Stores previous transformations, and the
/// ability to produce new transformations.
pub struct Heap<I, O, P>
{
    prev: Box<dyn Transformer<I, P>>,
    transformer: Box<dyn Transformer<P, O>>,
}

impl<I, O, P> Heap<I, O, P>
{
    /// Adds a stage of execution, giving a new
    /// [`Heap`] back.
    #[must_use]
    pub fn push<N, E>(
        self,
        transformer: E,
    ) -> Heap<I, N, O>
    where
        E: Transformer<O, N> + 'static,
        O: 'static,
        N: 'static,
        I: 'static,
        P: 'static,
    {
        let prev = Box::new(self);
        let transformer = Box::new(transformer);

        Heap { prev, transformer }
    }
}

impl<I, O, P> Transformer<I, O> for Heap<I, O, P>
{
    #[must_use]
    fn transform(
        &self,
        input: &I,
    ) -> O
    {
        self.transformer.transform(&self.prev.transform(input))
    }
}
