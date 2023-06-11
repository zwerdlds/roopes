use crate::prelude::*;
use std::hash::Hash;

pub struct ObservingCommand<C>
where
    C: Command,
{
    command: C,
}

impl<C> ObservingCommand<C>
where
    C: Command,
{
    pub fn new(command: C) -> ObservingCommand<C>
    {
        ObservingCommand { command }
    }
}

impl<C> From<C> for ObservingCommand<C>
where
    C: Command,
{
    fn from(command: C) -> Self
    {
        ObservingCommand::new(command)
    }
}

impl<C> Observer for ObservingCommand<C>
where
    C: Command,
{
    fn notify(&self)
    {
        self.command.execute();
    }
}

impl<C> Command for ObservingCommand<C>
where
    C: Command,
{
    fn execute(&self)
    {
        self.command.execute();
    }
}

impl<C> PartialEq for ObservingCommand<C>
where
    C: PartialEq + Command,
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.command == other.command
    }
}

impl<C> Eq for ObservingCommand<C> where C: Eq + Command {}

impl<C> Hash for ObservingCommand<C>
where
    C: Command + Hash,
{
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    )
    {
        self.command.hash(state);
    }
}
