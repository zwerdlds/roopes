pub mod lambda_factory;

pub trait Factory<T>
{
    fn create(&self) -> T;
}
