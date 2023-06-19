use super::Handler;
use std::{
    hash::Hash,
    marker::PhantomData,
};

#[derive(Clone)]
pub struct Hashable<D, M, H>
where
    D: Handler<M>,
    H: Hash + Eq,
{
    delegate: D,
    id: H,
    _t: PhantomData<M>,
}

impl<D, M, H> Hashable<D, M, H>
where
    D: Handler<M>,
    H: Hash + Eq,
{
    pub fn new(
        delegate: D,
        id: H,
    ) -> Hashable<D, M, H>
    {
        Hashable {
            delegate,
            id,
            _t: PhantomData,
        }
    }
}

impl<D, M, H> Handler<M> for Hashable<D, M, H>
where
    D: Handler<M>,
    H: Hash + Eq,
{
    fn handle(
        &self,
        message: &M,
    )
    {
        self.delegate.handle(message);
    }
}

impl<D, M, H> PartialEq for Hashable<D, M, H>
where
    D: Handler<M>,
    H: Hash + Eq,
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.id.eq(&other.id)
    }
}

impl<D, M, H> Eq for Hashable<D, M, H>
where
    D: Handler<M>,
    H: Hash + Eq,
{
}

impl<D, M, H> Hash for Hashable<D, M, H>
where
    D: Handler<M>,
    H: Hash + Eq,
{
    fn hash<S: std::hash::Hasher>(
        &self,
        state: &mut S,
    )
    {
        self.id.hash(state);
    }
}
