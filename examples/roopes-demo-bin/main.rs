use roopes_lib::{
    abstract_factory::{
        lambda_abstract_factory::LambdaAbstractFactory,
        AbstractFactory,
    },
    command::lambda_command::LambdaCommand,
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

    let mut vs = VectorSubject::default();

    vs.attach(lc);

    vs.notify();
}
