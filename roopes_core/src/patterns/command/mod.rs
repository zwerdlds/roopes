//!
#![cfg_attr(feature = "doc-images",
  cfg_attr(
    all(),
    doc = ::embed_doc_image::embed_image!(
        "command-diagram",
        "src/patterns/command/command.svg"
)))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and \
           Rust version >= 1.54 to enable."
)]
//! Contains types which encapsulate a repeatedly
//! callable block of code.
//!
//! ![command diagram][command-diagram]

pub mod executable;
pub mod hashable;
pub mod heap;

pub use executable::Executable;
pub use hashable::Hashable;
pub use heap::Heap;

/// Encapsulates a repeatedly callable block of
/// code.
pub trait Command
{
    /// Calls the encapsulated block of code.
    fn execute(&self);
}

/// Exposes the [`Command`] type at the library
/// level.
pub mod prelude
{
    pub use super::Command;
}
