use crate::log_message::LogMessage;
use ropes::prelude::*;

pub(crate) struct LogMessageSubscriber
{
    log_subscriber: Box<dyn Subscriber<LogMessage>>,
}

impl LogMessageSubscriber
{
    pub(crate) fn new(
        log_subscriber: Box<dyn Subscriber<LogMessage>>
    ) -> LogMessageSubscriber
    {
        LogMessageSubscriber { log_subscriber }
    }
}

impl Subscriber<LogMessage> for LogMessageSubscriber
{
    fn receive(
        &self,
        message: &LogMessage,
    )
    {
        self.log_subscriber.receive(message)
    }
}
