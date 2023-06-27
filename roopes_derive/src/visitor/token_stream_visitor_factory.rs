use super::{
    blocks::{
        AcceptorTransformer,
        PreambleTransformer,
        VisitorTraitTransformer,
    },
    visitor_transformer_params::VisitorTransformerParams,
};
use crate::common::VecTokenStringTransformer;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use roopes_core::{
    patterns::transformer_chain,
    prelude::*,
};
use syn::{
    parse_macro_input,
    DataEnum,
    DeriveInput,
};

pub(super) struct TokenStreamVisitorFactory
{
    token_stream: proc_macro::TokenStream,
}

impl TokenStreamVisitorFactory
{
    pub(super) fn new_from_token_stream(token_stream: TokenStream1) -> Self
    {
        Self { token_stream }
    }

    pub fn build(&self) -> TokenStream1
    {
        let tokens = self.token_stream.clone();
        let ast = parse_macro_input!(tokens as syn::DeriveInput);

        let result: TokenStream2 = transformer_chain::heap::Head::new(
            Box::new(TokenStreamToVisitorTransformerParamsTransformer),
        )
        .push(VisitorTransformer)
        .transform(&ast);

        // eprintln!("Results:");
        // let contents =
        // syn::parse_file(&result.to_string()).
        // expect(     "Syn parsing
        // failed.",
        // );
        // let formatted =
        // prettyplease::unparse(&contents);
        // eprintln!("{formatted}");

        result.into()
    }
}

struct VisitorTransformer;
impl Transformer<VisitorTransformerParams, TokenStream2> for VisitorTransformer
{
    fn transform(
        &self,
        shared: &VisitorTransformerParams,
    ) -> TokenStream2
    {
        let elements = vec![
            PreambleTransformer.transform(&shared),
            VisitorTraitTransformer.transform(&shared),
            AcceptorTransformer.transform(&shared),
        ];

        VecTokenStringTransformer.transform(&elements)
    }
}

pub(super) struct TokenStreamToVisitorTransformerParamsTransformer;
impl Transformer<DeriveInput, VisitorTransformerParams>
    for TokenStreamToVisitorTransformerParamsTransformer
{
    fn transform(
        &self,
        input: &DeriveInput,
    ) -> VisitorTransformerParams
    {
        let ast = input.clone();
        let visibility = ast.vis.clone();
        let visit_target = ast.ident.clone();
        let visitor = format_ident!("{visit_target}Visitor");
        let acceptor = format_ident!("{visit_target}Acceptor");

        let syn::DeriveInput {
            data:
                syn::Data::Enum(DataEnum{
                    variants,
                    ..
                }),
            ..
        } = ast else {
            unimplemented!(
                "derive(Visitor) only supports enums"
            )
        };

        VisitorTransformerParams {
            visibility,
            visit_target,
            visitor,
            acceptor,
            variants,
        }
    }
}
