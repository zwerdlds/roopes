#![feature(trait_alias)]
#![feature(associated_type_bounds)]

pub mod attachable;
pub mod detachable;
pub mod emitter;
pub mod executable;
pub mod handler;
pub mod transformer;

pub mod prelude
{
    pub use super::{
        attachable,
        detachable,
        emitter,
        executable,
        handler,
        transformer,
    };
    pub use attachable::Attachable;
    pub use detachable::Detachable;
    pub use emitter::Emitter;
    pub use executable::Executable;
    pub use handler::Handler;
    pub use transformer::Transformer;
}
