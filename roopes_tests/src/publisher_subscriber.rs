#[cfg(test)]
mod tests
{
    use roopes::prelude::*;
    use std::{
        cell::RefCell,
        rc::Rc,
    };

    #[test]
    fn simple_vec_publisher()
    {
        use publisher_subscriber::{
            heap::Subscriber as HeapSubscriber,
            VecPublisher,
        };

        let has_run = Rc::new(RefCell::new(false));

        let mut publisher: VecPublisher<bool, HeapSubscriber<_>> =
            VecPublisher::default();

        let has_run_ext = has_run.clone();

        let heap_handler: handler::Heap<_> = handler::Lambda::new(move |v| {
            (*has_run_ext.borrow_mut()) = *v;
        })
        .into();

        let sub_handler: SubscribingHandler<_, _> = heap_handler.into();

        let subscriber = HeapSubscriber::new(Box::new(sub_handler));

        publisher.attach(subscriber);

        publisher.publish(&true);

        assert!((*has_run.borrow()));
    }
}
