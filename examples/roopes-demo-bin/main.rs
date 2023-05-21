use roopes_lib::{
    abstract_factory::{
        lambda_abstract_factory::LambdaAbstractFactory,
        AbstractFactory,
    },
    command::lambda_command::LambdaCommand,
    observer::{
        vec_subject::VecSubject,
        Attachable,
        Subject,
    },
    observing_command::ObservingCommand,
};

fn main()
{
    let lc_factory = LambdaAbstractFactory::new(|| {
        LambdaCommand::new(|| {
            println!("Hello, world!");
        })
    });

    let lc: ObservingCommand<_> = lc_factory.create().into();

    let mut vs = VecSubject::default();

    vs.attach(lc);

    vs.notify();
}
