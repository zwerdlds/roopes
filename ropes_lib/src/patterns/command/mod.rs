pub mod hashable;
pub mod lambda;

pub use hashable::Hashable;
pub use lambda::Lambda;

pub trait Command
{
    fn execute(&self);
}
