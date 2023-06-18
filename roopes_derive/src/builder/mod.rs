pub mod token_stream_builder_factory;

use proc_macro::TokenStream;
use token_stream_builder_factory::TokenStreamBuilderFactory;

pub fn derive(input: TokenStream) -> TokenStream
{
    TokenStreamBuilderFactory::new_from_token_stream(input).build()
}
