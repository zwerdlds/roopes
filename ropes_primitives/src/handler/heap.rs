use crate::prelude::*;
use delegate::delegate;

pub struct Heap<M>
{
    delegate: Box<dyn Handler<M>>,
}

impl<M> Heap<M>
{
    #[must_use]
    pub fn new(delegate: Box<dyn Handler<M>>) -> Heap<M>
    {
        Heap { delegate }
    }
}

impl<M> Handler<M> for Heap<M>
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

        let lh = handler::Lambda::new(move |v| {
            (*has_run_ext.borrow_mut()) = *v;
        });
        let hh = handler::Heap::new(Box::new(lh));

        assert!(!(*has_run.borrow()));
        hh.handle(&true);
        assert!((*has_run.borrow()));
    }
}
