#![feature(trait_alias)]
#![feature(associated_type_bounds)]

pub mod attachable;
pub mod detachable;
pub mod emitter;
pub mod executable;
pub mod handler;
pub mod parameterized;
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
    pub use attachable::prelude::*;
    pub use detachable::prelude::*;
    pub use emitter::prelude::*;
    pub use executable::prelude::*;
    pub use handler::prelude::*;
    pub use transformer::prelude::*;
}
