//! Contains a wrapper struct to indirect calls to
//! [`executable::Executable`] via a [`Command`]

use crate::prelude::*;
use delegate::delegate;

/// Delegates execution to a specified
/// [`executable::Executable`] object.
pub struct Executable<C>
where
    C: executable::Executable,
{
    delegate: C,
}

impl<C> Executable<C>
where
    C: executable::Executable,
{
    /// Creates a new [`Executable`] object from a
    /// given [`executable::Executable`].
    pub fn new(delegate: C) -> Executable<C>
    {
        Executable { delegate }
    }
}

impl<C> Executable<executable::Lambda<C>>
where
    C: executable::lambda::Delegate,
{
    /// Creates a new [`Executable`] from a
    /// [`executable::lambda::Delegate`].
    pub fn new_lambda(delegate: C) -> Executable<executable::Lambda<C>>
    {
        let delegate = executable::Lambda::new(delegate);

        Executable { delegate }
    }
}

#[allow(clippy::inline_always)]
impl<C> Command for Executable<C>
where
    C: executable::Executable,
{
    delegate! {
        to self.delegate{
           fn execute(&self);
        }
    }
}

impl<C> From<C> for Executable<C>
where
    C: executable::Executable,
{
    fn from(delegate: C) -> Self
    {
        Executable::new(delegate)
    }
}
