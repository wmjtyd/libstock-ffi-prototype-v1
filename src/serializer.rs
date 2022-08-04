use std::io::Cursor;

use safer_ffi::prelude::c_slice;
use wmjtyd_libstock::data::fields::FieldError;
use wmjtyd_libstock::data::serializer::FieldSerializer;

use crate::errors::{LibstockErrors, LibstockResult};

pub fn inner_serialize_field<'a, S, F: 'a>(
    input: &'a F,
    buf: &mut c_slice::Mut<u8>,
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
