use super::Command;
use std::hash::Hash;

#[derive(Debug)]
pub struct Hashable<D, H>
where
    D: Command,
    H: Hash + Eq,
{
    command: D,
    id: H,
}

impl<D, H> Hashable<D, H>
where
    D: Command,
    H: Hash + Eq,
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
    H: Hash + Eq,
{
}

impl<D, H> PartialEq<Self> for Hashable<D, H>
where
    D: Command,
    H: Hash + Eq,
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
    H: Hash + Eq,
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
    H: Hash + Eq,
{
    fn execute(&self)
    {
        self.command.execute();
    }
}
