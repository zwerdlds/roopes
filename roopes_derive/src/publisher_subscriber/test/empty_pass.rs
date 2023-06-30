use roopes::prelude::*;

#[derive(PubSub)]
pub struct TestStruct;

fn main()
{
    let builder = TestStructPublisher::default();

    builder.publish(&TestStruct);
}
