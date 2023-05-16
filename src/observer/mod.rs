pub mod command_observer;
pub mod hash_subject;
pub mod vector_subject;

/// An object which notifies some group of [`Observer`]s.
pub trait Subject
{
    fn notify(&self);
}

/// An object which can be notified.
pub trait Observer
{
    fn notify(&self);
}

/// [`Subject`]s can also act as [`Observer`]s.  This enables them to chain
/// notifications.
impl<O> Subject for O
where
    O: Observer,
{
    fn notify(&self)
    {
        self.notify()
    }
}

/// Gives the ability for additional [`Observer`]s to be added to the list of
/// notified objects.
pub trait Attachable<O>
where
    O: Observer,
{
    fn attach(
        &mut self,
        attach_observer: O,
    );
}

/// An Error which occurs during detachment.
pub enum ObserverDetachError
{
    /// The specified observer couldn't be found.
    ObserverNotFound,
}

/// Gives the ability for [`Observer`]s to be removed from the list of notified
/// objects.
pub trait Detachable<O>
where
    O: Observer,
{
    fn detach(
        &mut self,
        detach_observer: O,
    ) -> Result<(), ObserverDetachError>;
}
