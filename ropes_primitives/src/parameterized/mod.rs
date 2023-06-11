pub trait Parameterized<P>
{
    fn get_params(&self) -> &P;
    fn set_params(
        &self,
        params: P,
    );
}
