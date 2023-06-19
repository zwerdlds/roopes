use crate::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
};

#[test]
fn vec_publisher_publish()
{
    let has_run = Rc::new(RefCell::new(false));

    let publisher = publisher_subscriber::VecPublisher::default();

    let has_run_ext = has_run.clone();

    let heap_handler: handler::Heap<_> = handler::Lambda::new(move |v| {
        (*has_run_ext.borrow_mut()) = *v;
    })
    .into();

    let sub_handler: SubscribingHandler<_, _> = heap_handler.into();

    let subscriber =
        publisher_subscriber::heap::Subscriber::new(Box::new(sub_handler));

    publisher.attach(subscriber);

    assert!(!(*has_run.borrow()));

    publisher.publish(&true);

    assert!((*has_run.borrow()));
}

#[test]
fn vec_publisher_detach()
{
    let has_run = Rc::new(RefCell::new(false));

    let publisher = publisher_subscriber::VecPublisher::default();

    let has_run_ext = has_run.clone();

    let heap_handler: handler::Heap<_> = handler::Lambda::new(move |v| {
        (*has_run_ext.borrow_mut()) = *v;
    })
    .into();

    let sub_handler: SubscribingHandler<_, _> = heap_handler.into();

    let subscriber =
        publisher_subscriber::heap::Subscriber::new(Box::new(sub_handler));

    publisher.attach(subscriber);

    assert!(!(*has_run.borrow()));

    publisher.publish(&true);

    assert!((*has_run.borrow()));
}
