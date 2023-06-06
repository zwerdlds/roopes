pub mod lambda;

pub use lambda::Lambda;

pub trait Builder<I, O>
{
    fn build(&self) -> O;
    fn params(&self) -> &I;
    fn set_params(
        &mut self,
        params: I,
    );
}
