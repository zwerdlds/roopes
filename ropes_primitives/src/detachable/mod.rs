/// Objects can be removed from something.
pub trait Detachable<O, E>
{
    ///Removed the object from [Self].
    /// # Errors
    /// E: The detaching somehow encountered a fault.
    fn detach(
        &mut self,
        object: O,
    ) -> Result<(), E>;
}
