use super::HeapPool;
use crate::abstract_factory::AbstractFactory;

pub struct BoxPool<T, F, W>
where
    F: AbstractFactory<T>,
    W: Fn(&mut T),
{
    unused_pool: Vec<Box<T>>,
    new_gen: F,
    grow_size: u8,
    washer: W,
}

impl<T, F, W> BoxPool<T, F, W>
where
    F: AbstractFactory<T>,
    W: Fn(&mut T),
{
    pub fn new(
        unused_pool: Vec<Box<T>>,
        new_gen: F,
        grow_size: u8,
        washer: W,
    ) -> BoxPool<T, F, W>
    {
        BoxPool {
            new_gen,
            unused_pool,
            grow_size,
            washer,
        }
    }
}

impl<T, F, W> HeapPool<Box<T>> for BoxPool<T, F, W>
where
    F: AbstractFactory<T>,
    W: Fn(&mut T),
{
    fn check_out(&mut self) -> Box<T>
    {
        if self.unused_pool.is_empty() {
            for _ in 0..=self.grow_size {
                let pool_obj = self.new_gen.create().into();

                self.unused_pool.push(pool_obj);
            }
        }

        self.unused_pool.pop().expect("Unused pool is empty.")
    }

    fn check_in(
        &mut self,
        container: Box<T>,
    )
    {
        let mut container = container;
        (self.washer)(&mut *container);
        self.unused_pool.push(container)
    }
}
