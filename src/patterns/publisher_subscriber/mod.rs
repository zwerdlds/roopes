pub trait Publisher<M>
{
    fn publish(message: M);
}
pub trait Subscriber<M>
{
    fn receive(message: M);
}
