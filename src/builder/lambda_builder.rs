use super::Builder;
use std::marker::PhantomData;

pub trait LambdaBuilderDelegate<I, O> = (Fn(&I) -> O);

pub struct LambdaBuilder<D, I, O>
where
    D: LambdaBuilderDelegate<I, O>,
{
    params: I,
    delegate: D,
    _t: PhantomData<(D, I, O)>,
}

impl<D, I, O> LambdaBuilder<D, I, O>
where
    D: LambdaBuilderDelegate<I, O>,
{
    pub fn new(
        delegate: D,
        params: I,
    ) -> LambdaBuilder<D, I, O>
    {
        LambdaBuilder {
            delegate,
            params,
            _t: PhantomData,
        }
    }
}

impl<D, I, O> Builder<I, O> for LambdaBuilder<D, I, O>
where
    D: LambdaBuilderDelegate<I, O>,
{
    fn build(&self) -> O
    {
        (self.delegate)(&self.params)
    }

    fn set_params(
        &mut self,
        params: I,
    )
    {
        self.params = params;
    }

    fn params(&self) -> &I
    {
        &self.params
    }
}
