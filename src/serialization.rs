mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Utf8(::std::str::Utf8Error);
        }

        errors {
            InvalidSize(size: usize) {
                description("usize is too big to become an u32"),
                display("usize is too big to become an u32: {}", size),
            }
            InvalidVersion(version: u32) {
                description("invalid regitry settings version"),
                display("invalid regitry settings version: {}", version),
            }
        }
    }
}

pub use self::errors::*;

use types;
use std;
use std::io::{BufWriter, BufReader, Write, Read};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};

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

fn usize_to_u32(a: usize) -> Result<u32> {
    if a > std::u32::MAX as usize {
        bail!(ErrorKind::InvalidSize(a));
    } else {
        Ok(a as u32)
    }
}

fn write_string<W: Write>(writer: &mut W, s: &str) -> Result<()> {
    writer.write_u32::<LittleEndian>(usize_to_u32(s.len())?)?;
    writer.write_all(s.as_bytes())?;
    return Ok(());
}

pub fn serialize<W: Write>(
    config: &types::FullConfig,
    writer: W
) -> Result<()> {
    let mut buffered = BufWriter::new(writer);

    buffered.write_u32::<LittleEndian>(0x46u32)?;
    buffered.write_u32::<LittleEndian>(config.counter)?;
    buffered.write_u32::<LittleEndian>(mk_bit_field(&config))?;

    write_string(&mut buffered, &config.config.manual_proxy_address)?;
    write_string(&mut buffered, &config.config.manual_proxy_overrides)?;
    write_string(&mut buffered, &config.config.setup_script_address)?;

    for _ in 0..32 {
        buffered.write_u8(0)?;
    }

    return Ok(());
}

fn read_string<R: Read>(reader: &mut R) -> Result<String> {
    let len = reader.read_u32::<LittleEndian>()?;
    let mut bytes = vec![0; len as usize];
    reader.read_exact(&mut bytes)?;

    let s = std::str::from_utf8(&bytes)?;
    return Ok(String::from(s));
}

pub fn deserialize<R: Read>(
    reader: R
) -> Result<types::FullConfig> {
    let mut buffered = BufReader::new(reader);

    let version = buffered.read_u32::<LittleEndian>()?;
    if version != 0x46u32 {
        bail!(ErrorKind::InvalidVersion(version));
    }

    let counter = buffered.read_u32::<LittleEndian>()?;
    let conf = buffered.read_u32::<LittleEndian>()?;

    let automatically_detect_settings = (conf & 0x08) != 0x00;
    let use_setup_script = (conf & 0x04) != 0x00;
    let use_manual_proxy = (conf & 0x02) != 0x00;

    let manual_proxy_address = read_string(&mut buffered)?;
    let manual_proxy_overrides = read_string(&mut buffered)?;
    let setup_script_address = read_string(&mut buffered)?;

    let config = types::FullConfig {
        counter,
        config: types::ProxyConfig {
            automatically_detect_settings,
            use_setup_script,
            setup_script_address,
            use_manual_proxy,
            manual_proxy_address,
            manual_proxy_overrides,
        }
    };

    return Ok(config);
}