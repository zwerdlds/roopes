#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![allow(unused_imports)]

#[macro_use]
extern crate ropes_derive;

extern crate ropes_aggregates;

extern crate ropes_patterns;

extern crate ropes_primitives;

pub mod prelude
{
    pub use ropes_aggregates::prelude::*;
    pub use ropes_derive::*;
    pub use ropes_patterns::prelude::*;
    pub use ropes_primitives::prelude::*;
}
