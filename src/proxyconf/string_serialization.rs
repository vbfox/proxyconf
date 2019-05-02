//! Handle strings serialized with an u32 for the size followed by ASCII characters

#[derive(Debug, Fail)]
pub enum StringSerializationError {
    #[fail(display = "usize is too big to become an u32: {}", _0)]
    InvalidSize(usize),

    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] ::std::io::Error),

    #[fail(display = "{}", _0)]
    Utf8(#[fail(cause)] ::std::str::Utf8Error),
}

impl From<::std::io::Error> for StringSerializationError {
    fn from(error: ::std::io::Error) -> StringSerializationError {
        StringSerializationError::Io(error)
    }
}

impl From<::std::str::Utf8Error> for StringSerializationError {
    fn from(error: ::std::str::Utf8Error) -> StringSerializationError {
        StringSerializationError::Utf8(error)
    }
}

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std;
use std::io;
use std::io::{Read, Write};

fn usize_to_u32(a: usize) -> Result<u32, StringSerializationError> {
    if a > std::u32::MAX as usize {
        return Err(StringSerializationError::InvalidSize(a).into());
    } else {
        return Ok(a as u32);
    }
}

/// Write a string prepended by an u32 containing it's size
pub fn write<W: Write>(writer: &mut W, s: &str) -> Result<(), StringSerializationError> {
    writer.write_u32::<LittleEndian>(usize_to_u32(s.len())?)?;
    writer.write_all(s.as_bytes())?;
    return Ok(());
}

/// Read a string prepended by an u32 containing it's size, also returning empty on EOF for the len field instead of an
/// error.
pub fn read<R: Read>(reader: &mut R) -> Result<String, StringSerializationError> {
    let len_result: io::Result<u32> = reader.read_u32::<LittleEndian>();
    match len_result {
        Ok(len) => {
            let mut bytes = vec![0; len as usize];
            reader.read_exact(&mut bytes)?;

            let s = std::str::from_utf8(&bytes)?;
            return Ok(String::from(s));
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                return Ok(String::from(""));
            } else {
                return Err(e.into());
            }
        }
    }
}
