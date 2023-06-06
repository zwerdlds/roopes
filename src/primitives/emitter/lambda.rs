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

#[cfg(test)]
mod tests
{
    use crate::prelude::*;
    use std::{
        cell::RefCell,
        rc::Rc,
    };

    #[test]
    fn simple_lambda_refcell_mutation()
    {
        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = emitter::Lambda::new(move || {
            let result = *has_run_ext.borrow_mut();
            (*has_run_ext.borrow_mut()) = true;
            result
        });

        assert!(!(*has_run.borrow()));
        assert!(!lc.emit());
        assert!((*has_run.borrow()));
        assert!(lc.emit());
    }
}
