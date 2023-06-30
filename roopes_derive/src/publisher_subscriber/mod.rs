pub mod token_stream_pubsub_factory;
use self::token_stream_pubsub_factory::TokenStreamPubSubFactory;
use proc_macro::TokenStream;

pub fn derive(input: TokenStream) -> TokenStream
{
    TokenStreamPubSubFactory::new_from_token_stream(input).build()
}
