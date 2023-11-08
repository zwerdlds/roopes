use crate::prelude::{
    handler::Lambda,
    *,
};
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
fn double_observable_hash_subscribing_handler()
{
    let handler_a = || {
        SubscribingHandler::new(handler::hash::Hashable::new(
            handler::Heap::new(Box::new(Lambda::new(move |_: &i32| {}))),
            "A",
        ))
    };

    let handler_b = || {
        SubscribingHandler::new(handler::hash::Hashable::new(
            handler::Heap::new(Box::new(Lambda::new(move |_: &i32| {}))),
            "B",
        ))
    };

    let hash = |sub_handler: SubscribingHandler<_, _>| {
        let mut hasher = DefaultHasher::new();
        sub_handler.hash(&mut hasher);
        hasher.finish()
    };

    let sub_handler_a: SubscribingHandler<_, _> = handler_a();
    let sub_handler_b_1: SubscribingHandler<_, _> = handler_b();
    let sub_handler_b_2: SubscribingHandler<_, _> = handler_b();

    let a_hash = hash(sub_handler_a);
    let b_1_hash = hash(sub_handler_b_1);
    let b_2_hash = hash(sub_handler_b_2);

    assert_ne!(a_hash, b_1_hash);
    assert_ne!(a_hash, b_2_hash);
    assert_eq!(b_1_hash, b_2_hash);
}

#[test]
fn double_observable_subscribing_handler()
{
    let has_run = Rc::new(RefCell::new(false));
    let has_run_ext = has_run.clone();

    let handler = handler::Lambda::new(|message| {
        (*has_run_ext.borrow_mut()) = *message;
    });

    let handler: SubscribingHandler<_, _> = handler.into();

    assert!(!(*has_run.borrow()));

    handler.handle(&true);

    assert!((*has_run.borrow()));
}
