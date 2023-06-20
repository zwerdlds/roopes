pub mod heap;
pub mod simple;

pub trait Context<S>
where
    S: State,
{
    fn handle(&mut self);
}

/// Provides transitions between states.
pub trait State
{
    /// Allow the state to engage in whatever processing it would normally do.
    /// The returned [`State`] is used to determine the new state the
    /// [`Context`] should assume.
    #[must_use]
    fn execute(&self) -> Self;
}

/// Exposes the [`Context`] and [`State`] types at the library level.
pub mod prelude
{
    pub use super::{
        Context,
        State,
    };
}

#[cfg(test)]
mod tests;
