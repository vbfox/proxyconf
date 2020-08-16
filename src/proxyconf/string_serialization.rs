//! Handle strings serialized with an u32 for the size followed by ASCII characters

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StringSerializationError {
    #[error("usize is too big to become an u32: {0}")]
    InvalidSize(usize),

    #[error(transparent)]
    Io(#[from] ::std::io::Error),

    #[error(transparent)]
    Utf8(#[from] ::std::str::Utf8Error),
}

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use std::{
    io,
    io::{ErrorKind, Read, Write},
};

fn usize_to_u32(a: usize) -> Result<u32, StringSerializationError> {
    if a > std::u32::MAX as usize {
        Err(StringSerializationError::InvalidSize(a))
    } else {
        Ok(a as u32)
    }
}

/// Write a string prepended by an u32 containing it's size
pub fn write<W: Write>(writer: &mut W, s: &str) -> Result<(), StringSerializationError> {
    writer.write_u32::<LittleEndian>(usize_to_u32(s.len())?)?;
    writer.write_all(s.as_bytes())?;
    Ok(())
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
            Ok(String::from(s))
        }
        Err(e) => {
            if e.kind() == ErrorKind::UnexpectedEof {
                Ok(String::from(""))
            } else {
                Err(e.into())
            }
        }
    }
}
