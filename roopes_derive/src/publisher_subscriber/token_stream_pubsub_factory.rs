use proc_macro::TokenStream;
use quote::{
    format_ident,
    quote,
};
use syn::parse_macro_input;

pub(super) struct TokenStreamPubSubFactory
{
    token_stream: TokenStream,
}

impl TokenStreamPubSubFactory
{
    pub(super) fn new_from_token_stream(token_stream: TokenStream) -> Self
    {
        Self { token_stream }
    }

    #[allow(clippy::too_many_lines)]
    pub fn build(&self) -> TokenStream
    {
        let tokens = self.token_stream.clone();

        let ast = parse_macro_input!(tokens as syn::DeriveInput);
        let build_target = ast.ident;
        let vis = ast.vis;
        let publisher = format_ident!("{build_target}Publisher");
        let subscriber = format_ident!("{build_target}Subscriber");
        let rps_ps = quote! {roopes::patterns::publisher_subscriber};
        let lambda_ns = quote! { roopes::primitives::handler::lambda };
        let handler_type = quote! { roopes::primitives::handler::Lambda<_, _> };
        let ra_sh = quote! { roopes::aggregates::subscribing_handler };
        let sub_handler_type = quote! { #ra_sh::SubscribingHandler<_, _> };
        let r_sub_type = quote! { #rps_ps::Subscriber<#build_target> };
        let att_pub_type =
            quote! { #rps_ps::AttachablePublisher<#build_target, #subscriber> };
        let inner_publisher_type = quote! { Box<dyn #att_pub_type> };
        let inner_subscriber_type = quote! { Box<dyn #r_sub_type> };

        let output = quote! {
            #vis struct #publisher {
                publisher: #inner_publisher_type,
            }

            impl #publisher {
                #vis fn new(publisher: #inner_publisher_type) -> Self {
                    Self { publisher }
                }
            }

            impl Default for #publisher {
                fn default() -> Self {
                    let publisher = {
                        let publisher = #rps_ps::VecPublisher::default();

                        Box::new(publisher)
                    };

                    #publisher::new(publisher)
                }
            }

            impl #rps_ps::Publisher<#build_target> for #publisher {
                fn publish(&self, event: &#build_target) {
                    self.publisher.publish(event);
                }
            }

            impl #att_pub_type for #publisher {
                fn attach(&mut self, subscriber: #subscriber){
                    self.publisher.attach(subscriber);
                }
            }

            #vis struct #subscriber {
                subscriber: #inner_subscriber_type,
            }

            impl #subscriber {
                #vis fn new<D>(delegate: D) -> Self
                where
                    D: #lambda_ns::Delegate<#build_target> + 'static,
                {
                    let handler: #handler_type = delegate.into();
                    let sub_handler: #sub_handler_type = handler.into();
                    let subscriber = Box::new(sub_handler);

                    Self { subscriber }
                }
            }

            impl #r_sub_type for #subscriber {
                fn receive(&self, event: &#build_target){
                    self.subscriber.receive(event);
                }
            }
        };

        eprintln!("{}", output.to_string());
        eprintln!("Formatted Results:");
        let contents = syn::parse_file(&output.to_string()).expect(
            "Syn parsing
        failed",
        );
        let formatted = prettyplease::unparse(&contents);
        eprintln!("{formatted}");

        output.into()
    }
}
