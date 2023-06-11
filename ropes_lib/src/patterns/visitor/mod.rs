pub trait Element<V, T>
where
    V: Visitor<Self, T>,
    Self: Sized,
{
    fn accept(
        &self,
        visitor: V,
    );
}

pub trait Visitor<E, T>
where
    E: Element<Self, T>,
    Self: Sized,
{
    fn visit(element: E);
}
