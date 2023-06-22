//!
#![cfg_attr(feature = "doc-images",
  cfg_attr(
    all(),
    doc = ::embed_doc_image::embed_image!(
        "handler-diagram",
        "src/primitives/handler/handler.svg"
)))]
//! Provides types which receive a borrowed value.
//!
//! ![handler diagram][handler-diagram]

pub mod hash;
pub mod heap;
pub mod lambda;

pub use hash::Hashable;
pub use heap::Heap;
pub use lambda::Lambda;

#[cfg(test)]
mod tests;

/// Defines a primitive interface which handles
/// messages via borrowing.
pub trait Handler<M>
{
    /// Receives a borrowed value.
    fn handle(
        &self,
        message: &M,
    );
}

// impl<C, M> From<Lambda<C, M>> for Heap<M>
// where
//     C: Delegate<M> + 'static,
//     M: 'static,
// {
//     fn from(handler: Lambda<C, M>) -> Heap<M>
//     {
//         Heap::new(Box::new(handler))
//     }
// }

/// Exposes the [`Handler`] type at the library
/// level.
pub mod prelude
{
    pub use super::Handler;
}
