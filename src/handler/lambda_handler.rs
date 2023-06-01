use super::Handler;
use std::marker::PhantomData;

pub trait LambdaHandlerDelegate<M> = Fn(M);

pub struct LambdaHandler<C, M>
where
    C: LambdaHandlerDelegate<M>,
{
    delegate: C,
    _t: PhantomData<M>,
}

impl<C, M> LambdaHandler<C, M>
where
    C: LambdaHandlerDelegate<M>,
{
    pub fn new(delegate: C) -> LambdaHandler<C, M>
    {
        LambdaHandler {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<C, M> Handler<M> for LambdaHandler<C, M>
where
    C: LambdaHandlerDelegate<M>,
{
    fn handle(
        &self,
        message: M,
    )
    {
        (self.delegate)(message);
    }
}

impl<C, M> From<C> for LambdaHandler<C, M>
where
    C: LambdaHandlerDelegate<M>,
{
    fn from(delegate: C) -> Self
    {
        LambdaHandler::new(delegate)
    }
}

#[cfg(test)]
mod tests
{
    use crate::handler::{
        lambda_handler::LambdaHandler,
        Handler,
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

        let lc = LambdaHandler::new(move |v| {
            (*has_run_ext.borrow_mut()) = v;
        });
        lc.handle(true);

        assert!((*has_run.borrow()));
    }
}
