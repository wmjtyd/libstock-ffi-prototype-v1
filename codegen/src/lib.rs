mod serializer;
mod converter;

use proc_macro_error::{proc_macro_error, abort, abort_call_site};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, Ident, spanned::Spanned};

/// 產生對應類型的 `libstock` 的 C FFI 序列化函數。
///
/// 使用範例如下：
///
/// ```ignore
/// serializer_function!(PriceDataField -> RPriceDataField);
/// ```
///
/// 根據上方的範例，產生出的 API 是這樣的：
///
/// ```ignore
/// #[ffi_export]
/// pub fn serialize_price_data_field<'a>(
///   input: &PriceDataField,
///   buf: &'a mut c_slice::Mut<'a, u8>,
///   written_len: &mut usize,
/// ) -> LibstockErrors {
///   result_to_status_code(
///     inner_serialize_field::<RPriceDataField, _>(input, buf, written_len)
///   )
/// }
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn serializer_function(item: TokenStream) -> TokenStream {
    serializer::inner_serializer_function(item.into()).into()
}

#[proc_macro_derive(Interop, attributes(rs_type))]
#[proc_macro_error]
pub fn try_from_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    // FFI 端的 type。
    let ffi_type = input.ident;

    // Rust 端的 type，用 #[rs_type(Type)] 指定。
    let rust_type = {
        let t = input.attrs.iter().find_map(|attr| {
            if attr.path.is_ident("rs_type") {
                match attr.parse_args::<Ident>() {
                    Ok(v) => Some(v),
                    Err(_) => abort!(attr.path.span(), "not a valid identifier"),
                }
            } else {
                None
            }
        });

        match t {
            Some(v) => v,
            None => abort_call_site!("missing #[rs_type(Ident)]"),
        }
    };

    // 拉出指定 struct 的所有 fields。
    let fields = match input.fields {
        Fields::Named(field) => {
            field.named
                .iter()
                .map(|field| {
                    match field.ident.clone() {
                        Some(ident) => ident,
                        None => abort!(field.span(), "failed to extract ident from field."),
                    }
                })
                .collect::<Vec<Ident>>()
        },
        _ => {
            abort!(input.fields.span(), "unnamed fields or unit struct are not supported")
        },
    };

    // 然後根據這些 fields，建構 try_from 的 body：
    //
    // try_from 的長相基本上是固定的：
    // 
    // ```ignore
    // Ok(Self {
    //    <field>: CovertBox(&value.<field>).try_into()?,
    //    ...
    // })
    // ```
    let body = quote! {
        Ok(Self {
            #(
                #fields: crate::converter::ConvertBox(&value.#fields).try_into()?,
            )*
        })
    };

    // 最後建構 Interop 的 impl。
    quote! {
        impl TryFrom<&#ffi_type> for #rust_type {
            type Error = crate::errors::LibstockErrors;
        
            fn try_from(value: &#ffi_type) -> Result<Self, Self::Error> {
                #body
            }
        }
        
        impl TryFrom<&#rust_type> for #ffi_type {
            type Error = LibstockErrors;
        
            fn try_from(value: &#rust_type) -> Result<Self, Self::Error> {
                #body
            }
        }
    }.into()
}
