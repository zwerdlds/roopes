use heck::{
    ToShoutySnakeCase,
    ToUpperCamelCase,
};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{
    format_ident,
    quote,
};
use std::borrow::Borrow;
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

    fn field_generic_name(field_name: &Ident) -> Ident
    {
        let name = field_name.to_string().to_shouty_snake_case();
        format_ident!("IS_{name}_SET")
    }

    fn field_struct_base_name(field_name: &Ident) -> Ident
    {
        let gn = field_name.to_string().to_upper_camel_case();
        format_ident!("{gn}")
    }

    fn field_populated_struct_name(field_name: &Ident) -> Ident
    {
        let gn = Self::field_struct_base_name(field_name);
        format_ident!("{gn}Set")
    }

    fn field_unpopulated_struct_name(field_name: &Ident) -> Ident
    {
        let gn = Self::field_struct_base_name(field_name);
        format_ident!("{gn}Unset")
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

        let build_set_fields = fields.iter().map(|field| {
            let id = field.clone().ident.unwrap();

            quote! { #id: self.#id.as_ref().unwrap().clone() }
        });

        let field_idents = fields.iter().map(|f| f.ident).map(Option::unwrap);

        let populated_indicator_generics =
            field_idents.clone().map(|f| Self::field_generic_name(&f));

        let field_populated_struct_names = populated_indicator_generics
            .clone()
            .map(|f| Self::field_populated_struct_name(&f));

        let builder_poppable = format_ident!("{builder}Settable");
        let builder_unpopped = format_ident!("{builder}Unset");
        let builder_popped = format_ident!("{builder}Set");

        let field_indicator_structs = {
            let field_structs = field_idents.map(|f| {
                let unpop = Self::field_unpopulated_struct_name(&f);
                let popped = Self::field_populated_struct_name(&f);

                quote! {
                    #vis struct #unpop {}
                    impl #builder_unpopped for #unpop {}
                    impl #builder_poppable for #unpop {}

                    #vis struct #popped {}
                    impl #builder_popped for #popped {}
                    impl #builder_poppable for #popped {}
                }
            });

            quote! {
                #(#field_structs)*
            }
        };

        let all_unpopulated = fields
            .iter()
            .map(|f| Self::field_unpopulated_struct_name(f));

        let all_populated =
            fields.iter().map(|f| Self::field_populated_struct_name(&f));

        let where_block = {
            let where_clauses = field_indicator_generics
                .clone()
                .map(|(pop_generic, fpe_name)| quote!(#pop_generic : #builder_poppable));

            quote!(where #(#where_clauses),*)
        };

        let setter_impls = fields.clone().into_iter()
            .map(|field| {
                let field = field.clone();
                let id = field.ident.unwrap();
                let ty = field.ty;
                let setter_id = format_ident!("set_{id}");
                let impl_setter_return_generics = fields.clone()
                    .into_iter()
                    .map(|f| f.ident)
                    .map(Option::unwrap)
                    .map(|gen_fld|{
                        if gen_fld.eq(field) {
                            quote!{

                            }
                        }
                        else {
                            Self::field_generic_name(fld) 
                        }
                    });

                quote! {
                    pub fn #setter_id(self, #id: #ty) -> Self<#(#impl_setter_return_generics),*>
                    {
                        self.#id = Some(#id);
                        self
                    }
                }
            });

        let output = quote! {
            #vis trait #builder_settable {}
            #vis trait #builder_unset {}
            #vis trait #builder_set {}

            #field_indicator_structs

            #vis struct #builder<#(#populated_indicator_generics),*> #where_block  {
                #(#fields_declare),*
            }

            impl<#(#field_populated_enum_names),*> #builder<#(#populated_indicator_generics),*>  {
                pub fn new() -> #builder<#(#all_unpopulated),*> {
                    #builder {
                        #(#fields_init),*
                    }
                }

                #setters_impl
            }


            impl #builder<#(#all_populated),*> {
                pub fn build(&self) -> #build_target {
                    #build_target {
                        #(#build_build),*
                    }
                }
            }
        };

        eprintln!("Results:");
        let contents =
            syn::parse_file(&output.to_string()).expect("Syn parsing failed.");
        let formatted = prettyplease::unparse(&contents);
        eprintln!("{formatted}");

        output.into()
    }
}
