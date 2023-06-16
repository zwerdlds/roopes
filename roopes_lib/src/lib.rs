#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![allow(unused_imports)]

#[macro_use]
extern crate roopes_derive;

extern crate roopes_aggregates;

extern crate roopes_patterns;

extern crate roopes_primitives;

pub mod prelude
{
    pub use roopes_aggregates::prelude::*;
    pub use roopes_derive::*;
    pub use roopes_patterns::prelude::*;
    pub use roopes_primitives::prelude::*;
}
