use super::Emitter;
use std::marker::PhantomData;

pub trait LambdaEmitterDelegate<O> = Fn() -> O;

#[derive(Clone)]
pub struct LambdaEmitter<L, O>
where
    L: LambdaEmitterDelegate<O>,
{
    delegate: L,
    _t: PhantomData<O>,
}

impl<L, O> LambdaEmitter<L, O>
where
    L: LambdaEmitterDelegate<O>,
{
    pub fn new(delegate: L) -> LambdaEmitter<L, O>
    {
        LambdaEmitter {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<L, O> Emitter<O> for LambdaEmitter<L, O>
where
    L: LambdaEmitterDelegate<O>,
{
    fn emit(&self) -> O
    {
        (self.delegate)()
    }
}

impl<C, M> From<C> for LambdaEmitter<C, M>
where
    C: LambdaEmitterDelegate<M>,
{
    fn from(delegate: C) -> Self
    {
        LambdaEmitter::new(delegate)
    }
}

#[cfg(test)]
mod tests
{
    use crate::emitter::{
        lambda_emitter::LambdaEmitter,
        Emitter,
    };
    use std::{
        cell::RefCell,
        rc::Rc,
    };

    #[test]
    fn simple_lambda_refcell_mutation()
    {
        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = LambdaEmitter::new(move || {
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
