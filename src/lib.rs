extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro2;
use proc_macro2::{Ident, Span};

#[macro_use]
extern crate quote;

#[proc_macro_derive(IsAnEnum)]
pub fn is_an_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let variants = match input.data {
        syn::Data::Enum(enum_item) => enum_item.variants,
        _ => panic!("IsAnEnum only works on Enums"),
    };

    let enum_name = input.ident;

    let funcs_for_variants = variants.iter().map(|var| {
        let variant_name = &var.ident;
        let func_name_str = format!("is_a_{}", &var.ident).to_lowercase();
        let func_name = Ident::new(&func_name_str, Span::call_site());
        quote! {
            #[inline]
            pub fn #func_name(&self) -> bool { self.is_a(#enum_name::#variant_name) }
        }
    });

    let expanded = quote! {
        impl #enum_name {
            #[inline]
            pub fn is_a(&self, x: #enum_name) -> bool {
                *self == x
            }

            #(#funcs_for_variants)*
        }
    };
    expanded.into()
}
