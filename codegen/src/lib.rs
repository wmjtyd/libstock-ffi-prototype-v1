use proc_macro::TokenStream;
use syn::token::Token;

mod serializer;

/// 產生 `libstock` 的 C FFI 序列化函數。
/// 
/// 使用方式是這樣的：
/// 
/// ```ignore
/// serializer_function!(type);
/// ```
/// 
/// 其中 `<type>` 為要序列化的類型。
///
/// 產生出的序列化函數，簽名大致是這樣的：
/// 
/// ```ignore
/// #[ffi_export]
/// fn serialize_<type>(input: <type>, buf: &mut [u8]) -> u32;
/// ```
///
/// `buf` 為序列化資料的目的地；`int` 為狀態（0 為成功，非 0 為失敗）。
#[proc_macro]
pub fn serializer_function(item: TokenStream) -> TokenStream {
    let ident = format!("serialize_{item}", item = item.to_string().to_lowercase());

    todo!()
}
