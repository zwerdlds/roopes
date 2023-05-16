use crate::command::Command;

pub mod vector_observer;

pub trait Observer = Command;

pub trait Observable
{
    fn attach(
        &mut self,
        observer: dyn Observer,
    );
}
