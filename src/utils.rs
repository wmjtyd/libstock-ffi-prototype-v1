use crate::errors::{LibstockErrors, LibstockResult};

#[inline]
pub fn result_to_status_code(r: LibstockResult<()>) -> LibstockErrors {
    if let Err(e) = r {
        e
    } else {
        LibstockErrors::Succeed
    }
}

macro_rules! create_from_owned {
    (@from $src:ty, $tgt:ty) => {
        impl From<$src> for $tgt {
            fn from(value: $src) -> Self {
                (&value).into()
            }
        }
    };

    ($ffi:ty, $rst:ty) => {
        $crate::utils::create_from_owned!(@from $ffi, $rst);
        $crate::utils::create_from_owned!(@from $rst, $ffi);
    };
}

macro_rules! create_try_from_owned {
    (@from $src:ty, $tgt:ty) => {
        impl TryFrom<$src> for $tgt {
            type Error = $crate::errors::LibstockErrors;
    
            fn try_from(value: $src) -> Result<Self, Self::Error> {
                (&value).try_into()
            }
        }
    };

    ($ffi:ty, $rst:ty) => {
        $crate::utils::create_try_from_owned!(@from $ffi, $rst);
        $crate::utils::create_try_from_owned!(@from $rst, $ffi);
    };
}

pub(crate) use { create_from_owned, create_try_from_owned };