#![feature(trait_alias)]
#![feature(unboxed_closures)]

// pub mod support;

mod log_handler;

fn main()
{
    println!("DNF");
    // let msg = Rc::new("Hello World".into());
    // let logger_handler_builder =
    //     Rc::new(RefCell::new(log_msg_handler_builder()));

    // let mut logger_command_builder = LambdaBuilder::new(
    //     |p| {
    //         let ct = RefCell::new(0);

    //         logger_handler_builder
    //             .borrow_mut()
    //             .params()
    //             .set_name(p.get_name().into());

    //         let call_logger: LambdaHandler<_, i32> =
    //             logger_handler_builder.borrow().build();

    //         LambdaCommand::new(move || {
    //             (*ct.borrow_mut()) += 1;
    //             call_logger.handle(&ct.borrow());
    //         })
    //     },
    //     MsgPrefix::default(),
    // );

    // logger_command_builder.params().set_name(msg);

    // let lc_factory =
    //     LambdaAbstractFactory::new(|| logger_command_builder.build());

    // let lc1: ObservingCommand<_> = lc_factory.create().into();

    // let mut vs = VecSubject::default();

    // vs.attach(lc1);

    // vs.notify();
    // vs.notify();
    // vs.notify();

    // let lc2: ObservingCommand<_> = lc_factory.create().into();
    // lc2.execute();
}
