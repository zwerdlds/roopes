pub mod attachable;
pub mod detachable;
pub mod emitter;
pub mod executable;
pub mod handler;
pub mod parameterized;
pub mod receiver;
pub mod transformer;

pub mod prelude
{
    pub use super::{
        attachable,
        detachable,
        emitter,
        executable,
        handler,
        receiver,
        transformer,
    };
    pub use attachable::prelude::*;
    pub use detachable::prelude::*;
    pub use emitter::prelude::*;
    pub use executable::prelude::*;
    pub use handler::prelude::*;
    pub use receiver::prelude::*;
    pub use transformer::prelude::*;
}
