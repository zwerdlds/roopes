pub mod lambda;

pub use lambda::Lambda;

pub trait AbstractFactory<T>
{
    fn create(&self) -> T;
}

pub mod prelude
{
    pub use super::AbstractFactory;
}
