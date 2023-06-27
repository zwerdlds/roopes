//! This crate implements macros supporting some
//! patterns:
//! - [`Builder`]
//! - [`Visitor`]

#![feature(proc_macro_internals)]
#![feature(trait_alias)]
#![feature(associated_type_bounds)]

use proc_macro::TokenStream;

mod builder;
mod visitor;

/// Creates a new type on the specified `struct`,
/// which allows fields to be set one at a time.
/// The new builder type's name is the specified
/// type, appended with "Builder".
///
/// # Examples
/// ``` rust
/// #[macro_use]
/// use roopes::prelude::*;
///
/// #[derive(Builder)]
/// struct TestObj
/// {
///     field: i32,
/// }
///
/// let mut builder = TestObjBuilder::new();
///
/// builder.set_field(10);
///
/// let test_obj = builder.build();
///
/// assert_eq!(test_obj.field, 10);
/// ```
#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream
{
    builder::derive(input)
}
/// Creates a new trait for user code to implement
/// on the specified `enum`. This trait requires
/// implementors to implement handlers for all the
/// specified variants in the given enum.
///
/// # Examples
/// ``` rust
/// #[macro_use]
/// use roopes::prelude::*;
///
/// #[derive(Visitor)]
/// enum TestEnum
/// {
///     Integer
///     {
///         value: i32,
///     },
///     Nothing,
/// }
///
/// struct Implementor;
/// impl TestEnumVisitor for Implementor
/// {
///     fn visit_integer(
///         &self,
///         value: &i32,
///     )
///     {
///         println!("got {value}");
///     }
///
///     fn visit_nothing(&self)
///     {
///         println!("got Nothing");
///     }
/// }
///
/// let test_visitor = TestEnumAcceptor::new(Implementor);
/// test_visitor.accept(&TestEnum::Integer { value: 10 });
/// test_visitor.accept(&TestEnum::Nothing);
/// ```
#[proc_macro_derive(Visitor)]
pub fn derive_visitor(input: TokenStream) -> TokenStream
{
    visitor::derive(input)
}

#[test]
fn macro_tests()
{
    let t = trybuild::TestCases::new();
    t.compile_fail("src/*/test/*_fail.rs");
    t.pass("src/*/test/*_pass.rs");
}
