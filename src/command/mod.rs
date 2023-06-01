use crate::executable::Executable;

pub mod hashable_command;
pub mod lambda_command;

pub trait Command
{
    fn execute(&self);
}

impl<E> Executable for E
where
    E: Command,
{
    fn execute(&self)
    {
        Command::execute(self);
    }
}
