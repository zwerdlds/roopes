//! Combines a transformer and a handler to create a new handler which conforms
//! to the transformer's input, but forwards its output to the handler.
use crate::prelude::*;
use std::marker::PhantomData;

/// Combines a transformer and a handler to create a new handler which conforms
/// to the transformer's input, but forwards its output to the handler.
pub struct TransformingHandler<T, H, TI, HI>
where
    T: Transformer<TI, HI>,
    H: Handler<HI>,
{
    transformer: T,
    handler: H,
    _phantom_data: PhantomData<(TI, HI)>,
}

impl<T, H, TI, HI> TransformingHandler<T, H, TI, HI>
where
    T: Transformer<TI, HI>,
    H: Handler<HI>,
{
    /// Creates a new [`TransformingHandler`] which transforms its input and
    /// forwards the result to the given handler.
    pub fn new(
        transformer: T,
        handler: H,
    ) -> Self
    {
        TransformingHandler {
            transformer,
            handler,
            _phantom_data: PhantomData,
        }
    }
}

impl<T, H, TI, HI> From<(T, H)> for TransformingHandler<T, H, TI, HI>
where
    T: Transformer<TI, HI>,
    H: Handler<HI>,
{
    fn from(value: (T, H)) -> Self
    {
        TransformingHandler::new(value.0, value.1)
    }
}

impl<T, H, TI, HI> Handler<TI> for TransformingHandler<T, H, TI, HI>
where
    T: Transformer<TI, HI>,
    H: Handler<HI>,
{
    fn handle(
        &self,
        message: &TI,
    )
    {
        let result = self.transformer.transform(message);
        self.handler.handle(&result);
    }
}

/// Default export types
pub mod prelude
{
    pub use super::TransformingHandler;
}
