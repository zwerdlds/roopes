//! This module contains types which build on
//! [`crate::patterns`] and [`crate::primitives`].

pub mod command_executable;
pub mod executable_command;
pub mod handling_publisher;
pub mod observing_command;
pub mod subscribing_handler;
pub mod transforming_handler;

pub use command_executable::CommandExecutable;
pub use executable_command::ExecutableCommand;
pub use handling_publisher::HandlingPublisher;
pub use observing_command::ObservingCommand;
pub use subscribing_handler::SubscribingHandler;
pub use transforming_handler::TransformingHandler;

/// Provides types which are exposed at the
/// library level.
pub mod prelude
{
    pub use super::{
        command_executable,
        executable_command,
        handling_publisher,
        observing_command,
        subscribing_handler,
        transforming_handler,
    };
    pub use command_executable::prelude::*;
    pub use executable_command::prelude::*;
    pub use handling_publisher::prelude::*;
    pub use observing_command::prelude::*;
    pub use subscribing_handler::prelude::*;
    pub use transforming_handler::prelude::*;
}
