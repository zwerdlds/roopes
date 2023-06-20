//! This module contains types which build on [`crate::patterns`] and
//! [`crate::primitives`].

pub mod executable_command;
pub mod observing_command;
pub mod subscribing_handler;

pub use executable_command::ExecutableCommand;
pub use observing_command::ObservingCommand;
pub use subscribing_handler::SubscribingHandler;

/// Provides types which are exposed at the library level.
pub mod prelude
{
    pub use super::{
        executable_command,
        observing_command,
        subscribing_handler,
    };
    pub use executable_command::prelude::*;
    pub use observing_command::prelude::*;
    pub use subscribing_handler::prelude::*;
}
