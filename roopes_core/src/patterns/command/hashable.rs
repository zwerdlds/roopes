use super::Command;
use std::hash::Hash;

pub trait Delegate = Command;
pub trait Subject = Hash + Eq;

#[derive(Debug)]
pub struct Hashable<D, H>
where
    D: Command,
    H: Subject,
{
    command: D,
    id: H,
}

impl<D, H> Hashable<D, H>
where
    D: Command,
    H: Subject,
{
    pub fn new(
        command: D,
        id: H,
    ) -> Hashable<D, H>
    {
        Hashable { command, id }
    }
}

impl<D, H> Eq for Hashable<D, H>
where
    D: Command,
    H: Subject,
{
}

impl<D, H> PartialEq<Self> for Hashable<D, H>
where
    D: Command,
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
    D: Command,
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
    D: Command,
    H: Subject,
{
    fn execute(&self)
    {
        self.command.execute();
    }
}
