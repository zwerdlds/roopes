use crate::{
    log_message::LogMessage,
    log_message_subscriber::LogMessageSubscriber,
};
use delegate::delegate;
use roopes::prelude::*;

#[derive(Default)]
pub(crate) struct LogPublisher
{
    publisher:
        publisher_subscriber::VecPublisher<LogMessage, LogMessageSubscriber>,
}

impl LogPublisher
{
    pub fn log(
        &self,
        message: &LogMessage,
    )
    {
        self.publisher.publish(message);
    }

    pub(crate) fn new(
        publisher: publisher_subscriber::VecPublisher<
            LogMessage,
            LogMessageSubscriber,
        >
    ) -> Self
    {
        LogPublisher { publisher }
    }
}

#[allow(clippy::inline_always)]
impl Publisher<LogMessage> for LogPublisher
{
    delegate! {
        to self.publisher{
            fn publish(&self, message: &LogMessage);
        }
    }
}

#[allow(clippy::inline_always)]
impl AttachablePublisher<LogMessage, LogMessageSubscriber> for LogPublisher
{
    delegate! {
        to self.publisher{
            fn attach(&mut self, subscriber: LogMessageSubscriber);
        }
    }
}
