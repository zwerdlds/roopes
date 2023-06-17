pub mod hashable;
pub mod heap;
pub mod lambda;

pub use hashable::Hashable;
pub use heap::Heap;
pub use lambda::Lambda;

pub trait Command
{
    fn execute(&self);
}

pub mod prelude
{
    pub use super::Command;
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
    fn simple_lambda_command_refcell_mutation()
    {
        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = command::Lambda::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        });
        lc.execute();

        assert!((*has_run.borrow()));
    }

    #[test]
    fn simple_hashable_command_refcell_mutation()
    {
        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = command::Lambda::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        });

        let hc = command::Hashable::new(lc, "test");

        hc.execute();

        assert!((*has_run.borrow()));
    }
}
