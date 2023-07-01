//! Contains types which allow [`Observer`]s
//! (which implement the appropriate [`Hash`] and
//! [`Eq`] traits) to be removed dynamically.

use super::{
    Observer,
    Subject,
};
use crate::prelude::*;
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashSet,
    hash::Hash,
};

/// Convenience type representing types which can
/// be added and removed from the
/// [`HashSetObserver`].
pub trait HashSetObserver = Observer + Eq + Hash;

/// Implements [`Subject`] backed by a
/// [`HashSet<T>`]. `T` must implement
/// [`PartialEq`] and  [`Hash`].
///
/// # Examples
/// ``` rust
/// use roopes::prelude::*;
/// use std::{
///     cell::RefCell,
///     rc::Rc,
/// };
/// use enclose::enclose;
///
/// let mut hs = observer::HashSubject::default();
///
/// let has_run = Rc::new(RefCell::new(false));
/// let lc: ObservingCommand<_> = command::Hashable::new(
///     command::Heap::from(enclose!((has_run) move || {
///         (*has_run.borrow_mut()) = true;
///     })),
///     "Has Run").into();
///
/// hs.attach(lc);
///
/// assert!(!(*has_run.borrow()));
/// hs.notify();
/// assert!((*has_run.borrow()));
/// ```
pub struct HashSubject<O>
where
    O: HashSetObserver,
{
    listeners: HashSet<O>,
}

impl<O> Default for HashSubject<O>
where
    O: HashSetObserver,
{
    fn default() -> HashSubject<O>
    {
        Self::new(HashSet::default())
    }
}

impl<O> HashSubject<O>
where
    O: HashSetObserver,
{
    /// Creates a new [`HashSubject`] with an
    /// interior-mutable listener set.
    /// [`HashSubject::default`] is probably
    /// preferable in most situations.
    #[must_use]
    pub fn new(listeners: HashSet<O>) -> HashSubject<O>
    {
        HashSubject { listeners }
    }
}

impl<O> AttachableSubject<O> for HashSubject<O>
where
    O: HashSetObserver,
{
    fn attach(
        &mut self,
        attach_observer: O,
    )
    {
        self.listeners.borrow_mut().insert(attach_observer);
    }
}

/// An Error which occurs during detachment.
#[derive(Debug)]
pub enum DetachError
{
    /// The specified observer couldn't be found.
    ObserverNotFound,
}

impl<O> DetachableSubject<O, DetachError> for HashSubject<O>
where
    O: HashSetObserver,
{
    fn detach(
        &mut self,
        detach_observer: &O,
    ) -> Result<(), DetachError>
    {
        if self.listeners.remove(detach_observer) {
            Ok(())
        } else {
            Err(DetachError::ObserverNotFound)
        }
    }
}

impl<O> Subject for HashSubject<O>
where
    O: HashSetObserver,
{
    fn notify(&self)
    {
        self.listeners.iter().for_each(Observer::notify);
    }
}
