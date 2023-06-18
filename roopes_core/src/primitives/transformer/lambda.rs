use super::Transformer;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Lambda<C, I, O>
where
    C: super::Delegate<I, O>,
{
    delegate: C,
    _t: PhantomData<(I, O)>,
}

impl<C, I, O> Lambda<C, I, O>
where
    C: super::Delegate<I, O>,
{
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
    C: super::Delegate<I, O>,
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
    C: super::Delegate<I, O>,
{
    fn from(delegate: C) -> Self
    {
        Lambda::new(delegate)
    }
}
