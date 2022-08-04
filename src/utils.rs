use crate::errors::{LibstockErrors, LibstockResult};

#[inline]
pub fn result_to_status_code(r: LibstockResult<()>) -> LibstockErrors {
    if let Err(e) = r {
        e
    } else {
        LibstockErrors::Succeed
    }
}
