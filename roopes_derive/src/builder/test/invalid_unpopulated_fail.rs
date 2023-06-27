use roopes::prelude::*;

#[derive(Builder)]
struct TestStruct {
    value_one: i32,
}

fn main(){
    let builder = TestStructBuilder::new();

    let test_struct = builder.build();

    assert_eq!(0, test_struct.value_one);
}