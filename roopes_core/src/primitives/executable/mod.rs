pub mod heap;
pub mod lambda;

pub use heap::Heap;
pub use lambda::Lambda;

#[cfg(test)]
mod tests;

pub trait Delegate = Fn();

pub trait Executable
{
    fn execute(&self);
}

pub mod prelude
{
    pub use super::Executable;
}
