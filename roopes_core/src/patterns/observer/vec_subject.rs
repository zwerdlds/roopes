//! Contains types which implement the Observer
//! pattern via an internal [`Vec`].

use super::{
    AttachableSubject,
    DetachableSubject,
    Observer,
    Subject,
};
use std::{
    borrow::BorrowMut,
    cell::RefCell,
};

/// Implements [`Subject`] backed by a [`Vec<T>`].
/// If `T` implements [`Eq`],
/// then [`DetachableSubject`] is also provided.
/// [`AttachableSubject`] is always provided.
///
///  # Examples
///  ``` rust
/// use roopes::prelude::*;
/// use std::{
///     cell::RefCell,
///     rc::Rc,
/// };
///
/// let mut vs = observer::VecSubject::default();
///
/// let has_run = Rc::new(RefCell::new(false));
/// {
///     let has_run = has_run.clone();
///
///     let lc: ObservingCommand<_> =
///         command::Executable::new_lambda(move
/// || {             (*has_run.borrow_mut()) =
/// true;         })
///         .into();
///
///     vs.attach(lc);
/// }
///
/// assert!(!(*has_run.borrow()));
/// vs.notify();
/// assert!((*has_run.borrow()));
///  ```

pub struct VecSubject<O>
where
    O: Observer,
{
    listeners: RefCell<Vec<O>>,
}

impl<O> VecSubject<O>
where
    O: Observer,
{
    /// Creates a new [`VecSubject`] with an
    /// existing list of listeners.
    /// [`VecSubject::default`] is probably
    /// preferable in most circumstances.
    #[must_use]
    pub fn new(listeners: RefCell<Vec<O>>) -> VecSubject<O>
    {
        VecSubject { listeners }
    }
}

impl<O> AttachableSubject<O> for VecSubject<O>
where
    O: Observer,
{
    fn attach(
        &mut self,
        attach_observer: O,
    )
    {
        self.listeners.borrow_mut().push(attach_observer);
    }
}

/// An Error which occurs during detachment.
#[derive(Debug)]
pub enum DetachError
{
    /// The specified observer couldn't be found.
    ObserverNotFound,
}
impl<O> DetachableSubject<O, DetachError> for VecSubject<O>
where
    O: Observer + Eq,
{
    fn detach(
        &mut self,
        detach_observer: &O,
    ) -> Result<(), DetachError>
    {
        let (i, _) = self
            .listeners
            .borrow()
            .iter()
            .enumerate()
            .find(|(_, o)| o.eq(&detach_observer))
            .ok_or(DetachError::ObserverNotFound)?;

        self.listeners.borrow_mut().swap_remove(i);

        Ok(())
    }
}

impl<O> Default for VecSubject<O>
where
    O: Observer,
{
    fn default() -> Self
    {
        Self::new(RefCell::default())
    }
}

impl<O> Subject for VecSubject<O>
where
    O: Observer,
{
    fn notify(&self)
    {
        self.listeners.borrow().iter().for_each(Observer::notify);
    }
}
