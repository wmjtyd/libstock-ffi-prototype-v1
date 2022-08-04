use crate::errors::{LibstockResult, LibstockErrors};

#[inline]
pub fn result_to_status_code(r: LibstockResult<()>) -> LibstockErrors {
    if let Err(e) = r {
        e
    } else {
        LibstockErrors::Succeed
    }
}
