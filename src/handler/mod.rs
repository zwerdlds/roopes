pub mod lambda_handler;

pub trait Handler<T>
{
    fn handle(
        &self,
        message: T,
    );
}
