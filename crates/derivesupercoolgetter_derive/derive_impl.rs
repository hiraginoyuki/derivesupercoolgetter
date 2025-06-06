use convert_case::{Case::Pascal, Casing};
use guard_macros::guard;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{LitStr, spanned::Spanned};

pub fn derive_impl(input: TokenStream) -> TokenStream {
    guard! {
        Ok(input) = syn::parse2::<syn::DeriveInput>(input),

        syn::Data::Struct(sutorakuto) = input.data => panic!("nooo not a structstruct"),
        syn::Fields::Named(fields) = sutorakuto.fields => panic!("nooo give me named fields bro 💔")
    }

    let struct_name = &input.ident;
    let enum_ref_name = format_ident!("{}Ref", struct_name);
    let enum_mut_name = format_ident!("{}Mut", struct_name);

    let (
        enum_ref_items,
        enum_mut_items,
        get_match_arms,
        get_mut_match_arms,
        get_as_match_arms,
        get_mut_as_match_arms,
    ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = fields.named.into_iter().map(|field| {
        let ident = field.ident.as_ref().unwrap().clone();
        let ident_pascal = format_ident!("{}", ident.to_string().to_case(Pascal));
        let lit = LitStr::new(&ident.to_string(), field.ident.span());
        let ty = field.ty.clone();
        (
            quote!( #ident_pascal(&'a #ty) ),
            quote!( #ident_pascal(&'a mut #ty) ),
            quote!( #lit => ::core::option::Option::Some(#enum_ref_name::#ident_pascal(&self.#ident)) ),
            quote!( #lit => ::core::option::Option::Some(#enum_mut_name::#ident_pascal(&mut self.#ident)) ),
            quote!( #lit => (&self.#ident as &dyn ::core::any::Any).downcast_ref() ),
            quote!( #lit => (&mut self.#ident as &mut dyn ::core::any::Any).downcast_mut() ),
        )
    }).multiunzip();

    quote! {
        pub enum #enum_ref_name<'a> {
            #(#enum_ref_items),*
        }
        pub enum #enum_mut_name<'a> {
            #(#enum_mut_items),*
        }

        impl<'a> SuperCoolGetter<'a> for #struct_name {
            type FieldRef = #enum_ref_name<'a>;
            type FieldMut = #enum_mut_name<'a>;

            fn get(&'a self, field_name: &str) -> Option<Self::FieldRef> {
                match field_name {
                    #(#get_match_arms ,)*
                    _ => None,
                }
            }
            fn get_mut(&'a mut self, field_name: &str) -> Option<Self::FieldMut> {
                match field_name {
                    #(#get_mut_match_arms ,)*
                    _ => None,
                }
            }

            fn get_as<T: 'static>(&'a self, field_name: &str) -> Option<&'a T> {
                match field_name {
                    #(#get_as_match_arms ,)*
                    _ => None,
                }
            }
            fn get_mut_as<T: 'static>(&'a mut self, field_name: &str) -> Option<&'a mut T> {
                match field_name {
                    #(#get_mut_as_match_arms ,)*
                    _ => None,
                }
            }
        }
    }
    .into()
}
