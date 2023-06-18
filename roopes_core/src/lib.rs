#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![allow(unused_imports)]

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
    pub use aggregates::prelude::*;
    pub use patterns::prelude::*;
    pub use primitives::prelude::*;
}
