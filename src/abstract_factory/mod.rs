pub mod lambda_abstract_factory;

pub trait AbstractFactory<T>
{
    fn create(&self) -> T;
}
