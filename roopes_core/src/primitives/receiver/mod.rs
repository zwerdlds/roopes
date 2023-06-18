pub mod heap;
pub mod lambda;

pub use heap::Heap;
pub use lambda::Lambda;

#[cfg(test)]
mod tests;

pub trait Delegate<M> = Fn(M);

/// Defines a primitive interface which consumes messages by taking ownership.
pub trait Receiver<M>
{
    fn receive(
        &self,
        message: M,
    );
}

impl<C, M> From<Lambda<C, M>> for Heap<M>
where
    C: Delegate<M> + 'static,
    M: 'static,
{
    fn from(receiver: Lambda<C, M>) -> Heap<M>
    {
        Heap::new(Box::new(receiver))
    }
}

pub mod prelude
{
    pub use super::Receiver;
}
