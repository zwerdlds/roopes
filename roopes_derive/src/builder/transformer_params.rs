use derive_getters::Getters;
use heck::{
    ToShoutySnakeCase,
    ToUpperCamelCase,
};
use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::{
    punctuated::Punctuated,
    token::Comma,
    Field,
    Visibility,
};

#[derive(Clone, Getters)]
pub(super) struct TransformerParams
{
    pub(super) visibility: Visibility,
    pub(super) target: Ident,
    pub(super) builder: Ident,
    pub(super) fields: Punctuated<Field, Comma>,
}

impl TransformerParams
{
    pub(super) fn phantom_fields(&self) -> Vec<Ident>
    {
        self.field_idents()
            .iter()
            .map(TransformerParams::phantom_field_name)
            .collect()
    }

    pub(super) fn builder_generic_ty(&self) -> TokenStream
    {
        let builder = self.builder();
        let poppable_indicator_generics = self
            .field_idents()
            .into_iter()
            .map(|fi| TransformerParams::field_generic_name(&fi));

        quote! {
            #builder<#(#poppable_indicator_generics),*>
        }
    }

    pub(super) fn all_unpopulated(&self) -> Vec<Ident>
    {
        self.field_idents()
            .iter()
            .map(Self::field_unpopulated_struct_name)
            .collect()
    }

    pub(super) fn all_populated(&self) -> Vec<Ident>
    {
        self.field_idents()
            .iter()
            .map(Self::field_populated_struct_name)
            .collect()
    }

    pub(super) fn field_idents(&self) -> Vec<Ident>
    {
        self.fields()
            .into_iter()
            .map(|f| {
                f.ident.clone().expect(
                    "Builder only works on structs with fields with names.",
                )
            })
            .collect()
    }

    pub(super) fn field_generic_name(field_name: &Ident) -> Ident
    {
        let name = field_name.to_string().to_shouty_snake_case();
        format_ident!("IS_FIELD_{name}_SET")
    }

    pub(super) fn phantom_field_name(field_name: &Ident) -> Ident
    {
        format_ident!("{field_name}_set")
    }

    pub(super) fn field_struct_base_name(field_name: &Ident) -> Ident
    {
        let gn = field_name.to_string().to_upper_camel_case();
        format_ident!("{gn}")
    }

    pub(super) fn field_populated_struct_name(field_name: &Ident) -> Ident
    {
        let gn = Self::field_struct_base_name(field_name);
        format_ident!("Field{gn}Set")
    }

    pub(super) fn field_unpopulated_struct_name(field_name: &Ident) -> Ident
    {
        let gn = Self::field_struct_base_name(field_name);
        format_ident!("Field{gn}Unset")
    }

    pub(super) fn builder_poppable(&self) -> Ident
    {
        let builder = self.builder();
        format_ident!("{builder}SettableField")
    }

    pub(super) fn builder_unpopped(&self) -> Ident
    {
        let builder = self.builder();
        format_ident!("{builder}FieldUnset")
    }

    pub(super) fn builder_popped(&self) -> Ident
    {
        let builder = self.builder();
        format_ident!("{builder}FieldSet")
    }

    pub(super) fn field_generic_names(&self) -> Vec<Ident>
    {
        self.field_idents()
            .iter()
            .map(TransformerParams::field_generic_name)
            .collect()
    }

    pub(super) fn poppable_where_block(&self) -> TokenStream
    {
        let builder_poppable = self.builder_poppable();

        let where_clauses = self
            .field_generic_names()
            .into_iter()
            .map(|pop_generic| quote!(#pop_generic : #builder_poppable));

        quote!(where #(#where_clauses),*)
    }
}
