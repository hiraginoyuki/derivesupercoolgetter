use proc_macro::TokenStream;

mod derive_impl;

#[allow(non_snake_case)]
#[proc_macro_derive(SuperCoolGetter)]
pub fn SuperCoolGetter(input: TokenStream) -> TokenStream {
    derive_impl::derive_impl(input.into()).into()
}
