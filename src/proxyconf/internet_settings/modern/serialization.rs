#[derive(Debug, Fail)]
pub enum SerializationError {
    #[fail(display = "Invalid registry settings version: {}", _0)]
    InvalidVersion(u32),

    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] ::std::io::Error),

    #[fail(display = "{}", _0)]
    Utf8(#[fail(cause)] ::std::str::Utf8Error),

    #[fail(display = "{}", _0)]
    StringSerialization(::string_serialization::StringSerializationError),
}

impl From<::std::io::Error> for SerializationError {
    fn from(error: ::std::io::Error) -> SerializationError {
        SerializationError::Io(error)
    }
}

impl From<::std::str::Utf8Error> for SerializationError {
    fn from(error: ::std::str::Utf8Error) -> SerializationError {
        SerializationError::Utf8(error)
    }
}

impl From<::string_serialization::StringSerializationError> for SerializationError {
    fn from(error: ::string_serialization::StringSerializationError) -> SerializationError {
        SerializationError::StringSerialization(error)
    }
}

use super::types;
use super::{IE6_VERSION, IE7_VERSION, WINHTTP_VERSION};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{BufReader, BufWriter, Read, Write};
use string_serialization;

fn mk_bit_field(version: u32, config: &types::FullConfig) -> u32 {
    let mut conf = 0x01;

    if config.config.use_manual_proxy {
        conf = conf | 0x02
    }

    if version >= IE6_VERSION {
        if config.config.use_setup_script {
            conf = conf | 0x04
        }
        if config.config.automatically_detect_settings {
            conf = conf | 0x08
        }
    }

    conf
}

pub fn serialize<W: Write>(
    config: &types::FullConfig,
    writer: W,
) -> Result<(), SerializationError> {
    let mut buffered = BufWriter::new(writer);

    let version = config.version;
    if version != WINHTTP_VERSION && version != IE6_VERSION && version != IE7_VERSION {
        return Err(SerializationError::InvalidVersion(version));
    }

    buffered.write_u32::<LittleEndian>(version)?;
    buffered.write_u32::<LittleEndian>(config.counter)?;
    buffered.write_u32::<LittleEndian>(mk_bit_field(version, &config))?;

    string_serialization::write(&mut buffered, &config.config.manual_proxy_address)?;
    string_serialization::write(&mut buffered, &config.config.manual_proxy_bypass_list)?;

    if version >= IE6_VERSION {
        string_serialization::write(&mut buffered, &config.config.setup_script_address)?;

        let unknown_bytes = match version {
            IE6_VERSION => 28,
            _ => 32,
        };
        for _ in 0..unknown_bytes {
            buffered.write_u8(0)?;
        }
    }
    buffered.flush()?;

    return Ok(());
}

fn deserialize_config<R: Read>(mut reader: R) -> Result<types::ProxyConfig, SerializationError> {
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

pub fn deserialize<'a, R: Read>(reader: R) -> Result<types::FullConfig, SerializationError> {
    let mut buffered = BufReader::new(reader);

    let version = buffered.read_u32::<LittleEndian>()?;
    if version < WINHTTP_VERSION || version > IE7_VERSION {
        // Versions seem forward compatible but it's hard to be sure that it will always be the case
        // The chance to encounter anything older than WinHTTP version seem small ;-)
        return Err(SerializationError::InvalidVersion(version));
    }

    let counter = buffered.read_u32::<LittleEndian>()?;
    let config = deserialize_config(buffered)?;

    return Ok(types::FullConfig {
        version,
        counter,
        config,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::FromHex;

    #[test]
    fn winhttp_deserialize_no_proxy() {
        let data = "2800000000000000010000000000000000000000"
            .from_hex()
            .unwrap();

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.version, WINHTTP_VERSION);
        assert_eq!(config.counter, 0);
        assert_eq!(config.config.use_manual_proxy, false);
        assert_eq!(config.config.manual_proxy_address, String::from(""));
        assert_eq!(config.config.manual_proxy_bypass_list, String::from(""));

        let mut roundtrip = Vec::new();
        serialize(&config, &mut roundtrip).unwrap();
        assert_eq!(roundtrip, data);
    }

    #[test]
    fn winhttp_deserialize_some_proxy() {
        let data = "28000000000000000300000004000000613A3432040000002A2E3432"
            .from_hex()
            .unwrap();

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.version, WINHTTP_VERSION);
        assert_eq!(config.counter, 0);
        assert_eq!(config.config.use_manual_proxy, true);
        assert_eq!(config.config.manual_proxy_address, String::from("a:42"));
        assert_eq!(config.config.manual_proxy_bypass_list, String::from("*.42"));

        let mut roundtrip = Vec::new();
        serialize(&config, &mut roundtrip).unwrap();
        assert_eq!(roundtrip, data);
    }

    #[test]
    fn ie6_deserialize_no_proxy() {
        let data = "3C0000003A0000000900000000000000000000000000000000000000000000000000000000000000000000000000000000000000".from_hex().unwrap();

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.version, IE6_VERSION);
        assert_eq!(config.counter, 58);
        assert_eq!(config.config.automatically_detect_settings, true);
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
    fn ie8_deserialize_no_proxy() {
        let data = "460000003A000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".from_hex().unwrap();

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.version, IE7_VERSION);
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
    fn ie8_deserialize_some_proxy() {
        let data ="46000000400000000F0000000D000000676F6F676C652E636F6D3A34320B0000004C6F6C3B3C6C6F63616C3E11000000687474703A2F2F676F6F676C652E66722F0000000000000000000000000000000000000000000000000000000000000000".from_hex().unwrap();

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.version, IE7_VERSION);
        assert_eq!(config.counter, 64);
        assert_eq!(config.config.automatically_detect_settings, true);
        assert_eq!(config.config.use_manual_proxy, true);
        assert_eq!(
            config.config.manual_proxy_address,
            String::from("google.com:42")
        );
        assert_eq!(
            config.config.manual_proxy_bypass_list,
            String::from("Lol;<local>")
        );
        assert_eq!(config.config.use_setup_script, true);
        assert_eq!(
            config.config.setup_script_address,
            String::from("http://google.fr/")
        );

        let mut roundtrip = Vec::new();
        serialize(&config, &mut roundtrip).unwrap();
        assert_eq!(roundtrip, data);
    }

    #[test]
    fn ie8_deserialize_some_proxy_not_used() {
        let data ="4600000044000000010000000D000000676F6F676C652E636F6D3A34320B0000004C6F6C3B3C6C6F63616C3E11000000687474703A2F2F676F6F676C652E66722F0000000000000000000000000000000000000000000000000000000000000000".from_hex().unwrap();

        let config = deserialize(&data[..]).unwrap();
        assert_eq!(config.version, IE7_VERSION);
        assert_eq!(config.counter, 68);
        assert_eq!(config.config.automatically_detect_settings, false);
        assert_eq!(config.config.use_manual_proxy, false);
        assert_eq!(
            config.config.manual_proxy_address,
            String::from("google.com:42")
        );
        assert_eq!(
            config.config.manual_proxy_bypass_list,
            String::from("Lol;<local>")
        );
        assert_eq!(config.config.use_setup_script, false);
        assert_eq!(
            config.config.setup_script_address,
            String::from("http://google.fr/")
        );

        let mut roundtrip = Vec::new();
        serialize(&config, &mut roundtrip).unwrap();
        assert_eq!(roundtrip, data);
    }
}
