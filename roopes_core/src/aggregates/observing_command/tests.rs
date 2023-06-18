use crate::prelude::{
    abstract_factory::lambda,
    *,
};
use std::{
    cell::RefCell,
    rc::Rc,
};

#[test]
fn simple_observable_command()
{
    let has_run = Rc::new(RefCell::new(false));
    let has_run_ext = has_run.clone();

    let lambda_command = command::Lambda::new(move || {
        (*has_run_ext.borrow_mut()) = true;
    });

    let observing_command: ObservingCommand<_> = lambda_command.into();
    let mut vs = observer::VecSubject::default();

    vs.attach(observing_command);

    assert!(!(*has_run.borrow()));

    vs.notify();

    assert!((*has_run.borrow()));
}
