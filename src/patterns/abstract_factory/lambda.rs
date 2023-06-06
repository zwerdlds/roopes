use super::AbstractFactory;
use std::marker::PhantomData;

pub trait Delegate<R> = (Fn() -> R);

pub struct Lambda<D, R>
where
    D: Delegate<R>,
{
    delegate: D,
    _t: PhantomData<(D, R)>,
}

impl<D, R> AbstractFactory<R> for Lambda<D, R>
where
    D: Delegate<R>,
{
    fn create(&self) -> R
    {
        (self.delegate)()
    }
}

impl<D, R> Lambda<D, R>
where
    D: Delegate<R>,
{
    pub fn new(delegate: D) -> Lambda<D, R>
    {
        Lambda {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<D, R> From<D> for Lambda<D, R>
where
    D: Delegate<R>,
{
    fn from(delegate: D) -> Self
    {
        Lambda::new(delegate)
    }
}
