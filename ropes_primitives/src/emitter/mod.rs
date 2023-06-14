pub mod iterator;
pub mod lambda;

pub use iterator::Iterator;
pub use lambda::Lambda;

pub trait Emitter<O>
{
    fn emit(&self) -> O;
}

pub mod prelude
{
    pub use super::Emitter;
}
