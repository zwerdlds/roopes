pub mod heap;
pub mod lambda;

pub use heap::Heap;
pub use lambda::Lambda;

#[cfg(test)]
mod tests;

pub trait Delegate<M> = Fn(&M);

/// Defines a primitive interface which handles messages via borrowing.
pub trait Handler<M>
{
    fn handle(
        &self,
        message: &M,
    );
}

impl<C, M> From<Lambda<C, M>> for Heap<M>
where
    C: Delegate<M> + 'static,
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
