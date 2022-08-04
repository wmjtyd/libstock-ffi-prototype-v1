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

/// 建立 FFI type 與 Rust type 之間的 Interop。
///
/// 每個欄位必須實作 `TryFrom<ConvertBox<&(Rust 的 type)>>` 和
/// `TryInto<ConvertBox<(FFI 的 type)>>`。
///
/// 使用範例如下：
///
/// ```ignore
/// #[derive(Interop)]
/// #[rs_type(YourRustStruct)]
/// pub struct YourFFIStruct {
///   pub test: i32,
///
///   #[into]  // 可以直接 .into() 的類型。
///   pub our_type: OurFFiStruct,
///
///   #[convert_box]  // Foreign Type 進行轉換。
///   pub foreign_type: char_p::Box,
/// }
/// ```
#[proc_macro_derive(Interop, attributes(rs_type, into, convert_box))]
#[proc_macro_error]
pub fn interop_derive_macro(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    interop::interop_derive_macro(input).into()
}
