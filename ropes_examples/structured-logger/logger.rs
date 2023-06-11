use crate::{
    log_formatter::LogFormatter,
    log_message::LogMessage,
};
use ropes_lib::prelude::*;

#[derive(Builder)]
pub(crate) struct Logger
{
    printer: Box<dyn Handler<String>>,
    formatter: Box<dyn LogFormatter>,
}

impl Logger
{
    pub(crate) fn new(
        printer: Box<dyn Handler<String>>,
        formatter: Box<dyn LogFormatter>,
    ) -> Logger
    {
        Logger { printer, formatter }
    }

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
