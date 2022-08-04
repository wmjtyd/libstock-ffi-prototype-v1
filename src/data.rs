use ::safer_ffi::prelude::*;
use codegen::{deserializer_function, serializer_function, Interop};
use wmjtyd_libstock::data::fields::PriceDataField as RPriceDataField;

/// The structure of a price data.
#[derive(Clone, Debug, Interop)]
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

serializer_function!(PriceDataField -> RPriceDataField);
deserializer_function!(RPriceDataField -> PriceDataField);
