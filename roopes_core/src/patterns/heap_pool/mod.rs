pub mod refcell_box;

#[cfg(test)]
mod tests;

pub trait HeapPool<C>
{
    fn check_out(&self) -> C;
    fn check_in(
        &self,
        container: C,
    );
}

/// Exposes the [`HeapPool`] types at the library level.
pub mod prelude
{
    pub use super::HeapPool;
}
