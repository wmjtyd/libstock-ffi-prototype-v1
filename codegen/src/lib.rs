mod serializer;

use proc_macro::TokenStream;

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
pub fn serializer_function(item: TokenStream) -> TokenStream {
    serializer::inner_serializer_function(item.into()).into()
}
