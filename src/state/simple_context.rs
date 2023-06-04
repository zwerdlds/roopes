use super::{
    Context,
    State,
};

pub struct SimpleContext<S>
where
    S: State,
{
    state: S,
}

impl<S> SimpleContext<S>
where
    S: State,
{
    pub fn new(starting_state: S) -> SimpleContext<S>
    {
        Self {
            state: starting_state,
        }
    }
}

impl<S> Context<S> for SimpleContext<S>
where
    S: State,
{
    fn handle(&mut self)
    {
        self.state = self.state.execute();
    }
}
