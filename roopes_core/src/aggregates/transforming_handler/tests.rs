use crate::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
};

#[test]
fn trans_handler_handle()
{
    struct DummyTransformer {}
    impl Transformer<bool, bool> for DummyTransformer
    {
        fn transform(
            &self,
            input: &bool,
        ) -> bool
        {
            !*input
        }
    }

    struct DummyHandler
    {
        value: Rc<RefCell<bool>>,
    }
    impl Handler<bool> for DummyHandler
    {
        fn handle(
            &self,
            message: &bool,
        )
        {
            *(*self.value).borrow_mut() = *message;
        }
    }

    let value = Rc::new(RefCell::new(false));
    let value_ext = value.clone();

    let trans_hand = TransformingHandler::new(
        DummyTransformer {},
        DummyHandler { value: value_ext },
    );

    assert!(!(*value.borrow()));

    trans_hand.handle(&false);

    assert!((*value.borrow()));

    trans_hand.handle(&true);

    assert!(!(*value.borrow()));
}
