use crate::{
    log_formatter::LogFormatter,
    log_message::LogMessage,
};
use std::cell::RefCell;

pub(crate) struct PrefixFormatter
{
    prefix: String,
    count: RefCell<usize>,
}

impl PrefixFormatter
{
    pub(crate) fn new(prefix: String) -> PrefixFormatter
    {
        PrefixFormatter {
            prefix,
            count: RefCell::new(0),
        }
    }

    pub(crate) fn increment(&self)
    {
        (*self.count.borrow_mut()) += 1;
    }
}

impl LogFormatter for PrefixFormatter
{
    fn format_message(
        &self,
        msg: &LogMessage,
    ) -> String
    {
        let prefix = &self.prefix;
        let ct = *self.count.borrow();
        let suffix = &msg.content;

        let output = format!("{ct} {prefix}: {suffix}");

        self.increment();
        output
    }
}
