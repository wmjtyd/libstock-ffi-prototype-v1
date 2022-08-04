use convert_case::{Casing, Case};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

pub fn inner_alloc_function(ffi_type: TokenStream) -> TokenStream {
    let ffi_type_str = ffi_type.to_string();

    let snake_case_ffi_type = ffi_type_str.to_case(Case::Snake);
    let upper_ffi_type = snake_case_ffi_type.to_ascii_uppercase();

    let layout_ident = format_ident!("{upper_ffi_type}_LAYOUT");
    let new_ident = format_ident!("new_{snake_case_ffi_type}");
    let free_ident = format_ident!("free_{snake_case_ffi_type}");

    let new_doc_str = format!("Allocate a new `{new_ident}`.");
    
    let free_doc_str = format!("Deallocate a new `{free_ident}`.");
    let free_safety_doc_str = format!("`f` should be the pointer allocated by `{new_ident}`.");

    quote! {
        const #layout_ident: ::std::alloc::Layout = ::std::alloc::Layout::new::<#ffi_type>();

        #[doc = #new_doc_str]
        /// 
        /// # Safety
        /// 
        /// The allocated memory may not be initiated.
        #[ffi_export]
        pub unsafe fn #new_ident() -> *mut #ffi_type {
            ::std::alloc::alloc(#layout_ident).cast()
        }

        #[doc = #free_doc_str]
        /// 
        /// # Safety
        /// 
        #[doc = #free_safety_doc_str]
        #[ffi_export]
        pub unsafe fn #free_ident(f: *mut #ffi_type) {
            std::alloc::dealloc(f.cast(), #layout_ident)
        }

    }
}
