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

const VERSION: u32 = 0x46u32;

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

    buffered.write_u32::<LittleEndian>(VERSION)?;
    buffered.write_u32::<LittleEndian>(config.counter)?;
    buffered.write_u32::<LittleEndian>(mk_bit_field(&config))?;

    string_serialization::write(&mut buffered, &config.config.manual_proxy_address)?;
    string_serialization::write(&mut buffered, &config.config.manual_proxy_bypass_list)?;
    string_serialization::write(&mut buffered, &config.config.setup_script_address)?;

    for _ in 0..32 {
        buffered.write_u8(0)?;
    }

    buffered.flush()?;

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
    if version != VERSION {
        bail!(ErrorKind::InvalidVersion(version));
    }

    let counter = buffered.read_u32::<LittleEndian>()?;
    let config = deserialize_config(buffered)?;

    return Ok(types::FullConfig {
        counter,
        config,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::hex::FromHex;

    #[test]
    fn deserialize_no_proxy() {
        let data = "460000003A000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".from_hex().unwrap();

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.counter, 58);
        assert_eq!(config.config.automatically_detect_settings, false);
        assert_eq!(config.config.use_manual_proxy, false);
        assert_eq!(config.config.manual_proxy_address, String::from(""));
        assert_eq!(config.config.manual_proxy_bypass_list, String::from(""));
        assert_eq!(config.config.use_setup_script, false);
        assert_eq!(config.config.setup_script_address, String::from(""));

        let mut roundtrip = Vec::new();
        serialize(&config, &mut roundtrip).unwrap();
        assert_eq!(roundtrip, data);
    }

    #[test]
    fn deserialize_some_proxy() {
        let data ="46000000400000000F0000000D000000676F6F676C652E636F6D3A34320B0000004C6F6C3B3C6C6F63616C3E11000000687474703A2F2F676F6F676C652E66722F0000000000000000000000000000000000000000000000000000000000000000".from_hex().unwrap();

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.counter, 64);
        assert_eq!(config.config.automatically_detect_settings, true);
        assert_eq!(config.config.use_manual_proxy, true);
        assert_eq!(config.config.manual_proxy_address, String::from("google.com:42"));
        assert_eq!(config.config.manual_proxy_bypass_list, String::from("Lol;<local>"));
        assert_eq!(config.config.use_setup_script, true);
        assert_eq!(config.config.setup_script_address, String::from("http://google.fr/"));

        let mut roundtrip = Vec::new();
        serialize(&config, &mut roundtrip).unwrap();
        assert_eq!(roundtrip, data);
    }

    #[test]
    fn deserialize_some_proxy_not_used() {
        let data ="4600000044000000010000000D000000676F6F676C652E636F6D3A34320B0000004C6F6C3B3C6C6F63616C3E11000000687474703A2F2F676F6F676C652E66722F0000000000000000000000000000000000000000000000000000000000000000".from_hex().unwrap();

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.counter, 68);
        assert_eq!(config.config.automatically_detect_settings, false);
        assert_eq!(config.config.use_manual_proxy, false);
        assert_eq!(config.config.manual_proxy_address, String::from("google.com:42"));
        assert_eq!(config.config.manual_proxy_bypass_list, String::from("Lol;<local>"));
        assert_eq!(config.config.use_setup_script, false);
        assert_eq!(config.config.setup_script_address, String::from("http://google.fr/"));

        let mut roundtrip = Vec::new();
        serialize(&config, &mut roundtrip).unwrap();
        assert_eq!(roundtrip, data);
    }
}