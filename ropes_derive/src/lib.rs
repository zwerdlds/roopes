#![feature(trait_alias)]
#![feature(associated_type_bounds)]

use proc_macro::TokenStream;

mod aggregates;
mod patterns;
mod primitives;

use patterns::builder;

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream
{
    builder::build_tokens(input)
}
