use std::io::Cursor;

use ::safer_ffi::prelude::*;
use wmjtyd_libstock::data::fields::{FieldError, PriceDataField as RPriceDataField};
use wmjtyd_libstock::data::serializer::FieldSerializer;

use super::errors::LibstockErrors;
use crate::errors::LibstockResult;
use crate::types::ErrorCode;

/// The structure of a price data.
// #[derive(Clone, Debug, PartialEq, Eq)]
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
        let price = value.price.to_str();
        let quantity_base = value.quantity_base.to_str();

        Ok(Self {
            price: price
                .try_into()
                .map_err(|_| LibstockErrors::DecimalStringInvalid)?,
            quantity_base: quantity_base
                .try_into()
                .map_err(|_| LibstockErrors::DecimalStringInvalid)?,
        })
    }
}

impl TryFrom<RPriceDataField> for PriceDataField {
    type Error = LibstockErrors;

    fn try_from(value: RPriceDataField) -> Result<Self, Self::Error> {
        let price = value
            .price
            .to_string()
            .try_into()
            .expect("failed to convert a string to C string.");
        let quantity_base = value
            .price
            .to_string()
            .try_into()
            .expect("failed to convert a string to C string.");

        Ok(Self {
            price,
            quantity_base,
        })
    }
}

fn inner_serialize_field<'a, S, F: 'a>(
    input: &'a F,
    mut buf: c_slice::Mut<u8>,
    written_len: &mut usize,
) -> LibstockResult<()>
where
    S: FieldSerializer<10, Err = FieldError> + TryFrom<&'a F, Error = LibstockErrors>,
{
    let rust_type = S::try_from(input)?;
    let mut cursor = Cursor::new(&mut buf[..]);

    rust_type.serialize_to_writer(&mut cursor)??;

    *written_len = cursor.position() as usize;

    Ok(())
}

#[ffi_export]
pub fn serialize_price_data_field(
    input: &PriceDataField,
    buf: c_slice::Mut<u8>,
    written_len: &mut usize,
) -> ErrorCode {
    inner_serialize_field::<RPriceDataField, _>(input, buf, written_len)
        .map(|_| 0)
        .unwrap_or_else(|e| e as u32)
}
