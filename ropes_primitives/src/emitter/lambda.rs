use super::Emitter;
use std::marker::PhantomData;

pub trait Delegate<O> = Fn() -> O;

#[derive(Clone)]
pub struct Lambda<L, O>
where
    L: Delegate<O>,
{
    delegate: L,
    _t: PhantomData<O>,
}

impl<L, O> Lambda<L, O>
where
    L: Delegate<O>,
{
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
    L: Delegate<O>,
{
    fn emit(&self) -> O
    {
        (self.delegate)()
    }
}

impl<C, M> From<C> for Lambda<C, M>
where
    C: Delegate<M>,
{
    fn from(delegate: C) -> Self
    {
        Lambda::new(delegate)
    }
}
