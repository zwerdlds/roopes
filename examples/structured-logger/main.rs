#![feature(trait_alias)]
#![feature(unboxed_closures)]

use crate::prefix_formatter::PrefixFormatter;
use log_message::LogMessage;
use log_message_subscriber::LogMessageSubscriber;
use log_publisher::LogPublisher;
use logger::Logger;
use println_printer::PrintlnPrinter;
use ropes::{
    prelude::{
        publisher_subscriber::vec_publisher::VecPublisher,
        SubscribingHandler,
    },
    primitives::Attachable,
};

mod log_formatter;
mod log_message;
mod log_message_subscriber;
mod log_publisher;
mod logger;
mod prefix_formatter;
mod println_printer;

fn main()
{
    let println_printer = PrintlnPrinter::new();
    let prefix_formatter = PrefixFormatter::new("Prefix Demo".into());

    let pretty_stdout_logger =
        Logger::new(Box::new(println_printer), Box::new(prefix_formatter));

    let pretty_stdout_logger = SubscribingHandler::new(pretty_stdout_logger);

    let mut log_publisher = LogPublisher::new(VecPublisher::default());

    log_publisher
        .attach(LogMessageSubscriber::new(Box::new(pretty_stdout_logger)));

    let msg = LogMessage::new("Message Demo".into());

    log_publisher.log(&msg);
    log_publisher.log(&msg);
    log_publisher.log(&msg);
    log_publisher.log(&msg);
}
