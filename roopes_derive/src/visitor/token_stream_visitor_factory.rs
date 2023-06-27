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
    Fields,
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

impl VisitorTransformerParams
{
    fn variant_ids(&self) -> Vec<Ident>
    {
        self.variants
            .iter()
            .map(|variant| variant.clone().ident)
            .collect()
    }

    fn visitor_fn_names(&self) -> Vec<Ident>
    {
        self.variant_ids()
            .into_iter()
            .map(|id| id.to_string().to_snek_case())
            .map(|id| format_ident!("visit_{id}"))
            .collect()
    }

    fn variants_fields(&self) -> Vec<Fields>
    {
        self.variants
            .clone()
            .into_iter()
            .map(|variant| variant.fields)
            .collect()
    }

    fn variants_field_params(&self) -> Vec<Vec<(Type, Ident)>>
    {
        self.variants_fields()
            .into_iter()
            .map(|fields| {
                fields
                    .into_iter()
                    .map(|field| {
                        (
                            field.ty.clone(),
                            field.ident.expect(
                                "All derive(Visitor) enum fields must be named",
                            ),
                        )
                    })
                    .collect()
            })
            .collect()
    }

    fn variants_field_names(&self) -> Vec<Vec<Ident>>
    {
        self.variants_field_params()
            .into_iter()
            .map(|fields| fields.into_iter().map(|(_, name)| name).collect())
            .collect()
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

struct PreambleTransformer;
impl Transformer<VisitorTransformerParams, TokenStream2> for PreambleTransformer
{
    fn transform(
        &self,
        input: &VisitorTransformerParams,
    ) -> TokenStream2
    {
        let visit_target = input.visit_target.clone();

        quote! {
            use #visit_target::*;
        }
    }
}

struct VisitorTraitTransformer;
impl Transformer<VisitorTransformerParams, TokenStream2>
    for VisitorTraitTransformer
{
    fn transform(
        &self,
        input: &VisitorTransformerParams,
    ) -> TokenStream2
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

struct AcceptorTransformer;
impl Transformer<VisitorTransformerParams, TokenStream2> for AcceptorTransformer
{
    fn transform(
        &self,
        input: &VisitorTransformerParams,
    ) -> TokenStream2
    {
        let elements = vec![
            AcceptorStructTransformer.transform(&input),
            AcceptorImplTransformer.transform(&input),
        ];

        VecTokenStringTransformer.transform(&elements)
    }
}

struct AcceptorStructTransformer;
impl Transformer<VisitorTransformerParams, TokenStream2>
    for AcceptorStructTransformer
{
    fn transform(
        &self,
        input: &VisitorTransformerParams,
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

struct AcceptorImplTransformer;
impl Transformer<VisitorTransformerParams, TokenStream2>
    for AcceptorImplTransformer
{
    fn transform(
        &self,
        input: &VisitorTransformerParams,
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
                    variant_ids: input.variant_ids(),
                    visitor_fn_names: input.visitor_fn_names(),
                    variants_field_names: input.variants_field_names(),
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
