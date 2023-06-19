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
fn simple_heap_command()
{
    let has_run = Rc::new(RefCell::new(false));
    let has_run_ext = has_run.clone();

    let command =
        executable::Heap::new(Box::new(executable::Lambda::new(move || {
            (*has_run_ext.borrow_mut()) = true;
        })));

    assert!(!(*has_run.borrow()));

    command.execute();

    assert!((*has_run.borrow()));
}
