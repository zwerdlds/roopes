use crate::prelude::*;
use std::{
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};
#[test]
fn simple_observable_command_notify()
{
    let has_run = Rc::new(RefCell::new(false));
    let has_run_ext = has_run.clone();

    let command =
        command::Executable::new(executable::Lambda::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        }));

    let observing_command: ObservingCommand<_> = command.into();

    assert!(!(*has_run.borrow()));

    observing_command.notify();

    assert!((*has_run.borrow()));
}

#[test]
fn simple_observable_command_execute()
{
    let has_run = Rc::new(RefCell::new(false));
    let has_run_ext = has_run.clone();

    let command =
        command::Executable::new(executable::Lambda::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        }));

    let observing_command: ObservingCommand<_> = command.into();

    assert!(!(*has_run.borrow()));

    observing_command.execute();

    assert!((*has_run.borrow()));
}

#[test]
fn notify_observable_command()
{
    let has_run = Rc::new(RefCell::new(false));
    let has_run_ext = has_run.clone();

    let command =
        command::Executable::new(executable::Lambda::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        }));

    let observing_command: ObservingCommand<_> = command.into();
    let mut vs = observer::VecSubject::default();

    vs.attach(observing_command);

    assert!(!(*has_run.borrow()));

    vs.notify();

    assert!((*has_run.borrow()));
}

#[test]
fn observable_detach_hash_command()
{
    let has_run_a = Rc::new(RefCell::new(false));
    let has_run_b = Rc::new(RefCell::new(false));
    let has_run_a_ext = has_run_a.clone();

    let command_a = command::Hashable::new(
        command::Heap::new(Box::new(command::Executable::new_lambda(
            move || {
                (*has_run_a_ext.borrow_mut()) = true;
            },
        ))),
        "A",
    );

    let command_b = || {
        let has_run_b_ext = has_run_b.clone();

        command::Hashable::new(
            command::Heap::new(Box::new(command::Executable::new_lambda(
                move || {
                    (*has_run_b_ext.borrow_mut()) = true;
                },
            ))),
            "B",
        )
    };

    let observing_command_a: ObservingCommand<_> = command_a.into();
    let observing_command_b_1: ObservingCommand<_> = command_b().into();
    let observing_command_b_2: ObservingCommand<_> = command_b().into();
    let mut vs = observer::HashSubject::default();

    vs.attach(observing_command_a);
    vs.attach(observing_command_b_1);

    assert!(!(*has_run_a.borrow()));
    assert!(!(*has_run_b.borrow()));

    vs.detach(&observing_command_b_2).unwrap();

    vs.notify();

    assert!((*has_run_a.borrow()));
    assert!(!(*has_run_b.borrow()));
}

#[test]
fn observable_detach_vec_command()
{
    let run_ct_a = Rc::new(RefCell::new(0));
    let run_ct_b = Rc::new(RefCell::new(0));
    let run_ct_a_ext = run_ct_a.clone();

    let command_a = command::Hashable::new(
        command::Heap::new(Box::new(command::Executable::new_lambda(
            move || {
                (*run_ct_a_ext.borrow_mut()) += 1;
            },
        ))),
        "A",
    );

    let command_b = || {
        let run_ct_b_ext = run_ct_b.clone();

        command::Hashable::new(
            command::Heap::new(Box::new(command::Executable::new_lambda(
                move || {
                    (*run_ct_b_ext.borrow_mut()) += 1;
                },
            ))),
            "B",
        )
    };

    let observing_command_a: ObservingCommand<_> = command_a.into();
    let observing_command_b_1: ObservingCommand<_> = command_b().into();
    let observing_command_b_2: ObservingCommand<_> = command_b().into();
    let mut vs = observer::VecSubject::default();

    vs.attach(observing_command_a);
    vs.attach(observing_command_b_1);

    assert_eq!(0, (*run_ct_a.borrow()));
    assert_eq!(0, (*run_ct_b.borrow()));

    vs.detach(&observing_command_b_2).unwrap();

    vs.notify();

    assert_eq!(1, *run_ct_a.borrow());
    assert_eq!(0, *run_ct_b.borrow());
}

#[test]
fn double_observable_hash_command()
{
    let command_a = command::Hashable::new(
        command::Heap::new(Box::new(command::Executable::new_lambda(
            move || {},
        ))),
        "A",
    );

    let command_b = || {
        command::Hashable::new(
            command::Heap::new(Box::new(command::Executable::new_lambda(
                move || {},
            ))),
            "B",
        )
    };

    let hash = |oc: ObservingCommand<_>| {
        let mut hasher = DefaultHasher::new();
        oc.hash(&mut hasher);
        hasher.finish()
    };

    let observing_command_a: ObservingCommand<_> = command_a.into();
    let observing_command_b_1: ObservingCommand<_> = command_b().into();
    let observing_command_b_2: ObservingCommand<_> = command_b().into();

    let a_hash = hash(observing_command_a);
    let b_1_hash = hash(observing_command_b_1);
    let b_2_hash = hash(observing_command_b_2);

    assert_ne!(a_hash, b_1_hash);
    assert_ne!(a_hash, b_2_hash);
    assert_eq!(b_1_hash, b_2_hash);
}
