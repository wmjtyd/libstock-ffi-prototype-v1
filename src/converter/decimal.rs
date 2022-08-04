use ::safer_ffi::prelude::*;
use safer_ffi::prelude::char_p;
use wmjtyd_libstock::data::fields::DecimalField;

use super::ConvertBox;
use crate::errors::LibstockErrors;

impl TryFrom<ConvertBox<&DecimalField<5>>> for ConvertBox<char_p::Box> {
    type Error = LibstockErrors;

    fn try_from(value: ConvertBox<&DecimalField<5>>) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .to_string()
                .try_into()
                .expect("failed to convert a string to C string."),
        ))
    }
}

impl TryFrom<ConvertBox<&char_p::Box>> for ConvertBox<DecimalField<5>> {
    type Error = LibstockErrors;

    fn try_from(value: ConvertBox<&char_p::Box>) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .0
                .to_str()
                .try_into()
                .map_err(|_| LibstockErrors::DecimalStringInvalid)?,
        ))
    }
}
