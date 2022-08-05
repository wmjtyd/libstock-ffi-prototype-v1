use proc_macro2::{TokenStream, Ident};
use quote::quote;

use super::common::extract_simple_ident_attribute;

pub fn interop_enum_derive_macro(input: syn::ItemEnum) -> TokenStream {
    // FFI 端的 type。
    let ffi_type = &input.ident;

    // Rust 端的 type，用 #[rs_type(Type)] 指定。
    let rust_type = extract_simple_ident_attribute(&input.attrs, "rs_type");

    let ffi_to_rust_from = construct_from_tokens(&input, ffi_type, &rust_type);
    let rust_to_ffi_from = construct_from_tokens(&input, &rust_type, ffi_type);

    quote! {
        #ffi_to_rust_from

        #rust_to_ffi_from

        crate::utils::create_from_owned!(#ffi_type, #rust_type);
    }
}

fn construct_from_tokens(input: &syn::ItemEnum, src_type: &Ident, tgt_type: &Ident) -> TokenStream {
    let fields = input.variants.iter().map(|v| {
        let common_ident = &v.ident;

        quote! {
            #src_type::#common_ident => #tgt_type::#common_ident
        }
    });

    quote! {
        impl From<&#src_type> for #tgt_type {
            fn from(value: &#src_type) -> Self {
                match value {
                    #(#fields),*
                }
            }
        }
    }
}

