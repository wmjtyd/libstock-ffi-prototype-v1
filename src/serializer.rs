use std::io::Cursor;

use safer_ffi::prelude::c_slice;
use wmjtyd_libstock::data::fields::FieldError;
use wmjtyd_libstock::data::serializer::{
    FieldDeserializer,
    FieldSerializer,
    StructDeserializer,
    StructSerializer,
};

use crate::errors::{LibstockErrors, LibstockResult};

pub fn inner_serialize_field<'a, const LEN: usize, RST, FFI: 'a, E, F>(
    input: &'a FFI,
    buf: &mut c_slice::Mut<u8>,
    written_len: &mut usize,
) -> LibstockResult<()>
where
    RST: TryFrom<&'a FFI, Error = E>,
    LibstockErrors: From<E>,
    F: From<RST> + FieldSerializer<LEN, Err = FieldError>,
{
    let rust_type = F::from(RST::try_from(input)?);
    let mut cursor = Cursor::new(&mut buf[..]);

    rust_type.serialize_to_writer(&mut cursor)??;

    *written_len = cursor.position() as usize;

    Ok(())
}

pub fn inner_deserialize_field<'a, const LEN: usize, RST, FFI, E, F>(
    input: &'a c_slice::Ref<'a, u8>,
    output: &'a mut FFI,
) -> LibstockResult<()>
where
    FFI: TryFrom<RST, Error = E> + Clone,
    RST: From<F>,
    LibstockErrors: From<E>,
    F: FieldDeserializer<LEN, Err = FieldError>,
{
    let mut input = input.as_slice();

    let data = F::deserialize_from_reader(&mut input)??;

    let result = FFI::try_from(RST::from(data))?;

    unsafe { std::ptr::write(output, result) };

    Ok(())
}

pub fn inner_serialize_structure<'a, RST, FFI: 'a, E, S>(
    input: &'a FFI,
    buf: &mut c_slice::Mut<u8>,
    written_len: &mut usize,
) -> LibstockResult<()>
where
    RST: TryFrom<&'a FFI, Error = LibstockErrors>,
    LibstockErrors: From<E>,
    S: From<RST> + StructSerializer<Err = E>,
{
    let rust_type = S::from(RST::try_from(input)?);
    let mut cursor = Cursor::new(&mut buf[..]);

    rust_type.serialize(&mut cursor)?;

    *written_len = cursor.position() as usize;

    Ok(())
}

pub fn inner_deserialize_structure<'a, RST, FFI, E, S>(
    input: &'a c_slice::Ref<'a, u8>,
    output: &'a mut FFI,
) -> LibstockResult<()>
where
    FFI: TryFrom<RST, Error = LibstockErrors> + Clone,
    RST: From<S>,
    LibstockErrors: From<E>,
    S: StructDeserializer<Err = E>,
{
    let mut input = input.as_slice();

    let data = S::deserialize(&mut input)?;

    let result = FFI::try_from(RST::from(data))?;

    unsafe { std::ptr::write(output, result) };

    Ok(())
}
