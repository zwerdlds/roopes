use super::AbstractFactory;
use std::marker::PhantomData;

pub trait LambdaAbstractFactoryDelegate<R> = (Fn() -> R);

pub struct LambdaAbstractFactory<D, R>
where
    D: LambdaAbstractFactoryDelegate<R>,
{
    delegate: D,
    _t: PhantomData<(D, R)>,
}

impl<D, R> AbstractFactory<R> for LambdaAbstractFactory<D, R>
where
    D: LambdaAbstractFactoryDelegate<R>,
{
    fn create(&self) -> R
    {
        (self.delegate)()
    }
}

impl<D, R> LambdaAbstractFactory<D, R>
where
    D: LambdaAbstractFactoryDelegate<R>,
{
    pub fn new(delegate: D) -> LambdaAbstractFactory<D, R>
    {
        LambdaAbstractFactory {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<D, R> From<D> for LambdaAbstractFactory<D, R>
where
    D: LambdaAbstractFactoryDelegate<R>,
{
    fn from(delegate: D) -> Self
    {
        LambdaAbstractFactory::new(delegate)
    }
}
