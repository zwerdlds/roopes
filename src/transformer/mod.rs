use self::{
    heap_transformer::HeapTransformer,
    lambda_transformer::{
        LambdaTransformer,
        LambdaTransformerDelegate,
    },
};

pub mod heap_transformer;
pub mod lambda_transformer;

pub trait Transformer<I, O>
{
    fn transform(
        &self,
        input: &I,
    ) -> O;
}

impl<C, I, O> From<LambdaTransformer<C, I, O>> for HeapTransformer<I, O>
where
    C: LambdaTransformerDelegate<I, O> + 'static,
    I: 'static,
    O: 'static,
{
    fn from(transformer: LambdaTransformer<C, I, O>) -> HeapTransformer<I, O>
    {
        HeapTransformer::new(Box::new(transformer))
    }
}
