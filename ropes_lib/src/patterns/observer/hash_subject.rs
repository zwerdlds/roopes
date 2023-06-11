use super::{
    Attachable,
    DetachError,
    Detachable,
    Observer,
    Subject,
};
use std::{
    collections::HashSet,
    hash::Hash,
};

pub trait HashSetObserver = Observer + Eq + Hash;

/// Implements [`Subject`] backed by a [`HashSet<T>`]. `T` must implement
/// [`PartialEq`] and  [`Hash`].
///
/// # Examples
/// ```
/// use ropes_lib::prelude::*;
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

impl<O> Detachable<O, DetachError> for HashSubject<O>
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

#[cfg(test)]
mod tests
{
    use crate::prelude::*;
    use enclose::enclose;
    use std::{
        cell::RefCell,
        rc::Rc,
    };
    #[derive(Hash, PartialEq, Eq)]
    enum TestCommands
    {
        HasRun,
        HasRunToggle,
        HasRunTwo,
    }

    #[test]
    fn simple_hashset_subject_notify()
    {
        let mut hs = observer::HashSubject::default();

        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = command::Lambda::new(move || {
            {
                (*has_run_ext.borrow_mut()) = true;
            }
        });

        let hc: observing_command::ObservingCommand<_> =
            command::Hashable::new(lc, TestCommands::HasRun).into();

        hs.attach(hc);

        hs.notify();

        assert!((*has_run.borrow()));
    }

    #[test]
    fn toggle_hashset_subject_notify()
    {
        let mut hs = observer::HashSubject::default();

        let has_run_toggle = Rc::new(RefCell::new(false));
        let lc = command::Lambda::new(enclose!(
            (has_run_toggle) move || {
                {
                    let tgl = *has_run_toggle.borrow();
                    (*has_run_toggle.borrow_mut()) = !tgl;
                }
                .into()
            }
        ));

        let hc: ObservingCommand<_> =
            command::Hashable::new(lc, TestCommands::HasRunToggle).into();

        assert!(!(*has_run_toggle.borrow()));
        hs.attach(hc);

        hs.notify();
        assert!((*has_run_toggle.borrow()));

        hs.notify();
        assert!(!(*has_run_toggle.borrow()));

        hs.notify();
        assert!((*has_run_toggle.borrow()));
    }

    #[test]
    fn multiple_hashset_subject_notify()
    {
        let mut hs = observer::HashSubject::default();

        let has_run_1 = Rc::new(RefCell::new(false));

        let lc = command::Lambda::new(enclose!(
            (has_run_1) move || {
                (*has_run_1.borrow_mut()) = true;
            }
        ));

        let hc: ObservingCommand<_> =
            command::Hashable::new(lc, TestCommands::HasRun).into();

        hs.attach(hc);
        assert!(!(*has_run_1.borrow()));

        hs.notify();
        assert!((*has_run_1.borrow()));

        let mut hs: observer::HashSubject<ObservingCommand<_>> =
            Default::default();

        let has_run_2 = Rc::new(RefCell::new(false));

        let lc = command::Lambda::new(enclose!(
            (has_run_2) move || {
                (*has_run_2.borrow_mut()) = true;
            }
        ));
        let hc = command::Hashable::new(lc, TestCommands::HasRunTwo);

        hs.attach(hc.into());

        assert!((*has_run_1.borrow()));
        assert!(!(*has_run_2.borrow()));

        hs.notify();

        assert!((*has_run_1.borrow()));
        assert!((*has_run_2.borrow()));
    }

    #[test]
    fn overwrite_hashset_subject_notify()
    {
        let mut hs = observer::HashSubject::default();

        let has_run_1 = Rc::new(RefCell::new(false));

        let lc = command::Lambda::new(enclose!(
            (has_run_1) move || {
                (*has_run_1.borrow_mut()) = true;
            }
        ));

        let hc: ObservingCommand<_> =
            command::Hashable::new(lc, TestCommands::HasRun).into();

        hs.attach(hc);

        let mut hs: observer::HashSubject<ObservingCommand<_>> =
            observer::HashSubject::default();

        let has_run_2 = Rc::new(RefCell::new(false));

        let lc = command::Lambda::new(enclose!(
            (has_run_2) move || {
                (*has_run_2.borrow_mut()) = true;
            }
        ));

        let hc = command::Hashable::new(lc, TestCommands::HasRun).into();

        hs.attach(hc);

        assert!(!(*has_run_1.borrow()));
        assert!(!(*has_run_2.borrow()));

        hs.notify();

        assert!(!(*has_run_1.borrow()));
        assert!((*has_run_2.borrow()));
    }
}
