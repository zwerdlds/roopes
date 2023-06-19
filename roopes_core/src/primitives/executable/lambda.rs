use super::Executable;

pub struct Lambda<D>
where
    D: super::Delegate,
{
    delegate: D,
}

impl<D> Lambda<D>
where
    D: super::Delegate,
{
    pub fn new(delegate: D) -> Self
    {
        Self { delegate }
    }
}

impl<D> Executable for Lambda<D>
where
    D: super::Delegate,
{
    fn execute(&self)
    {
        (self.delegate)();
    }
}
