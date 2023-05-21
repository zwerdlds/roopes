use super::Builder;
use std::marker::PhantomData;

pub trait LambdaBuilderDelegate<R, P> = (Fn(&P) -> R);

pub struct LambdaBuilder<D, R, P>
where
    D: LambdaBuilderDelegate<R, P>,
{
    params: P,
    delegate: D,
    _t: PhantomData<(D, R, P)>,
}

impl<D, R, P> LambdaBuilder<D, R, P>
where
    D: LambdaBuilderDelegate<R, P>,
{
    pub fn new(
        delegate: D,
        params: P,
    ) -> LambdaBuilder<D, R, P>
    {
        LambdaBuilder {
            delegate,
            params,
            _t: PhantomData,
        }
    }
}

impl<D, R, P> Builder<R, P> for LambdaBuilder<D, R, P>
where
    D: LambdaBuilderDelegate<R, P>,
{
    fn build(&self) -> R
    {
        (self.delegate)(&self.params)
    }

    fn params(&mut self) -> &mut P
    {
        &mut self.params
    }
}
