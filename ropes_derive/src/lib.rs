#![feature(proc_macro_internals)]
#![feature(trait_alias)]
#![feature(associated_type_bounds)]

use proc_macro::TokenStream;

mod builder;
mod visitor;

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream
{
    builder::derive::derive(input)
}

#[proc_macro_derive(Visitor)]
pub fn derive_visitor(input: TokenStream) -> TokenStream
{
    visitor::derive::derive(input)
}
