pub mod token_stream_builder_builder;

use super::Builder;
use proc_macro::TokenStream;
use token_stream_builder_builder::TokenStreamBuilderBuilder;

pub fn derive(input: TokenStream) -> TokenStream
{
    let builder_token_stream =
        TokenStreamBuilderBuilder::new_from_token_stream(input);

    builder_token_stream.build()
}
