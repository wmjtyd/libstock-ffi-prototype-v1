//! The errors that [`libstock`](wmjtyd_libstock) will return.

use ::safer_ffi::prelude::*;
use wmjtyd_libstock::data::fields::FieldError;

#[ffi_export]
#[derive_ReprC]
#[repr(u32)]
/// The errors that this FFI will return.
pub enum LibstockErrors {
    /// Failed to serialize the specified data.
    SerializeFailed = 0,

    /// The passed decimal number string is invalid.
    DecimalStringInvalid,

    /// The trade side ID is unexpected.
    UnexpectedTradeSide,

    /// Failed to get the system time.
    SystemTimeError,

    /// The exchange is unimplemented.
    UnimplementedExchange,

    /// The info type is unimplemented.
    UnimplementedInfoType,

    /// The period is unimplemented.
    UnimplementedPeriod,

    /// Failed to convert between a decimal number and a float.
    FloatOverflow,

    /// Data ended too early (missing \\0 in the end!)
    DataEndedTooEarly,

    /// I/O error from Rust.
    IoError,
}

pub type LibstockResult<T> = Result<T, LibstockErrors>;

impl From<FieldError> for LibstockErrors {
    fn from(fe: FieldError) -> Self {
        log::error!("Libstock FFI Error: {fe:?}");

        match fe {
            FieldError::UnimplementedPeriod(_) => LibstockErrors::UnimplementedPeriod,
            FieldError::UnimplementedExchange(_) => LibstockErrors::UnimplementedExchange,
            FieldError::UnimplementedInfoType(_) => LibstockErrors::UnimplementedInfoType,
            FieldError::UnexpectedTradeSide(_) => LibstockErrors::UnexpectedTradeSide,
            FieldError::SystemTimeError(_) => LibstockErrors::SystemTimeError,
            FieldError::FloatOverflow(_) => LibstockErrors::FloatOverflow,
            FieldError::DataEndedTooEarly => LibstockErrors::DataEndedTooEarly,
            FieldError::NumError(_) => unreachable!("No NumError currently."),
        }
    }
}

impl From<std::io::Error> for LibstockErrors {
    fn from(_e: std::io::Error) -> Self {
        // FIXME
        LibstockErrors::IoError
    }
}
