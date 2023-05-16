use roopes_lib::{
    command::lambda_command::LambdaCommand,
    observer::{
        vector_subject::VectorSubject,
        Attachable,
        Subject,
    },
};

fn main()
{
    let lc = LambdaCommand::new(|| {
        println!("Hello, world!");
    });

    let mut vs = VectorSubject::default();

    vs.attach(lc);

    vs.notify();
}
