//!
#![cfg_attr(feature = "doc-images",
  cfg_attr(
    all(),
    doc = ::embed_doc_image::embed_image!(
        "abstract-factory-diagram",
        "src/patterns/abstract_factory/abstract_factory.svg"
)))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and \
           Rust version >= 1.54 to enable."
)]
//! Provides an abstraction for object creation.
//!
//! ![abstract factory diagram][abstract-factory-diagram]

pub mod lambda;

pub use lambda::Lambda;

/// This trait enables the abstraction of the
/// creation of objects.
pub trait AbstractFactory<T>
{
    /// Executes the creation logic and provides
    /// the type back to the caller.
    fn create(&self) -> T;
}

/// Exposes [`AbstractFactory`] at the library level.
pub mod prelude
{
    pub use super::AbstractFactory;
}
