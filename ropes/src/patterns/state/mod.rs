pub mod simple_context;

pub trait Context<S>
where
    S: State,
{
    fn handle(&mut self);
}

pub trait State
{
    #[must_use]
    fn execute(&self) -> Self;
}
