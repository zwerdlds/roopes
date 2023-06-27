pub mod blocks;
pub mod token_stream_visitor_factory;
pub mod transformer_params;

use proc_macro::TokenStream;
use token_stream_visitor_factory::TokenStreamVisitorFactory;

pub fn derive(input: TokenStream) -> TokenStream
{
    TokenStreamVisitorFactory::new_from_token_stream(input).build()
}
