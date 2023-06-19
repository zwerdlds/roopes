pub mod executable;
pub mod hashable;
pub mod heap;

pub use executable::Executable;
pub use hashable::Hashable;
pub use heap::Heap;

pub trait Command
{
    fn execute(&self);
}

pub mod prelude
{
    pub use super::Command;
}
