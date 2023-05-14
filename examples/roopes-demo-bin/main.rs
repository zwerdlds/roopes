use roopes_lib::command::{
    lambda_command::LambdaCommand,
    Command,
};

fn main()
{
    let lc = LambdaCommand::new(|| {
        println!("Hello, world!");
    });

    lc.execute();
}
