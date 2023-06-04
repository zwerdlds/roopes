use crate::{
    executable::Executable,
    state::State,
};

pub struct ExecutableState<S>
where
    S: State,
{
    state: S,
}

impl<S> Executable for ExecutableState<S>
where
    S: State,
{
    fn execute(&self)
    {
        self.state.execute();
    }
}
