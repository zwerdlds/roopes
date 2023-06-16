#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![allow(unused_imports)]

pub mod abstract_factory;
pub mod command;
pub mod heap_pool;
pub mod observer;
pub mod publisher_subscriber;
pub mod state;
pub mod transformer_chain;

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
