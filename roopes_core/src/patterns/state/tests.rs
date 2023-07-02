use crate::prelude::*;
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
