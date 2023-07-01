use crate::prelude::{
    handler::Lambda,
    publisher_subscriber::VecPublisher,
    *,
};
use std::{
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    hash::Hash,
    rc::Rc,
};

#[test]
fn vec_publisher_delegate_publish()
{
    let has_run = Rc::new(RefCell::new(false));

    let mut publisher = publisher_subscriber::VecPublisher::default();

    let has_run_ext = has_run.clone();

    let heap_handler =
        handler::Heap::new(Box::new(handler::Lambda::new(move |v| {
            (*has_run_ext.borrow_mut()) = *v;
        })));

    let sub_handler: SubscribingHandler<_, _> = heap_handler.into();

    let subscriber =
        publisher_subscriber::heap::Subscriber::new(Box::new(sub_handler));

    publisher.attach(subscriber);

    let handler: HandlingPublisher<_, _> = publisher.into();

    assert!(!(*has_run.borrow()));

    handler.handle(&true);

    assert!((*has_run.borrow()));
}
