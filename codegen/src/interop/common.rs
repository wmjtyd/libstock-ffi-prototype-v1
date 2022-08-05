use proc_macro_error::{abort, abort_call_site};
use syn::{Attribute, Ident, spanned::Spanned};

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
pub fn extract_simple_ident_attribute(attributes: &[Attribute], ident: &str) -> Ident {
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
