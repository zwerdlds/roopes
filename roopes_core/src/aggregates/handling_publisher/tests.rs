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
fn vec_publisher_delegate_handle()
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

    handler.publish(&true);

    assert!((*has_run.borrow()));
}

#[test]
fn handling_publisher_eq()
{
    #[derive(PartialEq)]
    struct DummyPub
    {
        id: usize,
    }
    impl Publisher<()> for DummyPub
    {
        fn publish(
            &self,
            _message: &(),
        )
        {
            todo!()
        }
    }

    let dp_a1 = HandlingPublisher::from(DummyPub { id: 1 });
    let dp_b1 = HandlingPublisher::from(DummyPub { id: 2 });
    let dp_a2 = HandlingPublisher::from(DummyPub { id: 1 });

    assert!(dp_a1 == dp_a2);
    assert!(dp_a2 == dp_a1);
    assert!(dp_a1 != dp_b1);
    assert!(dp_a2 != dp_b1);
    assert!(dp_b1 != dp_a1);
    assert!(dp_b1 != dp_a2);
}

#[test]
fn format()
{
    #[derive(Debug)]
    struct DummyPub {}
    impl Publisher<()> for DummyPub
    {
        fn publish(
            &self,
            _message: &(),
        )
        {
            todo!()
        }
    }

    let hp = HandlingPublisher::from(DummyPub {});

    assert_eq!(
        format!("{:?}", hp),
        "HandlingPublisher { delegate: DummyPub }".to_string()
    );
}
