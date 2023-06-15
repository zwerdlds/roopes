use super::{
    DetachError,
    Observer,
    Subject,
};
use ropes_primitives::prelude::*;
use std::{
    collections::HashSet,
    hash::Hash,
};

pub trait HashSetObserver = Observer + Eq + Hash;

/// Implements [`Subject`] backed by a [`HashSet<T>`]. `T` must implement
/// [`PartialEq`] and  [`Hash`].
///
/// # Examples
/// ``` rust
/// use ropes::prelude::*;
/// use std::{
///     cell::RefCell,
///     rc::Rc,
/// };
/// use enclose::enclose;
///
/// let mut hs = observer::HashSubject::default();
///
/// let has_run = Rc::new(RefCell::new(false));
/// let lc:ObservingCommand<_> = command::Hashable::new(
///     command::Lambda::new(enclose!((has_run) move || {
///         (*has_run.borrow_mut()) = true;
///     })), "Has Run").into();
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

impl<O> HashSubject<O>
where
    O: HashSetObserver,
{
    #[must_use]
    pub fn new(listeners: HashSet<O>) -> HashSubject<O>
    {
        HashSubject { listeners }
    }
}

impl<O> Default for HashSubject<O>
where
    O: HashSetObserver,
{
    fn default() -> HashSubject<O>
    {
        let listeners = HashSet::new();

        HashSubject::new(listeners)
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

impl<O> Attachable<O> for HashSubject<O>
where
    O: HashSetObserver,
{
    fn attach(
        &mut self,
        attach_observer: O,
    )
    {
        self.listeners.insert(attach_observer);
    }
}

impl<O> Detachable<O, (), DetachError> for HashSubject<O>
where
    O: HashSetObserver,
{
    fn detach(
        &mut self,
        detach_observer: O,
    ) -> Result<(), DetachError>
    {
        if self.listeners.remove(&detach_observer) {
            Ok(())
        } else {
            Err(DetachError::ObserverNotFound)
        }
    }
}
