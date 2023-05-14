pub mod lambda_command;

pub trait Command
{
    fn execute(&self);
}
