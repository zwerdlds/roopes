use proc_macro2::TokenStream;
use quote::quote;
use roopes_core::prelude::*;

pub(crate) struct VecTokenStringTransformer;
impl Transformer<Vec<TokenStream>, TokenStream> for VecTokenStringTransformer
{
    fn transform(
        &self,
        elements: &Vec<TokenStream>,
    ) -> TokenStream
    {
        let elements = elements.iter().cloned();

        quote! {
            #(#elements)*
        }
    }
}
