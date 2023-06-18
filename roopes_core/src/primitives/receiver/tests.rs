use roopes::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
};

#[test]
fn simple_lambda_refcell_mutation()
{
    let has_run = Rc::new(RefCell::new(false));
    let has_run_ext = has_run.clone();

    let fn_receiver = move |v: bool| {
        (*has_run_ext.borrow_mut()) = v;
    };

    let lambda_receiver: receiver::Lambda<_, bool> = fn_receiver.into();

    lambda_receiver.receive(true);

    assert!((*has_run.borrow()));
}

#[test]
fn simple_heap_refcell_mutation()
{
    let has_run = Rc::new(RefCell::new(false));
    let has_run_ext = has_run.clone();

    let lh = receiver::Lambda::new(move |v| {
        (*has_run_ext.borrow_mut()) = v;
    });
    let hh = receiver::Heap::new(Box::new(lh));

    assert!(!(*has_run.borrow()));
    hh.receive(true);
    assert!((*has_run.borrow()));
}
