//! Contains an implementation of [`HeapPool`]
//! which allow client code to generate, use, and
//! re-use heap-allocated objects efficiently.

use super::HeapPool;
use crate::prelude::*;
use std::{
    borrow::Borrow,
    cell::RefCell,
};

/// Holds a list of allocated objects in a
/// scalable pool.  Previously allocated
/// objects can be checked back in after use, to
/// prevent immediate deallocation.
pub struct RefCellBox<T, F, W>
where
    F: Emitter<T>,
    W: Handler<RefCell<T>>,
{
    unused_pool: Vec<Box<RefCell<T>>>,
    new_gen: F,
    grow_size: u8,
    washer: W,
}

impl<T, F, W> RefCellBox<T, F, W>
where
    F: Emitter<T>,
    W: Handler<RefCell<T>>,
{
    /// Creates a new [`RefCellBox`].
    pub fn new(
        unused_pool: Vec<Box<RefCell<T>>>,
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

    /// Gets the current number of un-checked-out
    /// items in the pool.  The
    /// current "reserve" size.
    pub fn unused_pool_size(&self) -> usize
    {
        self.unused_pool.len()
    }

    /// Grows the [`RefCellBox`] pool by the
    /// previously specified quantity.
    pub fn expand(&mut self)
    {
        (0..self.grow_size).for_each(|_| {
            let pool_obj = Box::new(RefCell::new(self.new_gen.emit()));

            self.unused_pool.push(pool_obj);
        });
    }
}

impl<T, F, W> HeapPool<Box<RefCell<T>>> for RefCellBox<T, F, W>
where
    F: Emitter<T>,
    W: Handler<RefCell<T>>,
{
    fn check_out(&mut self) -> Box<RefCell<T>>
    {
        if self.unused_pool.is_empty() {
            self.expand();
        }

        self.unused_pool.pop().expect("Unused pool is empty.")
    }

    fn check_in(
        &mut self,
        container: Box<RefCell<T>>,
    )
    {
        self.washer.handle(container.borrow());
        self.unused_pool.push(container);
    }
}
