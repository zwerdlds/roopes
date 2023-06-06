use ropes_lib::prelude::*;

pub(crate) struct PrintlnPrinter {}

impl PrintlnPrinter
{
    pub(crate) fn new() -> PrintlnPrinter
    {
        PrintlnPrinter {}
    }
}

impl Handler<String> for PrintlnPrinter
{
    fn handle(
        &self,
        message: &String,
    )
    {
        println!("{message}")
    }
}
