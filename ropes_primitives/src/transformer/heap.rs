use super::Transformer;
use delegate::delegate;

pub struct Heap<I, O>
{
    delegate: Box<dyn Transformer<I, O>>,
}

impl<I, O> Heap<I, O>
{
    #[must_use]
    pub fn new(delegate: Box<dyn Transformer<I, O>>) -> Heap<I, O>
    {
        Heap { delegate }
    }
}

impl<I, O> Transformer<I, O> for Heap<I, O>
{
    delegate! {
        to self.delegate {
           fn transform(&self, input: &I) -> O;
        }
    }
}
