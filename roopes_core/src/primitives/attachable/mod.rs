/// Objects can be attached to the implementing object.
pub trait Attachable<O>
{
    fn attach(
        &self,
        object: O,
    );
}

#[allow(clippy::module_name_repetitions)]
pub type BoxedAttachable<O> = Box<dyn Attachable<O>>;

pub mod prelude
{
    pub use super::{
        Attachable,
        BoxedAttachable,
    };
}
