use std::marker::PhantomData;

#[cfg(test)]
mod tests
{
    use ropes_lib::prelude::*;
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

    #[test]
    fn simple_heap_refcell_mutation()
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
