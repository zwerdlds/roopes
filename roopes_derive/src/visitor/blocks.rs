use super::transformer_params::TransformerParams;
use crate::common::VecTokenStringTransformer;
use proc_macro2::TokenStream;
use quote::quote;
use roopes_core::prelude::*;

pub(super) struct PreambleTransformer;
impl Transformer<TransformerParams, TokenStream> for PreambleTransformer
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let visit_target = input.visit_target.clone();

        quote! {
            use #visit_target::*;
        }
    }
}

pub(super) struct VisitorTraitTransformer;
impl Transformer<TransformerParams, TokenStream> for VisitorTraitTransformer
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let visibility = input.visibility.clone();
        let visitor = input.visitor.clone();
        let visitor_fn_names = input.visitor_fn_names();
        let variants_field_params = input.variants_field_params();

        let visitor_fn_sig_params =
            variants_field_params.into_iter().map(|params| {
                params.into_iter().map(|(ty, name)| quote! {#name: &#ty})
            });

        let visitor_fn_signatures = visitor_fn_names
            .into_iter()
            .zip(visitor_fn_sig_params)
            .map(|(fn_name, fn_params)| {
                quote! { fn #fn_name(
                    &self,
                    #(#fn_params),*
                ) }
            });

        quote! {
            #visibility trait #visitor {
                #(#visitor_fn_signatures);*;
            }
        }
    }
}

pub(super) struct AcceptorTransformer;
impl Transformer<TransformerParams, TokenStream> for AcceptorTransformer
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let elements = vec![
            AcceptorStructTransformer.transform(input),
            AcceptorImplTransformer.transform(input),
            AcceptorHandlerTransformer.transform(input),
        ];

        VecTokenStringTransformer.transform(&elements)
    }
}

pub(super) struct AcceptorStructTransformer;
impl Transformer<TransformerParams, TokenStream> for AcceptorStructTransformer
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let visibility = input.visibility.clone();
        let visitor = input.visitor.clone();
        let acceptor = input.acceptor.clone();

        quote! {
            #visibility struct #acceptor<V>
                where V: #visitor
            {
                delegate: V
            }
        }
    }
}

pub(super) struct AcceptorImplTransformer;
impl Transformer<TransformerParams, TokenStream> for AcceptorImplTransformer
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let elements = VecTokenStringTransformer.transform(&vec![
            AcceptorImplNewFnTransformer.transform(input),
            AcceptorImplAcceptFnTransformer.transform(input),
        ]);

        let visitor = input.visitor.clone();
        let acceptor = input.acceptor.clone();

        quote! {
            impl<V> #acceptor<V> where V: #visitor {
                #elements
            }
        }
    }
}

pub(super) struct AcceptorHandlerTransformer;
impl Transformer<TransformerParams, TokenStream> for AcceptorHandlerTransformer
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let visitor = input.visitor.clone();
        let visit_target = input.visit_target.clone();
        let acceptor = input.acceptor.clone();
        let handler = quote! {
            roopes::primitives::handler::Handler<#visit_target>
        };

        quote! {
            impl<V> #handler for #acceptor<V> where V: #visitor {
                fn handle(&self, e: &#visit_target) {
                    self.accept(e);
                }
            }
        }
    }
}

pub(super) struct AcceptorImplNewFnTransformer;
impl Transformer<TransformerParams, TokenStream>
    for AcceptorImplNewFnTransformer
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let visibility = input.visibility.clone();
        let acceptor = input.acceptor.clone();

        quote! {
            #visibility fn new(delegate: V) -> #acceptor<V>
            {
                #acceptor {
                    delegate
                }
            }
        }
    }
}

pub(super) struct AcceptorImplAcceptFnTransformer;
impl Transformer<TransformerParams, TokenStream>
    for AcceptorImplAcceptFnTransformer
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let visibility = input.visibility.clone();
        let visit_target = input.visit_target.clone();
        let variant_ids = input.variant_ids().into_iter();
        let visitor_fn_names = input.visitor_fn_names();
        let variants_field_names = input.variants_field_names();

        let visitor_fn_calls = visitor_fn_names
            .into_iter()
            .zip(variants_field_names.clone())
            .map(|(name, args)| {
                quote! {
                    self.delegate.#name ( #(#args),* )
                }
            });

        let match_branches = variant_ids
            .zip(variants_field_names)
            .zip(visitor_fn_calls)
            .map(|((id, arg_names), func_call)| {
                let destructure_fields = {
                    if arg_names.is_empty() {
                        quote! {}
                    } else {
                        quote! {
                            { #(#arg_names),* }
                        }
                    }
                };

                quote! {
                    #id #destructure_fields => { #func_call; }
                }
            });

        quote! {
            #visibility fn accept(&self, e: &#visit_target) {
                match e {
                    #(#match_branches),*
                }
            }
        }
    }
}
