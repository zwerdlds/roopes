use crate::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
};

#[test]
fn validate_execution()
{
    struct DummyObs
    {
        has_notified: Rc<RefCell<bool>>,
    }
    impl Observer for DummyObs
    {
        fn notify(&self)
        {
            *self.has_notified.borrow_mut() = true;
        }
    }

    let has_notified = Rc::new(RefCell::new(false));

    let dummy = DummyObs {
        has_notified: has_notified.clone(),
    };

    let exec_dummy = ExecutableObserver { delegate: dummy };

    assert!(!*(*has_notified).borrow());
    exec_dummy.execute();
    assert!(*(*has_notified).borrow());
}

#[test]
fn validate_notify()
{
    struct DummyObs
    {
        has_notified: Rc<RefCell<bool>>,
    }
    impl Observer for DummyObs
    {
        fn notify(&self)
        {
            *self.has_notified.borrow_mut() = true;
        }
    }

    let has_notified = Rc::new(RefCell::new(false));

    let dummy = DummyObs {
        has_notified: has_notified.clone(),
    };

    let exec_dummy = ExecutableObserver { delegate: dummy };

    assert!(!*(*has_notified).borrow());
    exec_dummy.notify();
    assert!(*(*has_notified).borrow());
}
