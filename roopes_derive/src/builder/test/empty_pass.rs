use roopes::prelude::*;

#[derive(Builder)]
struct TestStruct {
}

fn main(){
    let builder = TestStructBuilder::new();

    builder.build();
}