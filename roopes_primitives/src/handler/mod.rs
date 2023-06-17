pub mod heap;
pub mod lambda;

pub use heap::Heap;
pub use lambda::Lambda;

/// Defines a primitive interface which consumes messages.
pub trait Handler<M>
{
    fn handle(
        &self,
        message: &M,
    );
}

impl<C, M> From<Lambda<C, M>> for Heap<M>
where
    C: lambda::Delegate<M> + 'static,
    M: 'static,
{
    fn from(handler: Lambda<C, M>) -> Heap<M>
    {
        Heap::new(Box::new(handler))
    }
}

pub mod prelude
{
    pub use super::Handler;
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
    fn simple_lambda_refcell_mutation()
    {
        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let fn_handler = move |v: &bool| {
            (*has_run_ext.borrow_mut()) = *v;
        };

        let lambda_handler: handler::Lambda<_, bool> = fn_handler.into();

        lambda_handler.handle(&true);

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
