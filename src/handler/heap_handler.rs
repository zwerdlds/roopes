use super::Handler;
use delegate::delegate;

pub struct HeapHandler<M>
{
    delegate: Box<dyn Handler<M>>,
}

impl<M> HeapHandler<M>
{
    pub fn new(delegate: Box<dyn Handler<M>>) -> HeapHandler<M>
    {
        HeapHandler { delegate }
    }
}

impl<M> Handler<M> for HeapHandler<M>
{
    delegate! {
        to self.delegate {
           fn handle(&self, message: &M);
        }
    }
}

#[cfg(test)]
mod tests
{
    use crate::handler::{
        heap_handler::HeapHandler,
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

        let lh = LambdaHandler::new(move |v| {
            (*has_run_ext.borrow_mut()) = *v;
        });
        let hh = HeapHandler::new(Box::new(lh));

        assert!(!(*has_run.borrow()));
        hh.handle(&true);
        assert!((*has_run.borrow()));
    }
}
