use super::{
    blocks::{
        BuildImpl,
        Builder,
        FieldIndicatorSection,
        NewImpl,
        SettersImpl,
        TargetImpl,
    },
    transformer_params::TransformerParams,
};
use crate::common::VecTokenStringTransformer;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use roopes_core::prelude::*;
use syn::{
    parse_macro_input,
    DeriveInput,
};

pub(super) struct TokenStreamBuilderFactory
{
    token_stream: TokenStream1,
}

impl TokenStreamBuilderFactory
{
    pub(super) fn new_from_token_stream(token_stream: TokenStream1) -> Self
    {
        Self { token_stream }
    }

    // #[allow(clippy::too_many_lines)]
    pub fn build(&self) -> TokenStream1
    {
        let tokens = self.token_stream.clone();
        let ast = parse_macro_input!(tokens as syn::DeriveInput);

        let result: TokenStream2 = transformer_chain::heap::Head::new(
            Box::new(TokenStreamToTransformerParamsTransformer),
        )
        .push(BuilderTransformer)
        .transform(&ast);

        // eprintln!("{}", result.to_string());
        // eprintln!("Formatted Results:");
        // let contents = syn::parse_file(&result.to_string()).expect(
        //     "Syn parsing
        // failed",
        // );
        // let formatted = prettyplease::unparse(&contents);
        // eprintln!("{formatted}");

        result.into()
    }
}

struct BuilderTransformer;
impl Transformer<TransformerParams, TokenStream2> for BuilderTransformer
{
    fn transform(
        &self,
        shared: &TransformerParams,
    ) -> TokenStream2
    {
        let elements = vec![
            FieldIndicatorSection.transform(shared),
            Builder.transform(shared),
            NewImpl.transform(shared),
            TargetImpl.transform(shared),
            SettersImpl.transform(shared),
            BuildImpl.transform(shared),
        ];

        VecTokenStringTransformer.transform(&elements)
    }
}

pub(super) struct TokenStreamToTransformerParamsTransformer;
impl Transformer<DeriveInput, TransformerParams>
    for TokenStreamToTransformerParamsTransformer
{
    fn transform(
        &self,
        input: &DeriveInput,
    ) -> TransformerParams
    {
        let ast = input.clone();
        let visibility = ast.vis.clone();
        let target = ast.ident.clone();
        let builder = format_ident!("{target}Builder");

        let syn::DeriveInput {
            data:
                syn::Data::Struct(syn::DataStruct {
                    fields:
                        syn::Fields::Named(syn::FieldsNamed {
                            named: fields, ..
                        }),
                    ..
                }),
            ..
        } = ast else {
            unimplemented!(
                "derive(Builder) only supports structs with named fields"
            )
        };

        TransformerParams {
            visibility,
            target,
            builder,
            fields,
        }
    }
}
