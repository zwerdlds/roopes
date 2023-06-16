pub mod box_pool;

pub trait HeapPool<C>
{
    fn check_out(&mut self) -> C;
    fn check_in(
        &mut self,
        container: C,
    );
}

pub mod prelude
{
    pub use super::HeapPool;
}
