use super::Executable;

pub struct Heap
{
    delegate: Box<dyn super::Executable>,
}

impl Heap
{
    #[must_use]
    pub fn new(delegate: Box<dyn super::Executable>) -> Self
    {
        Self { delegate }
    }
}

impl Executable for Heap
{
    fn execute(&self)
    {
        self.delegate.execute();
    }
}
