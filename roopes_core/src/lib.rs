//! The main container library for `roopes`.  All
//! statically-defined (non-macro)
//! types the library provides are contained in
//! this module.

#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![allow(unused_imports)]

pub mod aggregates;
pub mod patterns;
pub mod primitives;

/// The default public types and traits for the
/// roopes library.
pub mod prelude
{
    use crate::{
        aggregates,
        patterns,
        primitives,
    };
    pub use aggregates::prelude::*;
    pub use patterns::prelude::*;
    pub use primitives::prelude::*;
}
