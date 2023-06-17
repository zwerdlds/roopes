pub mod iterator;
pub mod lambda;

pub use iterator::Iterator;
pub use lambda::Lambda;

pub trait Emitter<O>
{
    fn emit(&self) -> O;
}

pub mod prelude
{
    pub use super::Emitter;
}

#[cfg(test)]
mod tests
{
    use roopes::prelude::*;
    use std::{
        cell::RefCell,
        rc::Rc,
    };

    #[test]
    fn latent_lambda_refcell_mutation()
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

    #[test]
    fn iterator_emitter()
    {
        let test_iter = {
            let vec = vec![1, 2, 3];
            let iter = vec.into_iter();
            let rcb_iter = RefCell::new(Box::new(iter));
            emitter::Iterator::new(rcb_iter)
        };

        assert!(test_iter.emit() == Some(1));
        assert!(test_iter.emit() == Some(2));
        assert!(test_iter.emit() == Some(3));
        assert!(test_iter.emit() == None);
        assert!(test_iter.emit() == None);
    }
}
