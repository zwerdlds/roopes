#![feature(proc_macro_internals)]
#![feature(trait_alias)]
#![feature(associated_type_bounds)]

use proc_macro::TokenStream;

mod builder;

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream
{
    builder::derive::derive(input)
}

pub(crate) mod prelude
{
    pub(crate) use super::builder;
    pub(crate) use builder::Builder;
}
