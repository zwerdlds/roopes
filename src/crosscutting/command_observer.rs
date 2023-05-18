use crate::{
    command::Command,
    observer::Observer,
};

impl<T> Observer for T
where
    T: Command,
{
    fn notify(&self)
    {
        self.execute();
    }
}
