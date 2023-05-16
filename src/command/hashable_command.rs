use super::Command;
use std::hash::Hash;

pub trait HashableCommandDelegate = Command;
pub trait HashSubject = Hash + Eq;

#[derive(Debug)]
pub struct HashableCommand<D, H>
where
    D: HashableCommandDelegate,
    H: HashSubject,
{
    delegate: D,
    id: H,
}

impl<D, H> HashableCommand<D, H>
where
    D: HashableCommandDelegate,
    H: HashSubject,
{
    pub fn new(
        delegate: D,
        id: H,
    ) -> HashableCommand<D, H>
    {
        HashableCommand { delegate, id }
    }
}

impl<D, H> Eq for HashableCommand<D, H>
where
    D: HashableCommandDelegate,
    H: HashSubject,
{
}

impl<D, H> PartialEq<Self> for HashableCommand<D, H>
where
    D: HashableCommandDelegate,
    H: HashSubject,
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.id.eq(&other.id)
    }
}

impl<D, H> Hash for HashableCommand<D, H>
where
    D: HashableCommandDelegate,
    H: HashSubject,
{
    fn hash<R: std::hash::Hasher>(
        &self,
        state: &mut R,
    )
    {
        self.id.hash(state);
    }
}

impl<D, H> Command for HashableCommand<D, H>
where
    D: HashableCommandDelegate,
    H: HashSubject,
{
    fn execute(&self)
    {
        self.delegate.execute();
    }
}

#[cfg(test)]
mod tests
{
    use crate::command::{
        hashable_command::HashableCommand,
        lambda_command::LambdaCommand,
        Command,
    };
    use std::{
        cell::RefCell,
        rc::Rc,
    };

    #[test]
    fn simple_hashable_command_refcell_mutation()
    {
        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = LambdaCommand::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        });

        let hc = HashableCommand::new(lc, "test");

        hc.execute();

        assert!((*has_run.borrow()));
    }
}
