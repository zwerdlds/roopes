use self::{
    heap_handler::HeapHandler,
    lambda_handler::{
        LambdaHandler,
        LambdaHandlerDelegate,
    },
};

pub mod heap_handler;
pub mod lambda_handler;

pub trait Handler<M>
{
    fn handle(
        &self,
        message: &M,
    );
}

impl<C, M> From<LambdaHandler<C, M>> for HeapHandler<M>
where
    C: LambdaHandlerDelegate<M> + 'static,
    M: 'static,
{
    fn from(handler: LambdaHandler<C, M>) -> HeapHandler<M>
    {
        HeapHandler::new(Box::new(handler))
    }
}
