use crate::prelude::*;

pub struct Executable<C>
where
    C: executable::Executable,
{
    delegate: C,
}

impl<C> Executable<C>
where
    C: executable::Executable,
{
    pub fn new(delegate: C) -> Executable<C>
    {
        Executable { delegate }
    }
}

impl<C> Executable<executable::Lambda<C>>
where
    C: executable::Delegate,
{
    pub fn new_lambda(delegate: C) -> Executable<executable::Lambda<C>>
    {
        let delegate = executable::Lambda::new(delegate);

        Executable { delegate }
    }
}

impl<C> Command for Executable<C>
where
    C: executable::Executable,
{
    fn execute(&self)
    {
        self.delegate.execute();
    }
}

impl<C> From<C> for Executable<C>
where
    C: executable::Executable,
{
    fn from(delegate: C) -> Self
    {
        Executable::new(delegate)
    }
}
