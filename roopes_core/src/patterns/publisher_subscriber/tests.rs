use crate::prelude::{
    publisher_subscriber::{
        heap,
        VecPublisher,
    },
    *,
};
use std::{
    cell::RefCell,
    rc::Rc,
};

#[test]
fn vec_publisher_publish()
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

    assert!(!(*has_run.borrow()));

    publisher.publish(&true);

    assert!((*has_run.borrow()));
}

#[test]
fn vec_publisher_detach()
{
    let run_ct_a = Rc::new(RefCell::new(0));
    let run_ct_b = Rc::new(RefCell::new(0));

    let handler_a = || {
        let run_ct_a_ext = run_ct_a.clone();

        handler::Hashable::new(
            handler::Heap::new(Box::new(handler::Lambda::new(
                move |message| {
                    (*run_ct_a_ext.borrow_mut()) += *message;
                },
            ))),
            "A",
        )
    };

    let handler_b = || {
        let run_ct_b_ext = run_ct_b.clone();

        handler::Hashable::new(
            handler::Heap::new(Box::new(handler::Lambda::new(
                move |message| {
                    (*run_ct_b_ext.borrow_mut()) += *message;
                },
            ))),
            "B",
        )
    };

    let subscribing_handler_a_1: SubscribingHandler<_, _> = handler_a().into();
    let subscribing_handler_a_2: SubscribingHandler<_, _> = handler_a().into();
    let subscribing_handler_b_1: SubscribingHandler<_, _> = handler_b().into();
    let subscribing_handler_b_2: SubscribingHandler<_, _> = handler_b().into();
    let mut vs = publisher_subscriber::VecPublisher::default();

    vs.attach(subscribing_handler_a_1);
    vs.attach(subscribing_handler_b_1);

    assert_eq!(0, (*run_ct_a.borrow()));
    assert_eq!(0, (*run_ct_b.borrow()));

    vs.detach(&subscribing_handler_b_2).unwrap();

    vs.publish(&1);

    assert_eq!(1, *run_ct_a.borrow());
    assert_eq!(0, *run_ct_b.borrow());

    vs.publish(&1);
    assert_eq!(2, *run_ct_a.borrow());

    vs.detach(&subscribing_handler_a_2).unwrap();

    vs.publish(&1);
    assert_eq!(2, *run_ct_a.borrow());
}

#[test]
fn format()
{
    #[derive(Debug)]
    struct DummySub {}
    impl Subscriber<()> for DummySub
    {
        fn receive(
            &self,
            _message: &(),
        )
        {
            todo!()
        }
    }

    let vp = VecPublisher::new(vec![DummySub {}]);

    assert_eq!(
        format!("{:?}", vp),
        "VecPublisher { listeners: [DummySub] }".to_string()
    );
}

#[test]
fn heap_handle()
{
    let has_run = Rc::new(RefCell::new(false));
    let has_run_ext = has_run.clone();

    let handler =
        handler::Heap::new(Box::new(handler::Lambda::new(move |message| {
            (*has_run_ext.borrow_mut()) = *message;
        })));

    let handler: SubscribingHandler<_, _> = handler.into();

    assert!(!(*has_run.borrow()));

    handler.handle(&true);

    assert!((*has_run.borrow()));
}

#[test]
fn heap_subscriber_fmt()
{
    struct DummySub {}
    impl Subscriber<()> for DummySub
    {
        fn receive(
            &self,
            _message: &(),
        )
        {
            todo!()
        }
    }
    let sub = heap::Subscriber::new(Box::new(DummySub {}));

    assert_eq!(format!("{:?}", sub), "Subscriber");
}
