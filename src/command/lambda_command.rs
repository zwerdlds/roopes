use super::Command;

pub trait LambdaCommandDelegate = Fn() -> () + 'static;

pub struct LambdaCommand
{
    delegate: Box<dyn LambdaCommandDelegate>,
}

impl LambdaCommand
{
    pub fn new<F>(delegate: F) -> LambdaCommand
    where
        F: LambdaCommandDelegate,
    {
        let delegate = Box::new(delegate);

        LambdaCommand { delegate }
    }
}

impl Command for LambdaCommand
{
    fn execute(&self)
    {
        (self.delegate)();
    }
}

impl<T> From<T> for LambdaCommand
where
    T: LambdaCommandDelegate,
{
    fn from(delegate: T) -> Self
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
