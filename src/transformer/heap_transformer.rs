use super::Transformer;
use delegate::delegate;

pub struct HeapTransformer<I, O>
{
    delegate: Box<dyn Transformer<I, O>>,
}

impl<I, O> HeapTransformer<I, O>
{
    pub fn new(delegate: Box<dyn Transformer<I, O>>) -> HeapTransformer<I, O>
    {
        HeapTransformer { delegate }
    }
}

impl<I, O> Transformer<I, O> for HeapTransformer<I, O>
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
    use crate::transformer::{
        heap_transformer::HeapTransformer,
        lambda_transformer::LambdaTransformer,
        Transformer,
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
        let n_runs = Rc::new(RefCell::new(0));

        let transformer = move |v: &bool| {
            (*has_run_ext.borrow_mut()) = *v;
            (*n_runs.borrow_mut()) += 1;
            *n_runs.borrow()
        };

        let lh = LambdaTransformer::new(transformer);
        let hh = HeapTransformer::new(Box::new(lh));

        assert!(!(*has_run.borrow()));
        assert_eq!(1, hh.transform(&true));
        assert!((*has_run.borrow()));
        assert_eq!(2, hh.transform(&true));
    }
}
