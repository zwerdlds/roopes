use crate::{
    log_message::LogMessage,
    log_message_subscriber::LogMessageSubscriber,
};
use delegate::delegate;
use ropes_lib::prelude::{
    publisher_subscriber::VecPublisher,
    *,
};

#[derive(Default)]
pub(crate) struct LogPublisher
{
    publisher: VecPublisher<LogMessage, LogMessageSubscriber>,
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
        publisher: VecPublisher<LogMessage, LogMessageSubscriber>
    ) -> Self
    {
        LogPublisher { publisher }
    }
}

impl Attachable<LogMessageSubscriber> for LogPublisher
{
    delegate! {
        to self.publisher{
            fn attach(&mut self, subscriber: LogMessageSubscriber);
        }
    }
}
