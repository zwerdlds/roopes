use super::{
    Attachable,
    DetachError,
    Detachable,
    MutableSubject,
    Observer,
    Subject,
};

/// Implements [`Subject`] backed by a [`Vec<T>`].  If `T` implements [`Eq`],
/// then [`Detachable`] is also implemented.  [`Attachable`] is always
/// implemented.
//
//  # Examples
//  ``` rust
// use roopes_lib::prelude::*;
// use std::{
//     cell::RefCell,
//     rc::Rc,
// };
// use ropes_primitives::prelude::*;
//
// let mut vs = observer::VecSubject::default();
//
// let has_run = Rc::new(RefCell::new(false));
// {
//     let has_run = has_run.clone();
//
//     let lc: ObservingCommand<_> =
//         command::Lambda::new(move || {
//             (*has_run.borrow_mut()) = true;
//         })
//         .into();
//
//     vs.attach(lc);
// }
//
// assert!(!(*has_run.borrow()));
// vs.notify();
// assert!((*has_run.borrow()));
//  ```
pub struct VecSubject<O>
where
    O: Observer,
{
    listeners: Vec<O>,
}

impl<O> VecSubject<O>
where
    O: Observer,
{
    #[must_use]
    pub fn new(listeners: Vec<O>) -> VecSubject<O>
    {
        VecSubject { listeners }
    }
}

impl<O> Default for VecSubject<O>
where
    O: Observer,
{
    fn default() -> Self
    {
        Self::new(Vec::new())
    }
}

impl<O> Subject for VecSubject<O>
where
    O: Observer,
{
    fn notify(&self)
    {
        self.listeners.iter().for_each(Observer::notify);
    }
}

impl<O> Attachable<O> for VecSubject<O>
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

impl<O> Detachable<O, (), DetachError> for VecSubject<O>
where
    O: Observer + Eq,
{
    fn detach(
        &mut self,
        detach_observer: O,
    ) -> Result<(), DetachError>
    {
        let (i, _) = self
            .listeners
            .iter()
            .enumerate()
            .find(|(_, o)| o.eq(&&detach_observer))
            .ok_or(DetachError::ObserverNotFound)?;

        self.listeners.swap_remove(i);

        Ok(())
    }
}

impl<O> MutableSubject<O> for VecSubject<O> where O: Observer + Eq {}
