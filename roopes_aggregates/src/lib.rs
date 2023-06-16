#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![allow(unused_imports)]

pub mod aggregates;

pub mod prelude
{
    pub use super::aggregates;
    pub use aggregates::*;
}
