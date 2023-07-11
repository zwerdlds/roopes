mod blocks;
pub mod token_stream_builder_factory;
mod transformer_params;

use proc_macro::TokenStream;
use token_stream_builder_factory::TokenStreamBuilderFactory;

pub fn derive(input: TokenStream) -> TokenStream
{
    TokenStreamBuilderFactory::new_from_token_stream(input).build()
}
