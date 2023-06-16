pub mod heap;
pub mod lambda;

pub use heap::Heap;
pub use lambda::Lambda;

/// Defines a primitive interface which consumes messages.
pub trait Handler<M>
{
    fn handle(
        &self,
        message: &M,
    );
}

impl<C, M> From<Lambda<C, M>> for Heap<M>
where
    C: lambda::Delegate<M> + 'static,
    M: 'static,
{
    fn from(handler: Lambda<C, M>) -> Heap<M>
    {
        Heap::new(Box::new(handler))
    }
}

pub mod prelude
{
    pub use super::Handler;
}
