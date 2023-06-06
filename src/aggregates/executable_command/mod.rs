use crate::{
    patterns::command::Command,
    primitives::executable::Executable,
};

pub struct ExecutableCommand<C>
where
    C: Command,
{
    cmd: C,
}

impl<C> Executable for ExecutableCommand<C>
where
    C: Command,
{
    fn execute(&self)
    {
        self.cmd.execute();
    }
}
