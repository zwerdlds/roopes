//! This module implements the State pattern.

pub mod simple;

/// This trait holds the active state, and acts as
/// a persistent handle for the state machine,
/// which could otherwise be the states
/// themselves.
pub trait Context<S>
where
    S: State,
{
    /// Delegates the state transition to the
    /// currently active state held by the
    /// context, probably triggering a state
    /// change by the context.
    fn handle(&mut self);
}

/// Provides transitions between states.
pub trait State
{
    /// Allow the state to engage in whatever
    /// processing it would normally do.
    /// The returned [`State`] is used to
    /// determine the new state the
    /// [`Context`] should assume.
    #[must_use]
    fn execute(&self) -> Self;
}

/// Exposes the [`Context`] and [`State`] types at
/// the library level.
pub mod prelude
{
    pub use super::{
        Context,
        State,
    };
}

#[cfg(test)]
mod tests;
