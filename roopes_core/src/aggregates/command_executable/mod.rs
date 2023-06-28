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
    pub use super::CommandExecutable;
}

/// Bridges [`Command`]s and [`Executable`]s into
/// one type, which implements both traits.
pub struct CommandExecutable<E>
where
    E: Executable,
{
    executable: E,
}

impl<E> CommandExecutable<E>
where
    E: Executable,
{
    /// Creates a new [`CommandExecutable`] from
    /// the specified [`Executable`].
    pub fn new(executable: E) -> CommandExecutable<E>
    {
        CommandExecutable { executable }
    }
}

#[allow(clippy::inline_always)]
impl<E> Executable for CommandExecutable<E>
where
    E: Executable,
{
    delegate! {
        to self.executable {
           fn execute(&self);
        }
    }
}

#[allow(clippy::inline_always)]
impl<E> Command for CommandExecutable<E>
where
    E: Executable,
{
    delegate! {
        to self.executable {
            fn execute(&self);
        }
    }
}

impl<E> From<E> for CommandExecutable<E>
where
    E: Executable,
{
    fn from(executable: E) -> Self
    {
        CommandExecutable::new(executable)
    }
}
