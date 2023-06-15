use crate::prelude::*;
use delegate::delegate;
use ropes_patterns::prelude::*;
use ropes_primitives::prelude::*;

pub struct ExecutableCommand<C>
where
    C: Command,
{
    command: C,
}

impl<C> ExecutableCommand<C>
where
    C: Command,
{
    pub fn new(command: C) -> ExecutableCommand<C>
    {
        ExecutableCommand { command }
    }
}

impl<C> Executable for ExecutableCommand<C>
where
    C: Command,
{
    delegate! { to self.command {
        fn execute(&self);
    } }
}

impl<C> From<C> for ExecutableCommand<C>
where
    C: Command,
{
    fn from(command: C) -> Self
    {
        ExecutableCommand::new(command)
    }
}