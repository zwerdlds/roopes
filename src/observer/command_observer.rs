use super::Observer;
use crate::command::Command;

impl<T> Observer for T
where
    T: Command,
{
    fn notify(&self)
    {
        self.execute();
    }
}
