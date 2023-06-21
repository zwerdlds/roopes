//! This module supplies essential types used
//! elsewhere in the library.

pub mod emitter;
pub mod executable;
pub mod handler;
pub mod transformer;

/// Exposes [`emitter`], [`executable`],
/// [`handler`], and [`transformer`] submodules
/// and their preludes at the library level.
pub mod prelude
{
    pub use super::{
        emitter,
        executable,
        handler,
        transformer,
    };
    pub use emitter::prelude::*;
    pub use executable::prelude::*;
    pub use handler::prelude::*;
    pub use transformer::prelude::*;
}
