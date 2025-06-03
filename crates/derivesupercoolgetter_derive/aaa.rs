use convert_case::{Case::Pascal, Casing};
use guard_macros::guard;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{LitStr, spanned::Spanned};

pub fn derive_super_cool_getter_impl(input: TokenStream) -> TokenStream {
    guard! {
        Ok(input) = syn::parse2::<syn::DeriveInput>(input),

        syn::Data::Struct(sutorakuto) = input.data => panic!("nooo not a structstruct"),
        syn::Fields::Named(fields) = sutorakuto.fields => panic!("nooo give me named fields bro 💔")
    }

    let struct_name = &input.ident;
    let fields_pascal_case = fields.named.iter().map(|field| {
        let ident = field.ident.clone().unwrap();
        let pascal_ident = format_ident!("{}", ident.to_string().to_case(Pascal));
        (field, pascal_ident)
    });

    let enum_ref_name = format_ident!("{}Ref", struct_name);
    let enum_mut_name = format_ident!("{}Mut", struct_name);

    let enum_ref_items = fields_pascal_case.clone().map(|(field, pascal_ident)| {
        let ty = &field.ty;
        quote! {
            #pascal_ident(&'a #ty)
        }
    });
    let enum_mut_items = fields_pascal_case.clone().map(|(field, pascal_ident)| {
        let ty = &field.ty;
        quote! {
            #pascal_ident(&'a mut #ty)
        }
    });
    let enum_bodies = quote! {
        pub enum #enum_ref_name<'a> {
            #(#enum_ref_items),*
        }
        pub enum #enum_mut_name<'a> {
            #(#enum_mut_items),*
        }
    };

    let fields3 = fields_pascal_case.map(|(field, pascal_ident)| {
        let field_lit = LitStr::new(
            &field.ident.clone().unwrap().to_string(),
            field.ident.span(),
        );
        let field_ident = &field.ident;
        (pascal_ident, field_lit, field_ident)
    });
    let get_match_arms = fields3
        .clone()
        .map(|( pascal_ident, field_lit, field_ident)| {
            quote! {
                #field_lit => ::core::option::Option::Some(#enum_ref_name::#pascal_ident(&self.#field_ident))
            }
        });
    let get_mut_match_arms =
        fields3
            .clone()
            .map(|( pascal_ident, field_lit, field_ident)| {
                quote! {
                        #field_lit => ::core::option::Option::Some(#enum_mut_name::#pascal_ident(&mut self.#field_ident))
                }
            });

    let get_as_match_arms = fields3.clone().map(|(_, field_lit, field_ident)| {
        quote! {
            #field_lit => (&self.#field_ident as &dyn ::core::any::Any).downcast_ref()
        }
    });
    let get_as_mut_match_arms = fields3.clone().map(|(_, field_lit, field_ident)| {
        quote! {
            #field_lit => (&mut self.#field_ident as &mut dyn ::core::any::Any).downcast_mut()
        }
    });

    let trait_impl = quote! {
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
                    #(#get_as_mut_match_arms ,)*
                    _ => None,
                }
            }
        }
    };

    quote! {
        #enum_bodies
        #trait_impl
    }
}
