use super::Handler;
use std::marker::PhantomData;

pub trait Delegate<M> = Fn(&M);

#[derive(Clone)]
pub struct Lambda<C, M>
where
    C: Delegate<M>,
{
    delegate: C,
    _t: PhantomData<M>,
}

impl<C, M> Lambda<C, M>
where
    C: Delegate<M>,
{
    pub fn new(delegate: C) -> Lambda<C, M>
    {
        Lambda {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<C, M> Handler<M> for Lambda<C, M>
where
    C: Delegate<M>,
{
    fn handle(
        &self,
        message: &M,
    )
    {
        (self.delegate)(message);
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

        let lc = handler::Lambda::new(move |v| {
            (*has_run_ext.borrow_mut()) = *v;
        });
        lc.handle(&true);

        assert!((*has_run.borrow()));
    }
}
