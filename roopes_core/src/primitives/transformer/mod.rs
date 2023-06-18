pub mod heap;
pub mod lambda;

pub use heap::Heap;
pub use lambda::Lambda;

pub trait Delegate<I, O> = Fn(&I) -> O;

pub trait Transformer<I, O>
{
    fn transform(
        &self,
        input: &I,
    ) -> O;
}

impl<C, I, O> From<Lambda<C, I, O>> for Heap<I, O>
where
    C: Delegate<I, O> + 'static,
    I: 'static,
    O: 'static,
{
    fn from(transformer: Lambda<C, I, O>) -> Heap<I, O>
    {
        Heap::new(Box::new(transformer))
    }
}

pub mod prelude
{
    pub use super::Transformer;
}

#[cfg(test)]
mod tests;
