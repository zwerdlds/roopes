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
