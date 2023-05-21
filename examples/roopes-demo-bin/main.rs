use roopes_lib::{
    abstract_factory::{
        lambda_abstract_factory::LambdaAbstractFactory,
        AbstractFactory,
    },
    command::lambda_command::LambdaCommand,
    crosscutting::observing_command::ObservingCommand,
    observer::{
        vector_subject::VectorSubject,
        Attachable,
        Subject,
    },
};

fn main()
{
    let lc_factory = LambdaAbstractFactory::new(|| {
        LambdaCommand::new(|| {
            println!("Hello, world!");
        })
    });

    let lc = lc_factory.create();

    let mut vs: VectorSubject<ObservingCommand<_>> = VectorSubject::default();

    vs.attach(lc.into());

    vs.notify();
}
