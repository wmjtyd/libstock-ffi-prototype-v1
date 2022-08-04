use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use syn::spanned::Spanned;
use syn::{Attribute, Fields, Ident};

pub fn interop_derive_macro(input: syn::ItemStruct) -> TokenStream {
    // 每個 TryFrom 來源結構體的名稱。
    let source_ident = Ident::new("src", proc_macro2::Span::call_site());

    // FFI 端的 type。
    let ffi_type = input.ident;

    // Rust 端的 type，用 #[rs_type(Type)] 指定。
    let rust_type = extract_simple_ident_attribute(&input.attrs, "rs_type");

    // 拉出指定 struct 的所有 fields。
    let fields = match input.fields {
        Fields::Named(field) => {
            let fields = field.named;

            fields.into_iter().map(|field| {
                // ident 不可能為空。
                let ident = field.ident.unwrap();
                let attrs = field.attrs;

                let convert_type = get_convert_type_bit_flag(&attrs);
                let convert_type = ConvertType::from(convert_type);

                ConvertableField::new(ident, convert_type)
            })
        }
        _ => {
            abort!(
                input.fields.span(),
                "both unnamed fields and unit struct are not supported"
            )
        }
    };

    // 根據這些 fields 預先建構每個 field 的 assignment 語句。
    let assignments = fields.map(|v| {
        let ident = &v.ident;
        let convert_rvalue = v.generate_converter(&source_ident);

        quote! {
            #ident: #convert_rvalue,
        }
    });

    // 然後根據這些 assignments，建構 try_from 的 body：
    //
    // try_from 的長相基本上是固定的：
    //
    // ```ignore
    // Ok(Self {
    //    <assignments>
    //    ...
    // })
    // ```
    let body = quote! {
        Ok(Self {
            #(
                #assignments
            )*
        })
    };

    // 最後建構 Interop 的 impl。
    quote! {
        impl TryFrom<&#ffi_type> for #rust_type {
            type Error = crate::errors::LibstockErrors;

            fn try_from(#source_ident: &#ffi_type) -> Result<Self, Self::Error> {
                #body
            }
        }

        impl TryFrom<&#rust_type> for #ffi_type {
            type Error = crate::errors::LibstockErrors;

            fn try_from(#source_ident: &#rust_type) -> Result<Self, Self::Error> {
                #body
            }
        }

        impl TryFrom<#ffi_type> for #rust_type {
            type Error = crate::errors::LibstockErrors;

            fn try_from(#source_ident: #ffi_type) -> Result<Self, Self::Error> {
                #body
            }
        }

        impl TryFrom<#rust_type> for #ffi_type {
            type Error = crate::errors::LibstockErrors;

            fn try_from(#source_ident: #rust_type) -> Result<Self, Self::Error> {
                #body
            }
        }
    }
}

/// 轉換類型。
#[derive(Copy, Clone, Debug, PartialEq, Eq, AsRefStr, EnumIter)]
#[strum(serialize_all = "snake_case")]
enum ConvertType {
    None = 0,
    Into,
    TryInto,
    ConvertBox,
    Default,
}

type ConvertTypeBitFlag = u64;

/// 可以產生對應轉換語句的 Field 封裝。
struct ConvertableField {
    pub ident: Ident,
    pub convert_type: ConvertType,
}

impl ConvertableField {
    pub fn new(ident: Ident, convert_type: ConvertType) -> Self {
        Self {
            ident,
            convert_type,
        }
    }

    pub fn generate_converter(&self, source_struct: &Ident) -> TokenStream {
        let ident = &self.ident;

        match self.convert_type {
            ConvertType::None => quote! { #source_struct.#ident },
            ConvertType::Into => quote! { #source_struct.#ident.into() },
            ConvertType::TryInto => quote! { #source_struct.#ident.try_into()? },
            ConvertType::ConvertBox => quote! {{
                use crate::converter::ConvertBox;
                let ConvertBox(f) = TryFrom::try_from(ConvertBox(&#source_struct.#ident))?;
                f
            }},
            ConvertType::Default => quote! { Default::default() }
        }
    }
}

impl From<ConvertTypeBitFlag> for ConvertType {
    fn from(flag: ConvertTypeBitFlag) -> Self {
        for convert_type in ConvertType::iter() {
            let mask = 1 << (convert_type as u64);

            // 如果 mask 之後不等於 0 => convert_type match!
            if flag & mask != 0 {
                return convert_type;
            }
        }

        ConvertType::None
    }
}

/// 根據傳入的 Attribute 判斷屬於哪種 ConvertType。
fn get_convert_type_bit_flag(attrs: &[Attribute]) -> ConvertTypeBitFlag {
    let attr = attrs.iter().find_map(|attr| {
        for convert_type in ConvertType::iter() {
            if attr.path.is_ident(convert_type.as_ref()) {
                return Some(1 << (convert_type as u64));
            }
        }
        None
    });

    // None。
    attr.unwrap_or(0)
}

/// 抓出指定 attributes 裡面的基礎 ident 值型 attribute。
///
/// 基礎 ident 值型 attribute 基本上是長這樣的：
///
/// ```ignore
/// #[ident(Ident)]
/// ```
///
/// # Panics
///
/// 以上述為例，如果找不到名為 `ident` 的 attribute，
/// 或 `Ident` 不是有效 ident 則會拋出錯誤。
fn extract_simple_ident_attribute(attributes: &[Attribute], ident: &str) -> Ident {
    let t = attributes.iter().find_map(|attr| {
        if attr.path.is_ident(ident) {
            match attr.parse_args() {
                Ok(v) => Some(v),
                Err(_) => abort!(attr.path.span(), "not a valid identifier"),
            }
        } else {
            None
        }
    });

    match t {
        Some(v) => v,
        None => abort_call_site!(format!("missing #[{ident}(Ident)]")),
    }
}

// 暫時沒用到。
//
// /// 抓出指定 attributes 裡面的 boolean 型 attribute。
// ///
// /// boolean 型 attribute 基本上是長這樣的：
// ///
// /// ```ignore
// /// #[into]
// /// ```
// ///
// /// 如果找到名為 `into` 的 Attribute 則回傳 true；反之為 false。
// fn extract_boolean_attribute(attributes: &[Attribute], ident: &str) -> bool {
//     attributes.iter().any(|attr| attr.path.is_ident(ident))
// }

#[cfg(test)]
mod tests {
    use super::ConvertType;

    #[test]
    fn test_convert_type_asref() {
        assert_eq!(ConvertType::None.as_ref(), "none");
        assert_eq!(ConvertType::Into.as_ref(), "into");
        assert_eq!(ConvertType::TryInto.as_ref(), "try_into");
        assert_eq!(ConvertType::ConvertBox.as_ref(), "convert_box");
        assert_eq!(ConvertType::Default.as_ref(), "default");
    }

    #[test]
    fn test_convert_type_from_mask() {
        #![allow(clippy::identity_op)]

        assert_eq!(ConvertType::from(0), ConvertType::None);
        assert_eq!(ConvertType::from(1 << 0), ConvertType::None);
        assert_eq!(ConvertType::from(1 << 1), ConvertType::Into);
        assert_eq!(ConvertType::from(1 << 2), ConvertType::TryInto);
        assert_eq!(ConvertType::from(1 << 3), ConvertType::ConvertBox);
        assert_eq!(ConvertType::from(1 << 4), ConvertType::Default);
    }
}
