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
        format_ident!("IS_FIELD_{name}_SET")
    }

    fn phantom_field_name(field_name: &Ident) -> Ident
    {
        format_ident!("{field_name}_set")
    }

    fn field_struct_base_name(field_name: &Ident) -> Ident
    {
        let gn = field_name.to_string().to_upper_camel_case();
        format_ident!("{gn}")
    }

    fn field_populated_struct_name(field_name: &Ident) -> Ident
    {
        let gn = Self::field_struct_base_name(field_name);
        format_ident!("Field{gn}Set")
    }

    fn field_unpopulated_struct_name(field_name: &Ident) -> Ident
    {
        let gn = Self::field_struct_base_name(field_name);
        format_ident!("Field{gn}Unset")
    }

    #[allow(clippy::too_many_lines)]
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

        let build_fields_declare = fields.iter().map(|field| {
            let field = field.clone();
            let id = field.ident.unwrap();
            let ty = field.ty;

            quote! {
                #id: std::option::Option<#ty>
            }
        });

        let field_idents: Vec<_> = fields
            .clone()
            .into_iter()
            .map(|f| f.ident)
            .map(Option::unwrap)
            .collect();

        let phantom_fields: Vec<_> =
            field_idents.iter().map(Self::phantom_field_name).collect();

        let phantom_fields_declare = field_idents.iter().map(|id| {
            let fgn = Self::field_generic_name(id);
            let phantom_field = Self::phantom_field_name(id);

            quote! {
                #phantom_field: std::marker::PhantomData<#fgn>
            }
        });

        let build_fields_init = field_idents.iter().map(|id| {
            quote! { #id: std::option::Option::None }
        });

        let phantom_fields_init = phantom_fields.iter().map(|id| {
            quote! { #id: std::marker::PhantomData {} }
        });

        let build_set_fields = field_idents.iter().map(|id| {
            quote! { #id: self.#id.unwrap() }
        });

        let poppable_indicator_generics =
            field_idents.iter().map(Self::field_generic_name);

        let builder_poppable = format_ident!("{builder}SettableField");
        let builder_unpopped = format_ident!("{builder}FieldUnset");
        let builder_popped = format_ident!("{builder}FieldSet");

        let field_indicator_structs = {
            let field_structs = field_idents.iter().map(|f| {
                let unpop = Self::field_unpopulated_struct_name(f);
                let popped = Self::field_populated_struct_name(f);

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

        let all_unpopulated: Vec<_> = field_idents
            .iter()
            .map(Self::field_unpopulated_struct_name)
            .collect();

        let all_populated =
            field_idents.iter().map(Self::field_populated_struct_name);

        let field_generic_names: Vec<_> =
            field_idents.iter().map(Self::field_generic_name).collect();

        let poppable_where_block = {
            let where_clauses = field_generic_names
                .iter()
                .map(|pop_generic| quote!(#pop_generic : #builder_poppable));

            quote!(where #(#where_clauses),*)
        };

        let setter_fns = fields.clone().into_iter().map(|field| {
            let id = field.ident.unwrap();
            let ty = field.ty;
            let fn_id = format_ident!("set_{id}");
            let return_generics = field_idents
                .iter()
                .map(|gen_fld| {
                    if gen_fld.eq(&id) {
                        Self::field_populated_struct_name(&id)
                    } else {
                        Self::field_generic_name(gen_fld)
                    }
                })
                .map(|f| quote! {#f});

            let remainder_ids = field_idents
                .iter()
                .filter(|fid| fid.ne(&&id))
                .cloned()
                .map(|fid| {
                    quote! {
                        #fid: self.#fid
                    }
                })
                .chain(phantom_fields.iter().cloned().map(
                    |phantom_fid: Ident| {
                        quote! {
                            #phantom_fid: std::marker::PhantomData {}
                        }
                    },
                ));

            quote! {
                #vis fn
                    #fn_id(self, #id: #ty)
                    -> #builder<#(#return_generics),*>
                {
                    #builder {
                        #id: Some(#id),
                        #(#remainder_ids),*
                    }
                }
            }
        });

        let fields_init = build_fields_init.chain(phantom_fields_init);
        let fields_declare = build_fields_declare.chain(phantom_fields_declare);

        let builder_generic_ty = quote! {
            #builder<#(#poppable_indicator_generics),*>
        };

        let new_impl_type = {
            quote! {
            #builder<#(#all_unpopulated),*>}
        };

        let new_fn = quote! {
            #vis fn new()
                    -> #builder<#(#all_unpopulated),*>
                {
                    #builder {
                        #(#fields_init),*
                    }
                }
        };

        let target_new_bulider_fn = quote! {
            #vis fn builder()
                    -> #builder<#(#all_unpopulated),*>
                {
                    #builder::new()
                }
        };

        let output = quote! {
            #vis trait #builder_poppable {}
            #vis trait #builder_unpopped {}
            #vis trait #builder_popped {}

            #field_indicator_structs

            #vis struct
                #builder_generic_ty
                #poppable_where_block
            {
                #(#fields_declare),*
            }

            impl #new_impl_type
            {
                #new_fn
            }

            impl #build_target
            {
                #target_new_bulider_fn
            }

            impl<#(#field_generic_names),*>
                #builder_generic_ty
                #poppable_where_block
            {
                #(#setter_fns)*
            }


            impl #builder<#(#all_populated),*> {
                #vis fn build(self) -> #build_target {
                    #build_target {
                        #(#build_set_fields),*
                    }
                }
            }
        };

        // eprintln!("{}", output.to_string());
        // eprintln!("Formatted Results:");
        // let contents = syn::parse_file(&output.to_string()).expect(
        //     "Syn parsing
        // failed",
        // );
        // let formatted = prettyplease::unparse(&contents);
        // eprintln!("{formatted}");

        output.into()
    }
}
