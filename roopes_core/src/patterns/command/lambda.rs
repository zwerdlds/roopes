use crate::prelude::*;

pub struct Lambda<C>
where
    C: executable::Delegate,
{
    delegate: C,
}

impl<C> Lambda<C>
where
    C: executable::Delegate,
{
    pub fn new(delegate: C) -> Lambda<C>
    {
        Lambda { delegate }
    }
}

impl<C> Command for Lambda<C>
where
    C: executable::Delegate,
{
    fn execute(&self)
    {
        (self.delegate)();
    }
}

impl<C> From<C> for Lambda<C>
where
    C: executable::Delegate,
{
    fn from(delegate: C) -> Self
    {
        Lambda::new(delegate)
    }
}
