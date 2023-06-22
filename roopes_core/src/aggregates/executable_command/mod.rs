//!
#![cfg_attr(feature = "doc-images",
  cfg_attr(
    all(),
    doc = ::embed_doc_image::embed_image!(
        "executable-command-diagram",
        "src/aggregates/executable_command/executable_command.svg"
)))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and \
           Rust version >= 1.54 to enable."
)]
//!  The [`executable_command`] module creates
//! [`Executable`]s from arbitrary [`Command`]s.
//! ![executable command diagram][executable-command-diagram]

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
