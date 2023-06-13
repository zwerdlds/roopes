use heck::ToSnekCase;
use proc_macro::TokenStream;
use quote::{
    format_ident,
    quote,
};
use syn::{
    parse_macro_input,
    DataEnum,
};

pub(super) struct TokenStreamVisitorFactory
{
    token_stream: TokenStream,
}

impl TokenStreamVisitorFactory
{
    pub(super) fn new_from_token_stream(token_stream: TokenStream) -> Self
    {
        Self { token_stream }
    }

    pub fn build(&self) -> TokenStream
    {
        let tokens = self.token_stream.clone();

        let ast = parse_macro_input!(tokens as syn::DeriveInput);
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

        let variant_ids = variants.iter().map(|variant| variant.clone().ident);

        let variants_fields = variants
            .iter()
            .map(|variant| variant.fields.clone().into_iter());

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

        let visitor_fn_names = variant_ids
            .clone()
            .map(|id| id.to_string().to_snek_case())
            .map(|id| format_ident!("visit_{id}"));

        let visitor_fn_calls = visitor_fn_names
            .clone()
            .zip(variants_field_names.clone())
            .map(|(name, args)| {
                quote! {
                    self.delegate.#name ( #(#args),* )
                }
            });

        let match_branches = variant_ids
            .clone()
            .zip(variants_field_names)
            .zip(visitor_fn_calls)
            .map(|((id, arg_names), func_call)| {
                let destructure_fields = {
                    if arg_names.clone().count() > 0 {
                        quote! {
                            { #(#arg_names),* }
                        }
                    } else {
                        quote! {}
                    }
                };

                quote! {
                    #id #destructure_fields => { #func_call; }
                }
            });

        let visitor_fn_sig_params = variants_field_params
            .map(|params| params.map(|(ty, name)| quote! {#name: &#ty}));

        let visitor_function_signatures = visitor_fn_names
            .zip(visitor_fn_sig_params)
            .map(|(fn_name, fn_params)| {
                quote! { fn #fn_name(
                    &self,
                    #(#fn_params),*
                ) }
            });

        let output = quote! {
            use #visit_target::*;

            #visibility trait #visitor {
                #(#visitor_function_signatures);*;
            }

            #visibility struct #acceptor<V>
                where V: #visitor
            {
                delegate: V
            }

            impl<V> #acceptor<V> where V: #visitor {
                fn new(delegate: V) -> #acceptor<V>
                {
                    #acceptor {
                        delegate
                    }
                }

                #visibility fn accept(&self, e: &#visit_target) {
                    match e {
                        #(#match_branches),*
                    }
                }
            }
        };

        eprintln!("{}", output.to_string());

        output.into()
    }
}
