use super::{
    Attachable,
    Detachable,
    Observer,
    ObserverDetachError,
    Subject,
};

/// Implements [`Subject`] backed by a [`Vec<T>`].  If `T` implements [`Eq`],
/// then [`Detachable`] is also implemented.  [`Attachable`] is always
/// implemented.
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
/// let mut vs = VectorSubject::default();
///
/// let has_run = Rc::new(RefCell::new(false));
/// {
///     let has_run = has_run.clone();
///
///     let lc = LambdaCommand::new(move || {
///         (*has_run.borrow_mut()) = true;
///     });
///
///     vs.attach(lc);
/// }
///
/// assert!(!(*has_run.borrow()));
/// vs.notify();
/// assert!((*has_run.borrow()));
/// ```
pub struct VectorSubject<O>
where
    O: Observer,
{
    listeners: Vec<O>,
}

impl<O> VectorSubject<O>
where
    O: Observer,
{
    pub fn new(listeners: Vec<O>) -> VectorSubject<O>
    {
        VectorSubject { listeners }
    }
}

impl<O> Default for VectorSubject<O>
where
    O: Observer,
{
    fn default() -> Self
    {
        Self::new(Vec::new())
    }
}

impl<O> Subject for VectorSubject<O>
where
    O: Observer,
{
    fn notify(&self)
    {
        self.listeners.iter().for_each(|l| l.notify());
    }
}

impl<O> Attachable<O> for VectorSubject<O>
where
    O: Observer,
{
    fn attach(
        &mut self,
        attach_observer: O,
    )
    {
        self.listeners.push(attach_observer);
    }
}

impl<O> Detachable<O> for VectorSubject<O>
where
    O: Observer + Eq,
{
    fn detach(
        &mut self,
        detach_observer: O,
    ) -> Result<(), ObserverDetachError>
    {
        let (i, _) = self
            .listeners
            .iter()
            .enumerate()
            .find(|(_, o)| o.eq(&&detach_observer))
            .ok_or(ObserverDetachError::ObserverNotFound)?;

        self.listeners.swap_remove(i);

        Ok(())
    }
}

#[cfg(test)]
mod tests
{
    use crate::{
        command::lambda_command::LambdaCommand,
        observer::{
            vector_subject::VectorSubject,
            Attachable,
            Subject,
        },
    };
    use std::{
        cell::RefCell,
        rc::Rc,
    };

    #[test]
    fn simple_vector_subject_notify()
    {
        let mut vs = VectorSubject::default();

        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = LambdaCommand::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        });

        vs.attach(lc);

        vs.notify();

        assert!((*has_run.borrow()));
    }

    #[test]
    fn toggle_vector_subject_notify()
    {
        let mut vs = VectorSubject::default();

        let has_run_toggle = Rc::new(RefCell::new(false));
        let has_run_toggle_ext = has_run_toggle.clone();

        let lc = LambdaCommand::new(move || {
            let tgl = *has_run_toggle_ext.borrow();

            (*has_run_toggle_ext.borrow_mut()) = !tgl;
        });

        vs.attach(lc);

        assert!(!(*has_run_toggle.borrow()));

        vs.notify();
        assert!((*has_run_toggle.borrow()));

        vs.notify();
        assert!(!(*has_run_toggle.borrow()));

        vs.notify();
        assert!((*has_run_toggle.borrow()));
    }

    #[test]
    fn multiple_vector_subject_notify()
    {
        let mut vs = VectorSubject::default();

        let has_run_1 = Rc::new(RefCell::new(false));
        let has_run_1_ext = has_run_1.clone();

        let lc = LambdaCommand::new(move || {
            (*has_run_1_ext.borrow_mut()) = true;
        });

        vs.attach(lc);

        assert!(!(*has_run_1.borrow()));

        vs.notify();
        assert!((*has_run_1.borrow()));

        let mut vs = VectorSubject::default();

        let has_run_2 = Rc::new(RefCell::new(false));
        let has_run_2_ext = has_run_2.clone();

        let lc = LambdaCommand::new(move || {
            (*has_run_2_ext.borrow_mut()) = true;
        });

        vs.attach(lc);

        assert!((*has_run_1.borrow()));
        assert!(!(*has_run_2.borrow()));

        vs.notify();

        assert!((*has_run_1.borrow()));
        assert!((*has_run_2.borrow()));
    }
}
