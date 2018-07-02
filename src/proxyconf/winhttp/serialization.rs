mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Utf8(::std::str::Utf8Error);
        }

        links {
            Serialization(::string_serialization::Error, ::string_serialization::ErrorKind);
        }

        errors {
            InvalidVersion(version: u32) {
                description("invalid regitry settings version"),
                display("invalid regitry settings version: {}", version),
            }
        }
    }
}

pub use self::errors::*;

use super::types;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{BufReader, BufWriter, Read, Write};
use ::string_serialization;

const VERSION: u32 = 0x28u32;

fn mk_bit_field(config: &types::ProxyConfig) -> u32 {
    let mut conf = 0x01u32;

    if config.use_manual_proxy {
        conf = conf | 0x02
    }

    conf
}

pub fn serialize<W: Write>(config: &types::ProxyConfig, writer: W) -> Result<()> {
    let mut buffered = BufWriter::new(writer);

    buffered.write_u32::<LittleEndian>(VERSION)?;
    buffered.write_u32::<LittleEndian>(0x0u32)?; // Unknown byte
    buffered.write_u32::<LittleEndian>(mk_bit_field(&config))?;

    string_serialization::write(&mut buffered, &config.manual_proxy_address)?;
    string_serialization::write(&mut buffered, &config.manual_proxy_bypass_list)?;

    buffered.flush()?;

    return Ok(());
}

fn deserialize_config<R: Read>(mut reader: R) -> Result<types::ProxyConfig> {
    reader.read_u32::<LittleEndian>()?; // Unknown byte

    let conf = reader.read_u32::<LittleEndian>()?;
    let use_manual_proxy = (conf & 0x02) != 0x00;
    let manual_proxy_address = string_serialization::read(&mut reader)?;
    let manual_proxy_bypass_list = string_serialization::read(&mut reader)?;

    return Ok(types::ProxyConfig {
        use_manual_proxy,
        manual_proxy_address,
        manual_proxy_bypass_list,
    });
}

pub fn deserialize<'a, R: Read>(reader: R) -> Result<types::ProxyConfig> {
    let mut buffered = BufReader::new(reader);

    let version = buffered.read_u32::<LittleEndian>()?;
    if version != VERSION {
        bail!(ErrorKind::InvalidVersion(version));
    }

    return Ok(deserialize_config(buffered)?);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_no_proxy() {
        let data: Vec<u8> = vec![0x28u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00];

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.use_manual_proxy, false);
        assert_eq!(config.manual_proxy_address, String::from(""));
        assert_eq!(config.manual_proxy_bypass_list, String::from(""));

        let mut roundtrip = Vec::new();
        serialize(&config, &mut roundtrip).unwrap();
        assert_eq!(roundtrip, data);
    }

    #[test]
    fn deserialize_some_proxy() {
        let data: Vec<u8> = vec![0x28u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
                                 0x00u8, 0x00u8, 0x03u8, 0x00u8, 0x00u8, 0x00u8,
                                 0x04u8, 0x00u8, 0x00u8, 0x00u8, 0x61u8, 0x3Au8,
                                 0x34u8, 0x32u8, 0x04u8, 0x00u8, 0x00u8, 0x00u8,
                                 0x2Au8, 0x2Eu8, 0x34u8, 0x32u8];

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.use_manual_proxy, true);
        assert_eq!(config.manual_proxy_address, String::from("a:42"));
        assert_eq!(config.manual_proxy_bypass_list, String::from("*.42"));

        let mut roundtrip = Vec::new();
        serialize(&config, &mut roundtrip).unwrap();
        assert_eq!(roundtrip, data);
    }
}