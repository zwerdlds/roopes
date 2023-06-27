use heck::ToSnekCase;
use proc_macro2::Ident;
use quote::format_ident;
use syn::{
    punctuated::Punctuated,
    token::Comma,
    Fields,
    Type,
    Variant,
    Visibility,
};

#[derive(Clone)]
pub(super) struct TransformerParams
{
    pub(super) visibility: Visibility,
    pub(super) visit_target: Ident,
    pub(super) visitor: Ident,
    pub(super) acceptor: Ident,
    pub(super) variants: Punctuated<Variant, Comma>,
}

impl TransformerParams
{
    pub(super) fn variant_ids(&self) -> Vec<Ident>
    {
        self.variants
            .iter()
            .map(|variant| variant.clone().ident)
            .collect()
    }

    pub(super) fn visitor_fn_names(&self) -> Vec<Ident>
    {
        self.variant_ids()
            .into_iter()
            .map(|id| id.to_string().to_snek_case())
            .map(|id| format_ident!("visit_{id}"))
            .collect()
    }

    pub(super) fn variants_fields(&self) -> Vec<Fields>
    {
        self.variants
            .clone()
            .into_iter()
            .map(|variant| variant.fields)
            .collect()
    }

    pub(super) fn variants_field_params(&self) -> Vec<Vec<(Type, Ident)>>
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

    pub(super) fn variants_field_names(&self) -> Vec<Vec<Ident>>
    {
        self.variants_field_params()
            .into_iter()
            .map(|fields| fields.into_iter().map(|(_, name)| name).collect())
            .collect()
    }
}
