use super::{
    builder_token_stream::BuilderTokenStream,
    Builder,
};
use proc_macro::TokenStream;

pub fn derive(input: TokenStream) -> TokenStream
{
    let builder_token_stream = BuilderTokenStream::new_from_token_stream(input);

    builder_token_stream.build()
}
