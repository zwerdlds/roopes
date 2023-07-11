use super::transformer_params::TransformerParams;
use crate::common::VecTokenStringTransformer;
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use roopes_core::prelude::*;
use syn::{
    Field,
    Ident,
};

pub(super) struct FieldIndicatorSection;
impl Transformer<TransformerParams, TokenStream> for FieldIndicatorSection
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let visibility = input.visibility().clone();
        let builder_poppable = input.builder_poppable();
        let builder_unpopped = input.builder_unpopped();
        let builder_popped = input.builder_popped();

        let field_indicator_structs = {
            let field_indicator_structs = input
                .field_idents()
                .iter()
                .map(|f| (f, input))
                .map(|e| FieldIndicatorStruct.transform(&e))
                .collect();

            VecTokenStringTransformer.transform(&field_indicator_structs)
        };

        quote! {
            #visibility trait #builder_poppable {}
            #visibility trait #builder_unpopped {}
            #visibility trait #builder_popped {}

            #field_indicator_structs
        }
    }
}

struct FieldIndicatorStruct;
impl Transformer<(&Ident, &TransformerParams), TokenStream>
    for FieldIndicatorStruct
{
    fn transform(
        &self,
        (field_ident, input): &(&Ident, &TransformerParams),
    ) -> TokenStream
    {
        let visibility = input.visibility().clone();
        let unpop =
            TransformerParams::field_unpopulated_struct_name(field_ident);
        let popped =
            TransformerParams::field_populated_struct_name(field_ident);
        let builder_unpopped = input.builder_unpopped();
        let builder_popped = input.builder_popped();
        let builder_poppable = input.builder_poppable();

        quote! {
            #visibility struct #unpop {}
            impl #builder_unpopped for #unpop {}
            impl #builder_poppable for #unpop {}

            #visibility struct #popped {}
            impl #builder_popped for #popped {}
            impl #builder_poppable for #popped {}
        }
    }
}

pub(super) struct Builder;
impl Transformer<TransformerParams, TokenStream> for Builder
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let visibility = input.visibility().clone();
        let poppable_where_block = input.poppable_where_block();
        let builder_generic_ty = input.builder_generic_ty();
        let builder_fields = BuilderFieldsDeclarations.transform(input);

        quote! {
            #visibility struct
                #builder_generic_ty
                #poppable_where_block
            {
                #builder_fields
            }
        }
    }
}

pub(super) struct BuilderFieldsDeclarations;
impl Transformer<TransformerParams, TokenStream> for BuilderFieldsDeclarations
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let phantom_fields = input.field_idents().into_iter().map(|id| {
            let fgn = TransformerParams::field_generic_name(&id);
            let phantom_field = TransformerParams::phantom_field_name(&id);

            quote! {
                #phantom_field: std::marker::PhantomData<#fgn>
            }
        });

        let builder_fields = input.fields().iter().map(|field| {
            let field = field.clone();
            let id = field.ident.unwrap();
            let ty = field.ty;

            quote! {
                #id: std::option::Option<#ty>
            }
        });

        let fields_declare = builder_fields.chain(phantom_fields);

        quote! {
            #(#fields_declare),*
        }
    }
}

pub(super) struct NewImpl;
impl Transformer<TransformerParams, TokenStream> for NewImpl
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let visibility = input.visibility().clone();
        let builder = input.builder();
        let all_unpopulated = input.all_unpopulated();

        let build_fields_init = input.field_idents().into_iter().map(|id| {
            quote! { #id: std::option::Option::None }
        });

        let phantom_fields_init =
            input.phantom_fields().into_iter().map(|id| {
                quote! { #id: std::marker::PhantomData {} }
            });

        let fields_init = build_fields_init.chain(phantom_fields_init);

        let new_impl_type = {
            quote! {
            #builder<#(#all_unpopulated),*>}
        };

        let new_fn = quote! {
            #visibility fn new()
                    -> #builder<#(#all_unpopulated),*>
                {
                    #builder {
                        #(#fields_init),*
                    }
                }
        };

        quote! {
            impl #new_impl_type
            {
                #new_fn
            }
        }
    }
}

pub(super) struct TargetImpl;
impl Transformer<TransformerParams, TokenStream> for TargetImpl
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let visibility = input.visibility().clone();
        let all_unpopulated = input.all_unpopulated();
        let builder = input.builder();
        let target = input.target();

        let target_new_builder_fn = quote! {
            #visibility fn builder()
                    -> #builder<#(#all_unpopulated),*>
                {
                    #builder::new()
                }
        };

        quote! {
            impl #target
            {
                #target_new_builder_fn
            }
        }
    }
}

pub(super) struct SettersImpl;
impl Transformer<TransformerParams, TokenStream> for SettersImpl
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let field_generic_names = input.field_generic_names();
        let builder_generic_ty = input.builder_generic_ty();
        let poppable_where_block = input.poppable_where_block();

        let setter_fns = input
            .fields()
            .clone()
            .into_iter()
            .map(|e| SetterFn.transform(&(&e, input)));

        quote! {
                impl<#(#field_generic_names),*>
                    #builder_generic_ty
                    #poppable_where_block
                {
                    #(#setter_fns)*
                }
        }
    }
}

pub(super) struct SetterFn;
impl Transformer<(&Field, &TransformerParams), TokenStream> for SetterFn
{
    fn transform(
        &self,
        (field, input): &(&Field, &TransformerParams),
    ) -> TokenStream
    {
        let visibility = input.visibility().clone();
        let field_idents = input.field_idents();
        let id = field.ident.clone().unwrap();
        let ty = field.ty.clone();
        let fn_id = format_ident!("set_{id}");
        let builder = input.builder();
        let phantom_fields: Vec<_> = input.phantom_fields();

        let return_generics = field_idents
            .iter()
            .map(|gen_fld| {
                if gen_fld.eq(&id) {
                    TransformerParams::field_populated_struct_name(&id)
                } else {
                    TransformerParams::field_generic_name(gen_fld)
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
            .chain(phantom_fields.iter().cloned().map(|phantom_fid: Ident| {
                quote! {
                    #phantom_fid: std::marker::PhantomData {}
                }
            }));

        quote! {
            #visibility fn
                #fn_id(self, #id: #ty)
                -> #builder<#(#return_generics),*>
            {
                #builder {
                    #id: Some(#id),
                    #(#remainder_ids),*
                }
            }
        }
    }
}

pub(super) struct BuildImpl;
impl Transformer<TransformerParams, TokenStream> for BuildImpl
{
    fn transform(
        &self,
        input: &TransformerParams,
    ) -> TokenStream
    {
        let builder = input.builder();
        let all_populated_generics = input.all_populated();
        let visibility = input.visibility().clone();
        let target = input.target();
        let build_set_fields = input.field_idents().into_iter().map(|id| {
            quote! { #id: self.#id.unwrap() }
        });

        quote! {
            impl #builder<#(#all_populated_generics),*> {
                #visibility fn build(self) -> #target {
                    #target {
                        #(#build_set_fields),*
                    }
                }
            }
        }
    }
}
