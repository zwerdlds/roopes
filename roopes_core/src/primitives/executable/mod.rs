pub trait Delegate = Fn();

pub trait Executable
{
    fn execute(&self);
}

pub mod prelude
{
    pub use super::Executable;
}
