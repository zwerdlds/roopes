use crate::prelude::*;
use proc_macro::TokenStream;
use quote::{
    format_ident,
    quote,
};
use syn::parse_macro_input;

pub(super) struct BuilderTokenStream
{
    token_stream: TokenStream,
}

impl BuilderTokenStream
{
    pub(super) fn new_from_token_stream(token_stream: TokenStream) -> Self
    {
        Self { token_stream }
    }
}

impl Builder<TokenStream, TokenStream> for BuilderTokenStream
{
    fn build(&self) -> TokenStream
    {
        let tokens = self.token_stream.clone();

        let ast = parse_macro_input!(tokens as syn::DeriveInput);
        let build_target = ast.ident.clone();
        let builder = format_ident!("{build_target}Builder");
        let vis = ast.vis;

        quote! {
            #vis struct #builder {
                printer: Option<Rc<dyn Handler<String>>>,
                formatter: Option<Rc<dyn LogFormatter>>,
            }
            impl #builder {
                pub fn new() -> LoggerBuilder {
                    LoggerBuilder {
                        printer: Option::None,
                        formatter: Option::None,
                    }
                }

                pub fn build(&self) -> #build_target {
                    #build_target {
                        printer: self.printer.as_ref().unwrap().clone(),
                        formatter: self.formatter.as_ref().unwrap().clone(),
                    }
                }

                pub fn set_printer(
                    &mut self,
                    printer: Rc<dyn Handler<String>>
                ) {
                    self.printer = Some(printer.clone());
                }

                pub fn set_formatter(
                    &mut self,
                    formatter: Rc<dyn LogFormatter>
                ) {
                    self.formatter = Some(formatter.clone());
                }
            }
        }
        .into()
    }

    fn get_params(&self) -> &TokenStream
    {
        &self.token_stream
    }

    fn set_params(
        &mut self,
        token_stream: TokenStream,
    )
    {
        self.token_stream = token_stream;
    }
}
