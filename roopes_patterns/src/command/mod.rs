pub mod hashable;
pub mod heap;
pub mod lambda;

pub use hashable::Hashable;
pub use heap::Heap;
pub use lambda::Lambda;

pub trait Command
{
    fn execute(&self);
}

pub mod prelude
{
    pub use super::Command;
}
