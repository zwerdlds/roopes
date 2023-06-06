pub(crate) struct LogMessage
{
    pub(crate) content: String,
}

impl LogMessage
{
    pub(crate) fn new(content: String) -> LogMessage
    {
        LogMessage { content }
    }
}
