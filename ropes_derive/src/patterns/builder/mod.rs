use proc_macro::TokenStream;
use syn::{
    parse_macro_input,
    DeriveInput,
};

pub fn build_tokens(input: TokenStream) -> TokenStream
{
    let ast = parse_macro_input!(input as DeriveInput);
    eprintln!("{ast:#?}");

    todo!()
}
