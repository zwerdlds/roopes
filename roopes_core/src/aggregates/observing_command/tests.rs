use crate::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
};

#[test]
fn simple_observable_command()
{
    let has_run = Rc::new(RefCell::new(false));
    let has_run_ext = has_run.clone();

    let command =
        command::Executable::new(executable::Lambda::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        }));

    let observing_command: ObservingCommand<_> = command.into();
    let vs = observer::VecSubject::default();

    vs.attach(observing_command);

    assert!(!(*has_run.borrow()));

    vs.notify();

    assert!((*has_run.borrow()));
}
