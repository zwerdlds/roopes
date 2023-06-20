//! Provides a simple wrapper struct around [`Emitter`], `Fn()->T`
//! types.
use super::AbstractFactory;
use crate::prelude::*;
use std::marker::PhantomData;

pub struct Lambda<D, R>
where
    D: Emitter<R>,
{
    delegate: D,
    _retain_types: PhantomData<(D, R)>,
}

impl<D, R> AbstractFactory<R> for Lambda<D, R>
where
    D: Emitter<R>,
{
    /// Creates the given type from the [`delegate`].
    fn create(&self) -> R
    {
        self.delegate.emit()
    }
}

impl<D, R> Lambda<D, R>
where
    D: Emitter<R>,
{
    pub fn new(delegate: D) -> Lambda<D, R>
    {
        Lambda {
            delegate,
            _retain_types: PhantomData,
        }
    }
}

impl<D, R> From<D> for Lambda<D, R>
where
    D: Emitter<R>,
{
    fn from(delegate: D) -> Self
    {
        Lambda::new(delegate)
    }
}
