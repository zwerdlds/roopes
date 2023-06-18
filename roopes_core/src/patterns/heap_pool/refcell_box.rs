use super::HeapPool;
use crate::prelude::*;
use std::{
    borrow::Borrow,
    cell::RefCell,
};

pub struct RefCellBox<T, F, W>
where
    F: Emitter<T>,
    W: Handler<RefCell<T>>,
{
    unused_pool: RefCell<Vec<Box<RefCell<T>>>>,
    new_gen: F,
    grow_size: u8,
    washer: W,
}

impl<T, F, W> RefCellBox<T, F, W>
where
    F: Emitter<T>,
    W: Handler<RefCell<T>>,
{
    pub fn new(
        unused_pool: RefCell<Vec<Box<RefCell<T>>>>,
        new_gen: F,
        grow_size: u8,
        washer: W,
    ) -> RefCellBox<T, F, W>
    {
        RefCellBox {
            unused_pool,
            new_gen,
            grow_size,
            washer,
        }
    }

    pub fn unused_pool_size(&self) -> usize
    {
        self.unused_pool.borrow().len()
    }

    pub fn expand(&self)
    {
        (0..self.grow_size).for_each(|_| {
            let pool_obj = Box::new(RefCell::new(self.new_gen.emit()));

            self.unused_pool.borrow_mut().push(pool_obj);
        });
    }
}

impl<T, F, W> HeapPool<Box<RefCell<T>>> for RefCellBox<T, F, W>
where
    F: Emitter<T>,
    W: Handler<RefCell<T>>,
{
    fn check_out(&self) -> Box<RefCell<T>>
    {
        if self.unused_pool.borrow().is_empty() {
            self.expand();
        }

        self.unused_pool
            .borrow_mut()
            .pop()
            .expect("Unused pool is empty.")
    }

    fn check_in(
        &self,
        container: Box<RefCell<T>>,
    )
    {
        self.washer.handle(container.borrow());
        self.unused_pool.borrow_mut().push(container);
    }
}
