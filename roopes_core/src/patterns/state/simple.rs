//! Implements a basic wrapper [`Context`] which contains a generic [`State`]
//! object.

use super::{
    Context,
    State,
};

/// A basic implementation of [`Context`].  Stores the current generic
/// [`State`], and possibly transitions to a new state when [`Context::handle`]
/// is called.
#[allow(clippy::module_name_repetitions)]
pub struct SimpleContext<S>
where
    S: State,
{
    state: S,
}

impl<S> SimpleContext<S>
where
    S: State,
{
    /// Creates a new [`SimpleContext`] with a given starting [`State`].
    pub fn new(starting_state: S) -> SimpleContext<S>
    {
        Self {
            state: starting_state,
        }
    }

    /// Gets the current [`State`].
    pub fn get_state(&self) -> &S
    {
        &self.state
    }
}

impl<S> Context<S> for SimpleContext<S>
where
    S: State,
{
    fn handle(&mut self)
    {
        self.state = self.state.execute();
    }
}
