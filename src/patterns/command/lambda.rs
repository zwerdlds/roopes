use super::Command;

pub trait Delegate = Fn();

pub struct Lambda<C>
where
    C: Delegate,
{
    delegate: C,
}

impl<C> Lambda<C>
where
    C: Delegate,
{
    pub fn new(delegate: C) -> Lambda<C>
    {
        Lambda { delegate }
    }
}

impl<C> Command for Lambda<C>
where
    C: Delegate,
{
    fn execute(&self)
    {
        (self.delegate)();
    }
}

impl<C> From<C> for Lambda<C>
where
    C: Delegate,
{
    fn from(delegate: C) -> Self
    {
        Lambda::new(delegate)
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

        let lc = command::Lambda::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        });
        lc.execute();

        assert!((*has_run.borrow()));
    }
}
