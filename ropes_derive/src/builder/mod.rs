#[allow(clippy::module_name_repetitions)]
pub mod builder_token_stream;
pub mod derive;

pub trait Builder<I, O>
{
    fn build(&self) -> O;
    fn get_params(&self) -> &I;
    fn set_params(
        &mut self,
        params: I,
    );
}
