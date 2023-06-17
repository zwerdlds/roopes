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
mod tests
{
    use roopes::prelude::*;
    use std::{
        cell::RefCell,
        rc::Rc,
    };

    #[derive(Eq, PartialEq)]
    enum TS
    {
        A,
        B,
    }

    impl State for TS
    {
        fn execute(&self) -> Self
        {
            use TS::*;

            match self {
                | A => B,
                | B => A,
            }
        }
    }

    #[test]
    fn simple_state()
    {
        let mut ctx = state::simple::SimpleContext::new(TS::A);

        assert!(ctx.get_state() == &TS::A);

        ctx.handle();
        assert!(ctx.get_state() == &TS::B);

        ctx.handle();
        assert!(ctx.get_state() == &TS::A);
    }
}
