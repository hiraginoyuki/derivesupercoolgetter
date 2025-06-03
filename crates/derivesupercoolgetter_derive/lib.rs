use proc_macro::TokenStream;

mod aaa;

#[allow(non_snake_case)]
#[proc_macro_derive(SuperCoolGetter)]
pub fn SuperCoolGetter(input: TokenStream) -> TokenStream {
    aaa::derive_super_cool_getter_impl(input.into()).into()
}
