use crate::log_message::LogMessage;

pub(crate) trait LogFormatter
{
    fn format_message(
        &self,
        msg: &LogMessage,
    ) -> String;
}
