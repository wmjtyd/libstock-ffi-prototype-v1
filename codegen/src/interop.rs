pub(self) mod common;

pub mod structure;
pub mod enumeration;

pub use structure::interop_struct_derive_macro;
pub use enumeration::interop_enum_derive_macro;
