use super::Transformer;
use delegate::delegate;

pub struct Heap<I, O>
{
    delegate: Box<dyn Transformer<I, O>>,
}

impl<I, O> Heap<I, O>
{
    #[must_use]
    pub fn new(delegate: Box<dyn Transformer<I, O>>) -> Heap<I, O>
    {
        Heap { delegate }
    }
}

impl<I, O> Transformer<I, O> for Heap<I, O>
{
    delegate! {
        to self.delegate {
           fn transform(&self, input: &I) -> O;
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
        let n_runs = Rc::new(RefCell::new(0));

        let transformer = move |v: &bool| {
            (*has_run_ext.borrow_mut()) = *v;
            (*n_runs.borrow_mut()) += 1;
            *n_runs.borrow()
        };

        let lh = transformer::Lambda::new(transformer);
        let hh = transformer::Heap::new(Box::new(lh));

        assert!(!(*has_run.borrow()));
        assert_eq!(1, hh.transform(&true));
        assert!((*has_run.borrow()));
        assert_eq!(2, hh.transform(&true));
    }
}
