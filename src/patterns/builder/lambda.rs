use super::Builder;
use std::marker::PhantomData;

pub trait Delegate<I, O> = (Fn(&I) -> O);

pub struct Lambda<D, I, O>
where
    D: Delegate<I, O>,
{
    params: I,
    delegate: D,
    _t: PhantomData<(D, I, O)>,
}

impl<D, I, O> Lambda<D, I, O>
where
    D: Delegate<I, O>,
{
    pub fn new(
        delegate: D,
        params: I,
    ) -> Lambda<D, I, O>
    {
        Lambda {
            delegate,
            params,
            _t: PhantomData,
        }
    }
}

impl<D, I, O> Builder<I, O> for Lambda<D, I, O>
where
    D: Delegate<I, O>,
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
