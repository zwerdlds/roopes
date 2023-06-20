pub mod hash_subject;
pub mod vec_subject;

use crate::prelude::*;
pub use hash_subject::HashSubject;
pub use vec_subject::VecSubject;

#[cfg(test)]
mod tests;

/// An object which notifies some group of [`Observer`]s.  When it is notified,
/// all the listeners are notified.
pub trait Subject
{
    fn notify(&self);
}

/// An object notified by a [`Subject`].
pub trait Observer
{
    fn notify(&self);
}

pub trait AttachableSubject<O>: Subject
where
    O: Observer,
{
    fn attach(
        &mut self,
        attach_observer: O,
    );
}

pub trait DetachableSubject<O, E>: Subject
where
    O: Observer,
{
    /// Detaches the given [`Observer`] from the [`Subject`] so it would no
    /// longer receive notifications.
    ///
    /// # Errors
    /// E: The error that occurred during detachment.
    fn detach(
        &mut self,
        detach_observer: &O,
    ) -> Result<(), E>;
}

/// Exposes the [`Observer`] and [`Subject`] types at the library level.
pub mod prelude
{
    pub use super::{
        AttachableSubject,
        DetachableSubject,
        Observer,
        Subject,
    };
}
