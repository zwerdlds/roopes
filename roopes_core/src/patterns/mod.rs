//! Exposes types which conform most directly to
//! the "Gang of Four" and other commonly accepted
//! OOP-style patterns.

pub mod abstract_factory;
pub mod command;
pub mod heap_pool;
pub mod observer;
pub mod publisher_subscriber;
pub mod state;
pub mod transformer_chain;

/// Exposes submodules and their preludes at the
/// library level.
pub mod prelude
{
    pub use super::{
        abstract_factory,
        command,
        heap_pool,
        observer,
        publisher_subscriber,
        state,
        transformer_chain,
    };
    pub use abstract_factory::prelude::*;
    pub use command::prelude::*;
    pub use heap_pool::prelude::*;
    pub use observer::prelude::*;
    pub use publisher_subscriber::prelude::*;
    pub use state::prelude::*;
}
