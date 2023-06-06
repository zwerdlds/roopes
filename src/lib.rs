#![feature(trait_alias)]
#![feature(associated_type_bounds)]

pub mod aggregates;
pub mod patterns;
pub mod primitives;

pub mod prelude
{
    use crate::{
        aggregates,
        patterns,
        primitives,
    };
    pub use aggregates::*;
    pub use patterns::*;
    pub use primitives::*;
}
