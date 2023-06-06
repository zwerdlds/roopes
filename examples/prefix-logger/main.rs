#![feature(trait_alias)]
#![feature(unboxed_closures)]

use crate::prefix_formatter::PrefixFormatter;
use log_message::LogMessage;
use logger::Logger;
use println_printer::PrintlnPrinter;

mod log_formatter;
mod log_message;
mod logger;
mod prefix_formatter;
mod println_printer;

fn main()
{
    let println_printer = PrintlnPrinter::new();
    let prefix_formatter = PrefixFormatter::new("Prefix Demo".into());

    let logger =
        Logger::new(Box::new(println_printer), Box::new(prefix_formatter));

    let msg = LogMessage::new("Message Demo".into());

    logger.log(&msg);
    logger.log(&msg);
    logger.log(&msg);
    logger.log(&msg);
}
