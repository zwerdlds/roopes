pub mod lambda_builder;

pub trait Builder<R, P>
{
    fn build(&self) -> R;
    fn params(&mut self) -> &mut P;
}
