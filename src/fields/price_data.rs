//! The module with a field to specify the price data,
//! which includes the price and quantity base.
//!
//! For more information, see [`PriceDataField`].

use ::safer_ffi::prelude::*;
use codegen::{alloc_function, deserializer_function, serializer_function, InteropStruct};
pub use wmjtyd_libstock::data::fields::PriceDataField as RPriceDataField;

/// The structure of a price data.
#[derive(Clone, Debug, InteropStruct)]
#[rs_type(RPriceDataField)]
#[derive_ReprC]
#[repr(C)]
pub struct PriceDataField {
    /// 價格 (5 bytes)
    #[convert_box]
    pub price: char_p::Box,

    /// 基本量 (5 bytes)
    #[convert_box]
    pub quantity_base: char_p::Box,
}

alloc_function!(PriceDataField);
serializer_function!(Field<10>, PriceDataField -> RPriceDataField);
deserializer_function!(Field<10>, RPriceDataField -> PriceDataField);
