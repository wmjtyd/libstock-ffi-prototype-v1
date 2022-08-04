use codegen::serializer_function;
use ::safer_ffi::prelude::*;
use wmjtyd_libstock::data::fields::PriceDataField as RPriceDataField;

use crate::utils::{decimal_field_to_c_str, c_str_to_decimal_field};

use super::errors::LibstockErrors;

/// The structure of a price data.
#[derive(Clone, Debug, PartialEq, Eq)]
#[derive_ReprC]
#[repr(C)]
pub struct PriceDataField {
    /// 價格 (5 bytes)
    pub price: char_p::Box,

    /// 基本量 (5 bytes)
    pub quantity_base: char_p::Box,
}

impl TryFrom<&PriceDataField> for RPriceDataField {
    type Error = LibstockErrors;

    fn try_from(value: &PriceDataField) -> Result<Self, Self::Error> {
        Ok(Self {
            price: c_str_to_decimal_field(value.price.as_ref())?,
            quantity_base: c_str_to_decimal_field(value.quantity_base.as_ref())?,
        })
    }
}

impl TryFrom<RPriceDataField> for PriceDataField {
    type Error = LibstockErrors;

    fn try_from(value: RPriceDataField) -> Result<Self, Self::Error> {
        Ok(Self {
            price: decimal_field_to_c_str(&value.price),
            quantity_base: decimal_field_to_c_str(&value.quantity_base),
        })
    }
}

serializer_function!(PriceDataField -> RPriceDataField);
