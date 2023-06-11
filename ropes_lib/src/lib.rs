#![feature(trait_alias)]
#![feature(associated_type_bounds)]

#[allow(unused_imports)]
#[macro_use]
extern crate ropes_derive;

#[allow(unused_imports)]
extern crate ropes_primitives;

pub mod aggregates;
pub mod patterns;

pub mod prelude
{
    pub use super::{
        aggregates,
        patterns,
    };
    pub use aggregates::*;
    pub use patterns::*;
    pub use ropes_derive::*;
    pub use ropes_primitives::prelude::*;
}
