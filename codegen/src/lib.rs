mod alloc;
mod interop;
mod serializer;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

/// 產生對應類型的 `libstock` 的 C FFI 序列化函數。
///
/// 使用範例如下：
///
/// ```ignore
/// serializer_function!(PriceDataField -> RPriceDataField);
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn serializer_function(item: TokenStream) -> TokenStream {
    serializer::inner_serializer_function(item.into()).into()
}

/// 產生對應類型的 `libstock` 的 C FFI 反序列化函數。
///
/// 使用範例如下：
///
/// ```ignore
/// deserializer_function!(RPriceDataField -> PriceDataField);
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn deserializer_function(item: TokenStream) -> TokenStream {
    serializer::inner_deserializer_function(item.into()).into()
}

/// 產生對應類型的 `libstock` 的 C FFI new 和 free 函數。
///
/// 使用範例如下：
///
/// ```ignore
/// alloc_function!(PriceDataField);
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn alloc_function(item: TokenStream) -> TokenStream {
    alloc::inner_alloc_function(item.into()).into()
}

/// 建立 FFI 與 Rust struct 之間的 Interop。
///
/// 使用範例如下：
///
/// ```ignore
/// #[derive(InteropStruct)]
/// #[rs_type(YourRustStruct)]
/// pub struct YourFFIStruct {
///   pub our_type: i32,
///
///   #[into]  // 可以直接 .into() 的類型。
///   pub our_intoable_type: OurFFiIntoStruct,
///
///   #[try_into]  // 可以直接 .try_into() 的類型。
///   pub our_tryintoable_type: OurFFiTryIntoStruct,
///
///   #[convert_box]  // Foreign Type 進行轉換。
///   pub foreign_type: char_p::Box,
/// 
///   #[default]  // 不需要特別指定，雙方用預設值即可。
///   pub end: (),
/// }
/// ```
#[proc_macro_derive(InteropStruct, attributes(rs_type, into, try_into, convert_box, default))]
#[proc_macro_error]
pub fn interop_struct_derive_macro(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    interop::interop_struct_derive_macro(input).into()
}

/// 建立 FFI 與 Rust enum 之間的 Interop。
///
/// 使用範例如下：
///
/// ```ignore
/// #[derive(InteropEnum)]
/// #[rs_type(YourRustEnum)]
/// pub enum YourFFIEnum {
///   VariantA = 1,
///   VariantB,
/// }
/// ```
#[proc_macro_derive(InteropEnum, attributes(rs_type, into, try_into, convert_box, default))]
#[proc_macro_error]
pub fn interop_enum_derive_macro(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemEnum);

    interop::interop_enum_derive_macro(input).into()
}
