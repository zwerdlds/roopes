//! The [`executable_command`] module creates
//! [`Executable`]s from arbitrary [`Command`]s.

use crate::prelude::*;
use delegate::delegate;

/// Exposes the default public types for the
/// [`executable_command`] module.
pub mod prelude
{
    pub use super::ExecutableCommand;
}

/// Bridges [`Command`]s and [`Executable`]s into
/// one type, which implements both traits.
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
    /// Creates a new [`ExecutableCommand`] from
    /// the specified [`Command`].
    pub fn new(command: C) -> ExecutableCommand<C>
    {
        ExecutableCommand { command }
    }
}

#[allow(clippy::inline_always)]
impl<C> Executable for ExecutableCommand<C>
where
    C: Command,
{
    delegate! {
        to self.command {
           fn execute(&self);
       }
    }
}

#[allow(clippy::inline_always)]
impl<C> Command for ExecutableCommand<C>
where
    C: Command,
{
    delegate! {
             to self.command {
            fn execute(&self);
        }
    }
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
