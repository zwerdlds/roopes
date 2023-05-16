use super::{
    Attachable,
    Detachable,
    Observer,
    ObserverDetachError,
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
/// use roopes_lib::{
///     command::lambda_command::LambdaCommand,
///     observer::{
///         vector_subject::VectorSubject,
///         Attachable,
///         Subject,
///     },
/// };
/// use std::{
///     cell::RefCell,
///     rc::Rc,
/// };
///
/// let mut hs = VectorSubject::new();
///
/// let has_run = Rc::new(RefCell::new(false));
/// {
///     let has_run = has_run.clone();
///
///     let lc = LambdaCommand::new(move || {
///         (*has_run.borrow_mut()) = true;
///     });
///
///     hs.attach(lc);
/// }
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
    pub fn new() -> HashSubject<O>
    {
        let listeners = HashSet::new();

        HashSubject { listeners }
    }
}

impl<O> Subject for HashSubject<O>
where
    O: HashSetObserver,
{
    fn notify(&self)
    {
        self.listeners.iter().for_each(|l| l.notify());
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

impl<O> Detachable<O> for HashSubject<O>
where
    O: HashSetObserver,
{
    fn detach(
        &mut self,
        detach_observer: O,
    ) -> Result<(), ObserverDetachError>
    {
        if !self.listeners.remove(&detach_observer) {
            Err(ObserverDetachError::ObserverNotFound)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests
{
    use crate::{
        command::{
            hashable_command::HashableCommand,
            lambda_command::LambdaCommand,
        },
        observer::{
            hash_subject::HashSubject,
            Attachable,
            Subject,
        },
    };
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
        let mut hs = HashSubject::new();

        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = LambdaCommand::new(move || {
            {
                (*has_run_ext.borrow_mut()) = true;
            }
        });

        let hc = HashableCommand::new(lc, TestCommands::HasRun);

        hs.attach(hc);

        hs.notify();

        assert!((*has_run.borrow()));
    }

    #[test]
    fn toggle_hashset_subject_notify()
    {
        let mut hs = HashSubject::new();

        let has_run_toggle = Rc::new(RefCell::new(false));
        let has_run_toggle_ext = has_run_toggle.clone();

        let lc = LambdaCommand::new(move || {
            {
                let tgl = *has_run_toggle_ext.borrow();
                (*has_run_toggle_ext.borrow_mut()) = !tgl;
            }
            .into()
        });

        let hc = HashableCommand::new(lc, TestCommands::HasRunToggle);

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
        let mut hs = HashSubject::new();

        let has_run_1 = Rc::new(RefCell::new(false));
        let has_run_1_ext = has_run_1.clone();

        let lc = LambdaCommand::new(move || {
            (*has_run_1_ext.borrow_mut()) = true;
        });
        let hc = HashableCommand::new(lc, TestCommands::HasRun);

        hs.attach(hc);
        assert!(!(*has_run_1.borrow()));

        hs.notify();
        assert!((*has_run_1.borrow()));

        let mut hs = HashSubject::new();

        let has_run_2 = Rc::new(RefCell::new(false));
        let has_run_2_ext = has_run_2.clone();

        let lc = LambdaCommand::new(move || {
            (*has_run_2_ext.borrow_mut()) = true;
        });
        let hc = HashableCommand::new(lc, TestCommands::HasRunTwo);

        hs.attach(hc);

        assert!((*has_run_1.borrow()));
        assert!(!(*has_run_2.borrow()));

        hs.notify();

        assert!((*has_run_1.borrow()));
        assert!((*has_run_2.borrow()));
    }

    #[test]
    fn overwrite_hashset_subject_notify()
    {
        let mut hs = HashSubject::new();

        let has_run_1 = Rc::new(RefCell::new(false));
        let has_run_1_ext = has_run_1.clone();

        let lc = LambdaCommand::new(move || {
            (*has_run_1_ext.borrow_mut()) = true;
        });

        let hc = HashableCommand::new(lc, TestCommands::HasRun);

        hs.attach(hc);

        let mut hs = HashSubject::new();

        let has_run_2 = Rc::new(RefCell::new(false));
        let has_run_2_ext = has_run_2.clone();

        let lc = LambdaCommand::new(move || {
            (*has_run_2_ext.borrow_mut()) = true;
        });

        let hc = HashableCommand::new(lc, TestCommands::HasRun);

        hs.attach(hc);

        assert!(!(*has_run_1.borrow()));
        assert!(!(*has_run_2.borrow()));

        hs.notify();

        assert!(!(*has_run_1.borrow()));
        assert!((*has_run_2.borrow()));
    }
}
