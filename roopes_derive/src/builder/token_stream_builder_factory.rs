use proc_macro::TokenStream;
use quote::{
    format_ident,
    quote,
};
use syn::parse_macro_input;

pub(super) struct TokenStreamBuilderFactory
{
    token_stream: TokenStream,
}

impl TokenStreamBuilderFactory
{
    pub(super) fn new_from_token_stream(token_stream: TokenStream) -> Self
    {
        Self { token_stream }
    }

    pub fn build(&self) -> TokenStream
    {
        let tokens = self.token_stream.clone();

        let ast = parse_macro_input!(tokens as syn::DeriveInput);
        let build_target = ast.ident.clone();
        let builder = format_ident!("{build_target}Builder");
        let vis = ast.vis.clone();

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

        let fields_declare = fields.iter().map(|field| {
            let field = field.clone();
            let id = field.ident.unwrap();
            let ty = field.ty;

            quote! {
                #id: std::option::Option<#ty>
            }
        });

        let fields_init = fields.iter().map(|field| {
            let id = field.clone().ident.unwrap();
            quote! { #id: std::option::Option::None }
        });

        let build_build = fields.iter().map(|field| {
            let id = field.clone().ident.unwrap();

            quote! { #id: self.#id.as_ref().unwrap().clone() }
        });

        let setters = fields.iter().map(|field| {
            let field = field.clone();
            let id = field.ident.unwrap();
            let ty = field.ty;

            let setter_id = format_ident!("set_{id}");

            quote! { pub fn #setter_id(&mut self, #id: #ty) -> &mut Self
                {
                    self.#id = Some(#id);
                    self
                }
            }
        });

        let output = quote! {
            #vis struct #builder {
                #(#fields_declare),*
            }
            impl #builder {
                pub fn new() -> #builder {
                    #builder {
                        #(#fields_init),*
                    }
                }

                pub fn build(&self) -> #build_target {
                    #build_target {
                        #(#build_build),*
                    }
                }

                #(#setters)*
            }
        };

        // eprintln!("Results:");
        // let contents =
        //     syn::parse_file(&result.
        // to_string()).expect("Syn parsing
        // failed."); let formatted =
        // prettyplease::unparse(&contents);
        // eprintln!("{formatted}");

        output.into()
    }
}
