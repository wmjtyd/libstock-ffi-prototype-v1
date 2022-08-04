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

pub fn inner_serialize_field<'a, RST, FFI: 'a>(
    input: &'a FFI,
    buf: &mut c_slice::Mut<u8>,
    written_len: &mut usize,
) -> LibstockResult<()>
where
    RST: FieldSerializer<10, Err = FieldError> + TryFrom<&'a FFI, Error = LibstockErrors>,
{
    let rust_type = RST::try_from(input)?;
    let mut cursor = Cursor::new(&mut buf[..]);

    rust_type.serialize_to_writer(&mut cursor)??;

    *written_len = cursor.position() as usize;

    Ok(())
}

pub fn inner_deserialize_field<'a, RST, FFI>(
    input: &'a c_slice::Ref<'a, u8>,
    output: &'a mut FFI,
) -> LibstockResult<()>
where
    FFI: TryFrom<RST, Error = LibstockErrors> + Clone,
    RST: FieldDeserializer<10, Err = FieldError>,
{
    let mut input = input.as_slice();

    let data = RST::deserialize_from_reader(&mut input)??;

    let result = FFI::try_from(data)?;

    unsafe { std::ptr::write(output, result) };

    Ok(())
}

pub fn inner_serialize_structure<'a, RST, FFI: 'a, E>(
    input: &'a FFI,
    buf: &mut c_slice::Mut<u8>,
    written_len: &mut usize,
) -> LibstockResult<()>
where
    RST: StructSerializer<Err = E> + TryFrom<&'a FFI, Error = LibstockErrors>,
    LibstockErrors: From<E>,
{
    let rust_type = RST::try_from(input)?;
    let mut cursor = Cursor::new(&mut buf[..]);

    rust_type.serialize(&mut cursor)?;

    *written_len = cursor.position() as usize;

    Ok(())
}

pub fn inner_deserialize_structure<'a, RST, FFI, E>(
    input: &'a c_slice::Ref<'a, u8>,
    output: &'a mut FFI,
) -> LibstockResult<()>
where
    FFI: TryFrom<RST, Error = LibstockErrors> + Clone,
    RST: StructDeserializer<Err = E>,
    LibstockErrors: From<E>,
{
    let mut input = input.as_slice();

    let data = RST::deserialize(&mut input)?;

    let result = FFI::try_from(data)?;

    unsafe { std::ptr::write(output, result) };

    Ok(())
}
