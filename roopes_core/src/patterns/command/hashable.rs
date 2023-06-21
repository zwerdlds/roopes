//! Contains an implementation of [`Command`]
//! which requires the instantiation to supply a
//! method of determining both [`Hash`] and
//! [`Eq`].  Can be used
//! with [`observer::HashSubject`].

use crate::prelude::*;
use std::hash::Hash;

/// Delegates [`Command::execute`] calls to a
/// delegate command while delegating [`Eq`] and
/// [`Hash`] to an `id` object.
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
    /// Creates a [`Hashable`] with the specified
    /// delegate [`Command`] and identifier.
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
