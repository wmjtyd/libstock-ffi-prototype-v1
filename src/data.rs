use core::slice;
use std::ptr::slice_from_raw_parts_mut;

use safer_ffi::String as FFIString;
use ::safer_ffi::prelude::*;
use wmjtyd_libstock::data::{fields::PriceDataField as RPriceDataField, serializer::FieldSerializer};
use crate::{types::{interop::Interopable, ErrorCode}, errors::LibstockResult};

use super::errors::LibstockErrors;

/// The structure of a price data.
// #[derive(Clone, Debug, PartialEq, Eq)]
#[derive_ReprC]
#[repr(C)]
pub struct PriceDataField {
    /// 價格 (5 bytes)
    pub price: FFIString,

    /// 基本量 (5 bytes)
    pub quantity_base: FFIString,
}

impl TryFrom<PriceDataField> for RPriceDataField {
    type Error = LibstockErrors;

    fn try_from(value: PriceDataField) -> Result<Self, Self::Error> {
        let price: &str = value.price.as_ref();
        let quantity_base: &str = value.quantity_base.as_ref();

        Ok(Self {
            price: price.try_into().map_err(|_| LibstockErrors::DecimalStringInvalid)?,
            quantity_base: quantity_base.try_into().map_err(|_| LibstockErrors::DecimalStringInvalid)?,
        })
    }
}

impl TryFrom<RPriceDataField> for PriceDataField {
    type Error = LibstockErrors;

    fn try_from(value: RPriceDataField) -> Result<Self, Self::Error> {
        Ok(Self {
            price: value.price.to_string().into(),
            quantity_base: value.quantity_base.to_string().into(),
        })
    }
}

impl Interopable for PriceDataField {
    type Target = RPriceDataField;
}

unsafe fn inner_serialize_price_data_field(input: PriceDataField, buf_pointer: *mut u8, buf_len: usize) -> LibstockResult<()> {
    let rust_type = RPriceDataField::try_from(input)?;
    // 內部分配一個 buffer 追蹤長度。
    let mut buf = Vec::with_capacity(buf_len);

    // let mut buf = slice::from_raw_parts_mut(buf_pointer, buf_len);

    rust_type.serialize_to_writer(&mut buf)??;

    let mut user_buf = slice_from_raw_parts_mut(data, len);
    user_buf.copy_from_slice(&user_buf);

    Ok(())
}

#[ffi_export]
pub unsafe fn serialize_price_data_field(input: PriceDataField, buf_pointer: *mut u8, buf_len: usize) -> ErrorCode {
    inner_serialize_price_data_field(input, buf_pointer, buf_len)
        .map(|_| 0).unwrap_or_else(|e| e as u32)
}
