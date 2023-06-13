use super::Command;

pub trait Delegate = Fn();

pub struct Lambda<C>
where
    C: Delegate,
{
    delegate: C,
}

impl<C> Lambda<C>
where
    C: Delegate,
{
    pub fn new(delegate: C) -> Lambda<C>
    {
        Lambda { delegate }
    }
}

impl<C> Command for Lambda<C>
where
    C: Delegate,
{
    fn execute(&self)
    {
        (self.delegate)();
    }
}

impl<C> From<C> for Lambda<C>
where
    C: Delegate,
{
    fn from(delegate: C) -> Self
    {
        Lambda::new(delegate)
    }
}
