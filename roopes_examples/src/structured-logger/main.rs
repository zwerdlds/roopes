#![feature(trait_alias)]
#![feature(unboxed_closures)]

mod log_formatter;
mod log_message;
mod log_message_subscriber;
mod log_publisher;
mod logger;
mod prefix_formatter;
mod println_printer;

#[cfg(test)]
mod test;

use crate::prefix_formatter::PrefixFormatter;
use log_message::LogMessage;
use log_message_subscriber::LogMessageSubscriber;
use log_publisher::LogPublisher;
use logger::LoggerBuilder;
use println_printer::PrintlnPrinter;
use roopes::prelude::*;
use std::rc::Rc;

fn main()
{
    let println_printer = Rc::new(PrintlnPrinter::new());
    let prefix_formatter = Rc::new(PrefixFormatter::new("Prefix Demo".into()));

    let logger_builder = LoggerBuilder::new()
        .set_printer(println_printer)
        .set_formatter(prefix_formatter);

    let pretty_stdout_logger = SubscribingHandler::new(logger_builder.build());

    let mut log_publisher =
        LogPublisher::new(publisher_subscriber::VecPublisher::default());

    log_publisher
        .attach(LogMessageSubscriber::new(Box::new(pretty_stdout_logger)));

    let msg = LogMessage::new("Message Demo".into());

    log_publisher.log(&msg);
    log_publisher.log(&msg);
    log_publisher.log(&msg);
    log_publisher.log(&msg);
}
