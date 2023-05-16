use super::Command;

pub trait LambdaCommandDelegate = Fn() + 'static;

pub struct LambdaCommand<C>
where
    C: LambdaCommandDelegate,
{
    delegate: C,
}

impl<C> LambdaCommand<C>
where
    C: LambdaCommandDelegate,
{
    pub fn new(delegate: C) -> LambdaCommand<C>
    {
        LambdaCommand { delegate }
    }
}

impl<C> Command for LambdaCommand<C>
where
    C: LambdaCommandDelegate,
{
    fn execute(&self)
    {
        (self.delegate)();
    }
}

impl<C> From<C> for LambdaCommand<C>
where
    C: LambdaCommandDelegate,
{
    fn from(delegate: C) -> Self
    {
        LambdaCommand::new(delegate)
    }
}

#[cfg(test)]
mod tests
{
    use crate::command::{
        lambda_command::LambdaCommand,
        Command,
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

        let lc = LambdaCommand::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        });
        lc.execute();

        assert!((*has_run.borrow()));
    }
}
