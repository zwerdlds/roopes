use super::Transformer;
use std::marker::PhantomData;

pub trait Delegate<I, O> = Fn(&I) -> O;

#[derive(Clone)]
pub struct Lambda<C, I, O>
where
    C: Delegate<I, O>,
{
    delegate: C,
    _t: PhantomData<(I, O)>,
}

impl<C, I, O> Lambda<C, I, O>
where
    C: Delegate<I, O>,
{
    pub fn new(delegate: C) -> Lambda<C, I, O>
    {
        Lambda {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<C, I, O> Transformer<I, O> for Lambda<C, I, O>
where
    C: Delegate<I, O>,
{
    fn transform(
        &self,
        input: &I,
    ) -> O
    {
        (self.delegate)(input)
    }
}

impl<C, I, O> From<C> for Lambda<C, I, O>
where
    C: Delegate<I, O>,
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

        let lc = transformer::Lambda::new(move |v| {
            (*has_run_ext.borrow_mut()) = *v;
        });
        lc.transform(&true);

        assert!((*has_run.borrow()));
    }
}
