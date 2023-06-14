/// Gives the ability for additional [`Observer`]s to be added to the list of
/// notified objects.
pub trait Attachable<O>
{
    fn attach(
        &mut self,
        object: O,
    );
}

pub mod prelude
{
    pub use super::Attachable;
}
