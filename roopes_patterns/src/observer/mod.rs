pub mod hash_subject;
pub mod vec_subject;

use crate::prelude::*;
pub use hash_subject::HashSubject;
use roopes_primitives::prelude::*;
pub use vec_subject::VecSubject;

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

/// An Error which occurs during detachment.
pub enum DetachError
{
    /// The specified observer couldn't be found.
    ObserverNotFound,
}

pub trait MutableSubject<O>:
    Attachable<O> + Detachable<O, (), DetachError> + Subject
where
    O: Observer,
{
}

pub mod prelude
{
    pub use super::{
        Observer,
        Subject,
    };
}
