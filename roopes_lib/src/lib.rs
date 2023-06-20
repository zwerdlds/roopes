#![doc = include_str!("../../readme.md")]
#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![allow(unused_imports)]

#[macro_use]
extern crate roopes_derive;

extern crate roopes_core;

/// Exposes the most used types from the library.
pub mod prelude
{
    pub use roopes_core::prelude::*;
    pub use roopes_derive::{
        Builder,
        Visitor,
    };
}
