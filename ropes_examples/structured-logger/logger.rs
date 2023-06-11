use crate::{
    log_formatter::LogFormatter,
    log_message::LogMessage,
};
use ropes_lib::prelude::*;
use std::rc::Rc;

#[derive(Builder)]
pub(crate) struct Logger
{
    printer: Rc<dyn Handler<String>>,
    formatter: Rc<dyn LogFormatter>,
}

impl Logger
{
    pub(crate) fn log(
        &self,
        message: &LogMessage,
    )
    {
        self.printer.handle(&self.formatter.format_message(message));
    }
}

impl Handler<LogMessage> for Logger
{
    fn handle(
        &self,
        message: &LogMessage,
    )
    {
        self.log(message);
    }
}
