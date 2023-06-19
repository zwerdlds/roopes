/// Objects can be removed from the implementing object.
pub trait Detachable<O, S, E>
{
    ///Removed the object from [Self].
    /// # Errors
    /// E: The detaching somehow encountered a fault.
    fn detach(
        &self,
        object: &O,
    ) -> Result<S, E>;
}
#[allow(clippy::module_name_repetitions)]
pub type BoxedDetachable<O, S, E> = Box<dyn Detachable<O, S, E>>;

pub mod prelude
{
    pub use super::{
        BoxedDetachable,
        Detachable,
    };
}
