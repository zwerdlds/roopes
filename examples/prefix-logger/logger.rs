use crate::{
    log_formatter::LogFormatter,
    log_message::LogMessage,
};
use rope_lib::prelude::*;

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
        msg: &LogMessage,
    )
    {
        self.printer.handle(&self.formatter.format_message(msg));
    }
}
