use ropes_lib::prelude::*;
use std::cell::RefCell;

// This demo shows how OOP + Rust can be used to create a mostly functional but
// non-coupled logging system.
fn main()
{
    // The parameters for the builder.
    struct GreetingMessage
    {
        prefix: &'static str,
    }

    let greeting_message = RefCell::new(GreetingMessage { prefix: "AAA" });

    // Make a builder which takes `GreetingMessage` as parameters.
    let logger_factory = abstract_factory::Lambda::new(|| {
        // Pull out the prefix at lambda build time: it does not change
        // after the lambda is built.
        let prefix = (*greeting_message.borrow()).prefix;

        // Create local state for the number of times the lambda is called.
        let ct = RefCell::new(0);

        // Build the lambda.
        command::Lambda::new(move || {
            // Increment the local count.
            (*ct.borrow_mut()) += 1;

            // Prep the count for more legible formatting.
            let count = ct.borrow();

            // Print the message.
            println!("{prefix}: called {count} time(s)");
        })
    });

    // Get a logger with the unset prefix
    let aaa_logger: ObservingCommand<_> = logger_factory.create().into();

    // Demonstrate unmodified prefix.
    aaa_logger.execute();

    // Change the prefix after the builder is setup.
    greeting_message.replace(GreetingMessage {
        prefix: "BBB".into(),
    });

    // Create two independent loggers.
    let bbb_logger: ObservingCommand<_> = logger_factory.create().into();

    // Change the prefix after the builder is setup.
    greeting_message.replace(GreetingMessage {
        prefix: "CCC".into(),
    });
    let ccc_logger: ObservingCommand<_> = logger_factory.create().into();

    // Attach a logger to a minimal subject.
    let mut example_subject = observer::VecSubject::default();
    example_subject.attach(bbb_logger);

    // Demonstrate the lambda being called repeatedly.
    example_subject.notify();
    example_subject.notify();
    example_subject.notify();

    // Demonstrate the other logger is unmodified.
    ccc_logger.execute();
}

#[cfg(test)]
mod tests
{
    use crate::main;

    #[test]
    fn run_main()
    {
        main();
    }
}
