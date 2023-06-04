use roopes_lib::{
    builder::Builder,
    handler::{
        lambda_handler::LambdaHandler,
        Handler,
    },
};

pub struct LogHandler
{
    delegate: Box<dyn Handler<String>>,
}

impl LogHandler
{
    pub fn new(delegate: Box<dyn Handler<String>>) -> LogHandler
    {
        LogHandler { delegate }
    }
}

impl Handler<String> for LogHandler
{
    fn handle(
        &self,
        message: &String,
    )
    {
        (self.delegate).handle(message)
    }
}

pub struct LogHandlerParams
{
    prefix: &'static str,
    // formatter: Rc<RefCell<>>
}

pub struct LogHandlerBuilder
{
    delegate: Box<dyn Builder<LogHandlerParams, LogHandler>>,
}

impl Builder<LogHandlerParams, LogHandler> for LogHandlerBuilder
{
    fn build(&self) -> LogHandler
    {
        let prefix = self.delegate.params().prefix;

        let delegate = Box::new(LambdaHandler::new(move |msg: &String| {
            println!("{prefix}: {msg}");
        }));

        LogHandler::new(delegate)
    }

    fn params(&self) -> &LogHandlerParams
    {
        self.delegate.params()
    }

    fn set_params(
        &mut self,
        params: LogHandlerParams,
    )
    {
        self.delegate.set_params(params)
    }
}
