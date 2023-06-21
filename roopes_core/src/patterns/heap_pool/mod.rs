//! Contains types which allow client code to
//! generate, use, and re-use heap-allocated
//! objects efficiently.
pub mod refcell_box;

#[cfg(test)]
mod tests;

/// Holds a list of allocated objects in a
/// scalable pool.  Previously allocated
/// objects can be checked back in after use, to
/// prevent immediate deallocation.
pub trait HeapPool<C>
{
    /// Get a value from the [`HeapPool`].
    fn check_out(&mut self) -> C;

    /// Give a value back to the [`HeapPool`].
    fn check_in(
        &mut self,
        container: C,
    );
}

/// Exposes the [`HeapPool`] types at the library
/// level.
pub mod prelude
{
    pub use super::HeapPool;
}
