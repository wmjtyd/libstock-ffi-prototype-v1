use crate::errors::{LibstockResult, LibstockErrors};
use ::safer_ffi::prelude::*;
use wmjtyd_libstock::data::{num::Decimal, fields::DecimalField};

#[inline]
pub fn result_to_status_code(r: LibstockResult<()>) -> LibstockErrors {
    if let Err(e) = r {
        e
    } else {
        LibstockErrors::Succeed
    }
}

#[inline]
pub fn c_str_to_decimal_field<const LEN: usize>(c_str: char_p::Ref) -> LibstockResult<DecimalField<LEN>> {
    c_str
        .to_str()
        .try_into()
        .map_err(|_| LibstockErrors::DecimalStringInvalid)
}

#[inline]
pub fn decimal_field_to_c_str<const LEN: usize>(f: &DecimalField<LEN>) -> char_p::Box {
    f
        .to_string()
        .try_into()
        .expect("failed to convert a string to C string.")
}

