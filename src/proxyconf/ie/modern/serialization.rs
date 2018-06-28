mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Utf8(::std::str::Utf8Error);
            Serialization(super::super::super::super::string_serialization::Error);
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
use super::super::super::string_serialization;

fn mk_bit_field(config: &types::FullConfig) -> u32 {
    let mut conf = 0x01u32;

    if config.config.use_manual_proxy {
        conf = conf | 0x02
    }
    if config.config.use_setup_script {
        conf = conf | 0x04
    }
    if config.config.automatically_detect_settings {
        conf = conf | 0x08
    }

    conf
}

pub fn serialize<W: Write>(config: &types::FullConfig, writer: W) -> Result<()> {
    let mut buffered = BufWriter::new(writer);

    buffered.write_u32::<LittleEndian>(0x46u32)?;
    buffered.write_u32::<LittleEndian>(config.counter)?;
    buffered.write_u32::<LittleEndian>(mk_bit_field(&config))?;

    string_serialization::write(&mut buffered, &config.config.manual_proxy_address)?;
    string_serialization::write(&mut buffered, &config.config.manual_proxy_bypass_list)?;
    string_serialization::write(&mut buffered, &config.config.setup_script_address)?;

    for _ in 0..32 {
        buffered.write_u8(0)?;
    }

    return Ok(());
}

fn deserialize_config<R: Read>(mut reader: R) -> Result<types::ProxyConfig> {
    let conf = reader.read_u32::<LittleEndian>()?;

    let automatically_detect_settings = (conf & 0x08) != 0x00;
    let use_setup_script = (conf & 0x04) != 0x00;
    let use_manual_proxy = (conf & 0x02) != 0x00;

    let manual_proxy_address = string_serialization::read(&mut reader)?;
    let manual_proxy_bypass_list = string_serialization::read(&mut reader)?;
    let setup_script_address = string_serialization::read(&mut reader)?;

    return Ok(types::ProxyConfig {
        automatically_detect_settings,
        use_setup_script,
        setup_script_address,
        use_manual_proxy,
        manual_proxy_address,
        manual_proxy_bypass_list,
    });
}

pub fn deserialize<'a, R: Read>(reader: R) -> Result<types::FullConfig> {
    let mut buffered = BufReader::new(reader);

    let version = buffered.read_u32::<LittleEndian>()?;
    if version != 0x46u32 {
        bail!(ErrorKind::InvalidVersion(version));
    }

    let counter = buffered.read_u32::<LittleEndian>()?;
    let config = deserialize_config(buffered)?;

    return Ok(types::FullConfig {
        counter,
        config,
    });
}
