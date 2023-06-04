use super::Transformer;
use std::marker::PhantomData;

pub trait LambdaTransformerDelegate<I, O> = Fn(&I) -> O;

#[derive(Clone)]
pub struct LambdaTransformer<C, I, O>
where
    C: LambdaTransformerDelegate<I, O>,
{
    delegate: C,
    _t: PhantomData<(I, O)>,
}

impl<C, I, O> LambdaTransformer<C, I, O>
where
    C: LambdaTransformerDelegate<I, O>,
{
    pub fn new(delegate: C) -> LambdaTransformer<C, I, O>
    {
        LambdaTransformer {
            delegate,
            _t: PhantomData,
        }
    }
}

impl<C, I, O> Transformer<I, O> for LambdaTransformer<C, I, O>
where
    C: LambdaTransformerDelegate<I, O>,
{
    fn transform(
        &self,
        input: &I,
    ) -> O
    {
        (self.delegate)(input)
    }
}

impl<C, I, O> From<C> for LambdaTransformer<C, I, O>
where
    C: LambdaTransformerDelegate<I, O>,
{
    fn from(delegate: C) -> Self
    {
        LambdaTransformer::new(delegate)
    }
}

#[cfg(test)]
mod tests
{
    use crate::transformer::{
        lambda_transformer::LambdaTransformer,
        Transformer,
    };
    use std::{
        cell::RefCell,
        rc::Rc,
    };

    #[test]
    fn simple_lambda_refcell_mutation()
    {
        let has_run = Rc::new(RefCell::new(false));
        let has_run_ext = has_run.clone();

        let lc = LambdaTransformer::new(move |v| {
            (*has_run_ext.borrow_mut()) = *v;
        });
        lc.transform(&true);

        assert!((*has_run.borrow()));
    }
}
