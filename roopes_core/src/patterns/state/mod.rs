pub mod heap;
pub mod simple;

pub trait Context<S>
where
    S: State,
{
    fn handle(&mut self);
}

pub trait State
{
    #[must_use]
    fn execute(&self) -> Self;
}

pub mod prelude
{
    pub use super::{
        Context,
        State,
    };
}

#[cfg(test)]
mod tests;
