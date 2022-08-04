use codegen::{serializer_function, Interop};
use ::safer_ffi::prelude::*;
use wmjtyd_libstock::data::fields::PriceDataField as RPriceDataField;

use super::errors::LibstockErrors;

/// The structure of a price data.
#[derive(Clone, Debug, PartialEq, Eq, Interop)]
#[rs_type(RPriceDataField)]
#[derive_ReprC]
#[repr(C)]
pub struct PriceDataField {
    /// 價格 (5 bytes)
    pub price: char_p::Box,

    /// 基本量 (5 bytes)
    pub quantity_base: char_p::Box,
}

serializer_function!(PriceDataField -> RPriceDataField);
