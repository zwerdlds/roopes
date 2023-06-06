use super::Command;
use std::hash::Hash;

pub trait Delegate = Command;
pub trait Subject = Hash + Eq;

#[derive(Debug)]
pub struct Hashable<D, H>
where
    D: Delegate,
    H: Subject,
{
    delegate: D,
    id: H,
}

impl<D, H> Hashable<D, H>
where
    D: Delegate,
    H: Subject,
{
    pub fn new(
        delegate: D,
        id: H,
    ) -> Hashable<D, H>
    {
        Hashable { delegate, id }
    }
}

impl<D, H> Eq for Hashable<D, H>
where
    D: Delegate,
    H: Subject,
{
}

impl<D, H> PartialEq<Self> for Hashable<D, H>
where
    D: Delegate,
    H: Subject,
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.id.eq(&other.id)
    }
}

impl<D, H> Hash for Hashable<D, H>
where
    D: Delegate,
    H: Subject,
{
    fn hash<R: std::hash::Hasher>(
        &self,
        state: &mut R,
    )
    {
        self.id.hash(state);
    }
}

impl<D, H> Command for Hashable<D, H>
where
    D: Delegate,
    H: Subject,
{
    fn execute(&self)
    {
        self.delegate.execute();
    }
}

#[cfg(test)]
mod tests
{
    use crate::prelude::*;
    use std::{
        cell::RefCell,
        rc::Rc,
    };

    #[test]
    fn simple_hashable_command_refcell_mutation()
    {
        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = command::Lambda::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        });

        let hc = command::Hashable::new(lc, "test");

        hc.execute();

        assert!((*has_run.borrow()));
    }
}
