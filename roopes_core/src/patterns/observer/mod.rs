//! Contains types which implement the "Observer"
//! pattern, in which a single object is used to
//! delegate notifications to a group of dynamic
//! listening object.

pub mod hash_subject;
pub mod vec_subject;

use crate::prelude::*;
pub use hash_subject::HashSubject;
pub use vec_subject::VecSubject;

#[cfg(test)]
mod tests;

/// An object which notifies some group of
/// [`Observer`]s.  When it is notified,
/// all the listeners are notified.
pub trait Subject
{
    /// Triggers the [`Observer`]s corresponding
    /// `notify` implementations.
    fn notify(&self);
}

/// An object notified by a [`Subject`].
pub trait Observer
{
    /// Executes some arbitrary block in the
    /// implementing object.
    fn notify(&self);
}

/// Allows [`Observer`]s to be added to the
/// implementing [`Subject`].
pub trait AttachableSubject<O>: Subject
where
    O: Observer,
{
    /// Adds the [`Observer`] to the list of
    /// elements notified when the [`Subject`]
    /// is notified.
    fn attach(
        &mut self,
        attach_observer: O,
    );
}

/// Allows [`Observer`]s to be removed from the
/// implementing [`Subject`].
pub trait DetachableSubject<O, E>: Subject
where
    O: Observer,
{
    /// Detaches the given [`Observer`] from the
    /// [`Subject`] so it would no
    /// longer receive notifications.
    ///
    /// # Errors
    /// E: The error that occurred during
    /// detachment.
    fn detach(
        &mut self,
        detach_observer: &O,
    ) -> Result<(), E>;
}

/// Exposes the [`Observer`] and [`Subject`] types
/// at the library level.
pub mod prelude
{
    pub use super::{
        AttachableSubject,
        DetachableSubject,
        Observer,
        Subject,
    };
}
