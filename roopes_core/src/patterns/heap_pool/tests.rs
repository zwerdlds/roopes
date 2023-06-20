use super::refcell_box::RefCellBox;
use crate::prelude::{
    command::heap,
    *,
};
use std::{
    borrow::BorrowMut,
    cell::{
        Ref,
        RefCell,
    },
    rc::Rc,
};

struct TestObj
{
    new: bool,
    clean: bool,
}

fn clean_test_obj(test_obj_rc: &RefCell<TestObj>)
{
    let mut test_obj = test_obj_rc.borrow_mut();

    test_obj.clean = true;
}

fn create_test_obj() -> TestObj
{
    let new = true;
    let clean = true;

    TestObj { new, clean }
}

fn use_test_obj(test_obj: &RefCell<TestObj>)
{
    let mut test_obj = test_obj.borrow_mut();

    test_obj.new = false;
    test_obj.clean = false;
}

#[test]
fn heap_pool_direct_expansion()
{
    let mut heap_pool: RefCellBox<
        TestObj,
        emitter::Lambda<_, _>,
        handler::Lambda<_, _>,
    > = RefCellBox::new(
        Vec::new(),
        create_test_obj.into(),
        5,
        clean_test_obj.into(),
    );

    assert_eq!(0, heap_pool.unused_pool_size());

    heap_pool.expand();

    assert_eq!(5, heap_pool.unused_pool_size());
}

#[test]
fn heap_pool_indirect_expansion()
{
    let mut heap_pool: RefCellBox<
        TestObj,
        emitter::Lambda<_, _>,
        handler::Lambda<_, _>,
    > = RefCellBox::new(
        Vec::new(),
        create_test_obj.into(),
        5,
        clean_test_obj.into(),
    );

    assert_eq!(0, heap_pool.unused_pool_size());

    heap_pool.check_out();

    assert_eq!(4, heap_pool.unused_pool_size());
}

#[test]
fn heap_pool_washing()
{
    let mut heap_pool: RefCellBox<
        TestObj,
        emitter::Lambda<_, _>,
        handler::Lambda<_, _>,
    > = RefCellBox::new(
        Vec::new(),
        create_test_obj.into(),
        5,
        clean_test_obj.into(),
    );

    let test_contents = heap_pool.check_out();

    assert_eq!(true, test_contents.borrow().clean);
    assert_eq!(true, test_contents.borrow().new);

    use_test_obj(&test_contents);

    assert_eq!(false, test_contents.borrow().clean);
    assert_eq!(false, test_contents.borrow().new);

    heap_pool.check_in(test_contents);

    let test_contents = heap_pool.check_out();

    assert_eq!(true, test_contents.borrow().clean);
    assert_eq!(false, test_contents.borrow().new);
}
