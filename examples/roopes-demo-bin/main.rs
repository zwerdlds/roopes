use roopes_lib::{
    abstract_factory::{
        lambda_abstract_factory::LambdaAbstractFactory,
        AbstractFactory,
    },
    builder::{
        lambda_builder::LambdaBuilder,
        Builder,
    },
    command::{
        lambda_command::LambdaCommand,
        Command,
    },
    observer::{
        vec_subject::VecSubject,
        Attachable,
        Subject,
    },
    observing_command::ObservingCommand,
};
use std::cell::RefCell;

fn main()
{
    struct TextParams(&'static str);

    let mut builder = LambdaBuilder::new(
        |t| {
            let msg = t.0;
            let ct = RefCell::new(0);

            LambdaCommand::new(move || {
                (*ct.borrow_mut()) += 1;
                println!("{msg}: called {} time(s)", ct.borrow());
            })
        },
        TextParams("unset".into()),
    );

    builder.params().0 = "Hello World!".into();

    let lc_factory = LambdaAbstractFactory::new(|| builder.build());

    let lc1: ObservingCommand<_> = lc_factory.create().into();

    let mut vs = VecSubject::default();

    vs.attach(lc1);

    vs.notify();
    vs.notify();
    vs.notify();

    let lc2: ObservingCommand<_> = lc_factory.create().into();
    lc2.execute();
}
