pub mod lambda_emitter;

pub trait Emitter<O>
{
    fn emit(&self) -> O;
}
