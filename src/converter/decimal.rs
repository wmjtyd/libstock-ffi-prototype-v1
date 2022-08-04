use ::safer_ffi::prelude::*;

use safer_ffi::prelude::char_p;
use wmjtyd_libstock::data::fields::DecimalField;

use crate::errors::LibstockErrors;

use super::ConvertBox;

impl TryFrom<ConvertBox<&DecimalField<5>>> for char_p::Box {
    type Error = LibstockErrors;

    fn try_from(value: ConvertBox<&DecimalField<5>>) -> Result<Self, Self::Error> {
        Ok(value.0
            .to_string()
            .try_into()
            .expect("failed to convert a string to C string.")
        )
    }
}

impl TryFrom<ConvertBox<&char_p::Box>> for DecimalField<5> {
    type Error = LibstockErrors;

    fn try_from(value: ConvertBox<&char_p::Box>) -> Result<Self, Self::Error> {
        value.0
            .to_str()
            .try_into()
            .map_err(|_| LibstockErrors::DecimalStringInvalid)
    }
}
