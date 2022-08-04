use std::borrow::Borrow;

use safer_ffi::char_p::char_p_boxed;
use wmjtyd_libstock::data::fields::{DecimalField, PriceDataField as RPDF};
use wmjtyd_libstock_ffi::errors::LibstockErrors;

pub struct ConvertBox<T>(T);

impl<T> std::ops::Deref for ConvertBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for ConvertBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for ConvertBox<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl TryFrom<ConvertBox<&DecimalField<5>>> for char_p_boxed {
    type Error = LibstockErrors;

    fn try_from(value: ConvertBox<&DecimalField<5>>) -> Result<Self, Self::Error> {
        Ok(value.0
            .to_string()
            .try_into()
            .expect("failed to convert a string to C string.")
        )
    }
}

impl TryFrom<ConvertBox<&char_p_boxed>> for DecimalField<5> {
    type Error = LibstockErrors;

    fn try_from(value: ConvertBox<&char_p_boxed>) -> Result<Self, Self::Error> {
        value.0
            .to_str()
            .try_into()
            .map_err(|_| LibstockErrors::DecimalStringInvalid)
    }
}

pub struct PriceDataField {
    /// 價格 (5 bytes)
    pub price: char_p_boxed,

    /// 基本量 (5 bytes)
    pub quantity_base: char_p_boxed,
}

/* AUTO GENERATION ATTEMPT */

impl TryFrom<&PriceDataField> for RPDF {
    type Error = LibstockErrors;

    fn try_from(value: &PriceDataField) -> Result<Self, Self::Error> {
        Ok(Self {
            price: ConvertBox(&value.price).try_into()?,
            quantity_base: ConvertBox(&value.quantity_base).try_into()?,
        })
    }
}

impl TryFrom<&RPDF> for PriceDataField {
    type Error = LibstockErrors;

    fn try_from(value: &RPDF) -> Result<Self, Self::Error> {
        Ok(Self {
            price: ConvertBox(&value.price).try_into()?,
            quantity_base: ConvertBox(&value.quantity_base).try_into()?,
        })
    }
}


fn main() {
    
}
