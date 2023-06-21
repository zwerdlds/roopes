use heck::ToSnekCase;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{
    Ident,
    TokenStream as TokenStream2,
};
use quote::{
    format_ident,
    quote,
};
use roopes_core::{
    patterns::transformer_chain,
    prelude::*,
};
use syn::{
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    DataEnum,
    DeriveInput,
    Type,
    Variant,
    Visibility,
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

#[derive(Clone)]
struct VisitorTransformerParams
{
    visibility: Visibility,
    visit_target: Ident,
    visitor: Ident,
    acceptor: Ident,
    variants: Punctuated<Variant, Comma>,
}

struct VisitorTransformer;
impl VisitorTransformer
{
    fn build_parameters(
        shared: &VisitorTransformerParams
    ) -> (
        PreambleTransformerParams,
        VisitorTraitTransformerParams,
        AcceptorTransformerParams,
    )
    {
        let variant_ids =
            shared.variants.iter().map(|variant| variant.clone().ident);

        let visitor_fn_names = variant_ids
            .clone()
            .map(|id| id.to_string().to_snek_case())
            .map(|id| format_ident!("visit_{id}"));

        let variants_fields = shared
            .variants
            .clone()
            .into_iter()
            .map(|variant| variant.fields.into_iter());

        let variants_field_params = variants_fields.map(|fields| {
            fields.map(|field| {
                (
                    field.ty.clone(),
                    field.ident.expect(
                        "All derive(Visitor) enum fields must be named",
                    ),
                )
            })
        });

        let variants_field_names = variants_field_params
            .clone()
            .map(|fields| fields.map(|(_, name)| name));

        let visibility = shared.visibility.clone();
        let visit_target = shared.visit_target.clone();
        let visitor = shared.visitor.clone();
        let acceptor = shared.acceptor.clone();

        let variant_ids: Vec<_> = variant_ids.collect();
        let visitor_fn_names: Vec<_> = visitor_fn_names.collect();
        let variants_field_params: Vec<Vec<_>> =
            variants_field_params.map(Iterator::collect).collect();
        let variants_field_names: Vec<Vec<_>> =
            variants_field_names.map(Iterator::collect).collect();

        let preamble_params = PreambleTransformerParams {
            visit_target: visit_target.clone(),
        };

        let visitor_trait_transformer_params = VisitorTraitTransformerParams {
            visibility: visibility.clone(),
            visitor: visitor.clone(),
            visitor_fn_names: visitor_fn_names.clone(),
            variants_field_params,
        };

        let acceptor_transformer_params = AcceptorTransformerParams {
            visibility,
            visit_target,
            visitor,
            acceptor,
            variant_ids,
            visitor_fn_names,
            variants_field_names,
        };

        (
            preamble_params,
            visitor_trait_transformer_params,
            acceptor_transformer_params,
        )
    }
}
impl Transformer<VisitorTransformerParams, TokenStream2> for VisitorTransformer
{
    fn transform(
        &self,
        shared: &VisitorTransformerParams,
    ) -> TokenStream2
    {
        let (
            preamble_params,
            visitor_trait_transformer_params,
            acceptor_transformer_params,
        ) = Self::build_parameters(shared);

        let elements = vec![
            PreambleTransformer.transform(&preamble_params),
            VisitorTraitTransformer
                .transform(&visitor_trait_transformer_params),
            AcceptorTransformer.transform(&acceptor_transformer_params),
        ];

        VecTokenStringTransformer.transform(&elements)
    }
}

struct TokenStreamToVisitorTransformerParamsTransformer;
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

struct PreambleTransformerParams
{
    visit_target: Ident,
}
struct PreambleTransformer;
impl Transformer<PreambleTransformerParams, TokenStream2>
    for PreambleTransformer
{
    fn transform(
        &self,
        input: &PreambleTransformerParams,
    ) -> TokenStream2
    {
        let visit_target = input.visit_target.clone();

        quote! {
            use #visit_target::*;
        }
    }
}

struct VisitorTraitTransformerParams
{
    visibility: Visibility,
    visitor: Ident,
    variants_field_params: Vec<Vec<(Type, Ident)>>,
    visitor_fn_names: Vec<Ident>,
}
struct VisitorTraitTransformer;
impl Transformer<VisitorTraitTransformerParams, TokenStream2>
    for VisitorTraitTransformer
{
    fn transform(
        &self,
        input: &VisitorTraitTransformerParams,
    ) -> TokenStream2
    {
        let visibility = input.visibility.clone();
        let visitor = input.visitor.clone();
        let visitor_fn_names = input.visitor_fn_names.clone();
        let variants_field_params = input.variants_field_params.clone();

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

struct AcceptorTransformerParams
{
    visibility: Visibility,
    visit_target: Ident,
    visitor: Ident,
    acceptor: Ident,
    variant_ids: Vec<Ident>,
    visitor_fn_names: Vec<Ident>,
    variants_field_names: Vec<Vec<Ident>>,
}
struct AcceptorTransformer;
impl Transformer<AcceptorTransformerParams, TokenStream2>
    for AcceptorTransformer
{
    fn transform(
        &self,
        input: &AcceptorTransformerParams,
    ) -> TokenStream2
    {
        let struct_input = AcceptorStructTransformerParams {
            visibility: input.visibility.clone(),
            visitor: input.visitor.clone(),
            acceptor: input.acceptor.clone(),
        };

        let impl_input = AcceptorImplTransformerParams {
            visibility: input.visibility.clone(),
            visit_target: input.visit_target.clone(),
            visitor: input.visitor.clone(),
            acceptor: input.acceptor.clone(),
            variant_ids: input.variant_ids.clone(),
            visitor_fn_names: input.visitor_fn_names.clone(),
            variants_field_names: input.variants_field_names.clone(),
        };

        let elements = vec![
            AcceptorStructTransformer.transform(&struct_input),
            AcceptorImplTransformer.transform(&impl_input),
        ];

        VecTokenStringTransformer.transform(&elements)
    }
}

struct AcceptorStructTransformerParams
{
    visibility: Visibility,
    visitor: Ident,
    acceptor: Ident,
}
struct AcceptorStructTransformer;
impl Transformer<AcceptorStructTransformerParams, TokenStream2>
    for AcceptorStructTransformer
{
    fn transform(
        &self,
        input: &AcceptorStructTransformerParams,
    ) -> TokenStream2
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

struct AcceptorImplTransformerParams
{
    visibility: Visibility,
    visit_target: Ident,
    visitor: Ident,
    acceptor: Ident,
    variant_ids: Vec<Ident>,
    visitor_fn_names: Vec<Ident>,
    variants_field_names: Vec<Vec<Ident>>,
}
struct AcceptorImplTransformer;
impl Transformer<AcceptorImplTransformerParams, TokenStream2>
    for AcceptorImplTransformer
{
    fn transform(
        &self,
        input: &AcceptorImplTransformerParams,
    ) -> TokenStream2
    {
        let visitor = input.visitor.clone();
        let acceptor = input.acceptor.clone();

        let elements = VecTokenStringTransformer.transform(&vec![
            AcceptorImplNewFnTransformer.transform(
                &AcceptorImplNewFnTransformerParams {
                    acceptor: input.acceptor.clone(),
                },
            ),
            AcceptorImplAcceptFnTransformer.transform(
                &AcceptorImplAcceptFnTransformerParams {
                    visibility: input.visibility.clone(),
                    visit_target: input.visit_target.clone(),
                    variant_ids: input.variant_ids.clone(),
                    visitor_fn_names: input.visitor_fn_names.clone(),
                    variants_field_names: input.variants_field_names.clone(),
                },
            ),
        ]);

        quote! {
            impl<V> #acceptor<V> where V: #visitor {
                #elements
            }
        }
    }
}

struct AcceptorImplNewFnTransformerParams
{
    acceptor: Ident,
}
struct AcceptorImplNewFnTransformer;
impl Transformer<AcceptorImplNewFnTransformerParams, TokenStream2>
    for AcceptorImplNewFnTransformer
{
    fn transform(
        &self,
        input: &AcceptorImplNewFnTransformerParams,
    ) -> TokenStream2
    {
        let acceptor = input.acceptor.clone();

        quote! {
            fn new(delegate: V) -> #acceptor<V>
            {
                #acceptor {
                    delegate
                }
            }
        }
    }
}

struct AcceptorImplAcceptFnTransformerParams
{
    visibility: Visibility,
    visit_target: Ident,
    variant_ids: Vec<Ident>,
    visitor_fn_names: Vec<Ident>,
    variants_field_names: Vec<Vec<Ident>>,
}
struct AcceptorImplAcceptFnTransformer;
impl Transformer<AcceptorImplAcceptFnTransformerParams, TokenStream2>
    for AcceptorImplAcceptFnTransformer
{
    fn transform(
        &self,
        input: &AcceptorImplAcceptFnTransformerParams,
    ) -> TokenStream2
    {
        let visibility = input.visibility.clone();
        let visit_target = input.visit_target.clone();
        let variant_ids = input.variant_ids.clone().into_iter();
        let visitor_fn_names = input.visitor_fn_names.clone();
        let variants_field_names = input.variants_field_names.clone();

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

struct VecTokenStringTransformer;
impl Transformer<Vec<TokenStream2>, TokenStream2> for VecTokenStringTransformer
{
    fn transform(
        &self,
        elements: &Vec<TokenStream2>,
    ) -> TokenStream2
    {
        let elements = elements.iter().cloned();

        quote! {
            #(#elements)*
        }
    }
}
