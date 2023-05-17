use super::Factory;
use std::marker::PhantomData;

pub trait LambdaFactoryDelegate<R> = (Fn() -> R) + 'static;

pub struct LambdaFactory<D, R>
where
    D: LambdaFactoryDelegate<R>,
{
    delegate: D,
    _t: PhantomData<(D, R)>,
}

impl<D, R> Factory<R> for LambdaFactory<D, R>
where
    D: LambdaFactoryDelegate<R>,
{
    fn create(&self) -> R
    {
        (self.delegate)()
    }
}

impl<D, R> LambdaFactory<D, R>
where
    D: LambdaFactoryDelegate<R>,
{
    pub fn new(delegate: D) -> LambdaFactory<D, R>
    {
        LambdaFactory {
            delegate,
            _t: PhantomData,
        }
    }
}
