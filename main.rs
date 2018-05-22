extern crate byteorder;
extern crate winreg;

use std::io;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;

mod proxyconf {
    #[derive(Debug)]
    pub struct ProxyConfig {
        automatically_detect_settings: bool,
        use_setup_script: bool,
        setup_script_address: String,
        use_manual_proxy: bool,
        manual_proxy_address: String,
        manual_proxy_overrides: String,
    }

    pub fn empty() -> ProxyConfig {
        ProxyConfig {
            automatically_detect_settings: false,
            use_setup_script: false,
            setup_script_address: String::from(""),
            use_manual_proxy: false,
            manual_proxy_address: String::from(""),
            manual_proxy_overrides: String::from(""),
        }
    }

    #[derive(Debug)]
    pub struct FullConfig {
        counter: u32,
        config: ProxyConfig,
    }

    mod serialization {
        use proxyconf;
        use std;
        use std::{io, error, fmt};
        use std::io::{BufWriter, BufReader, Write, Read};
        use std::error::Error;
        use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};

        fn mk_bit_field(config: &proxyconf::FullConfig) -> u32 {
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

        #[derive(Debug)]
        pub struct FromUsizeError {
            invalid_size: usize
        }

        impl fmt::Display for FromUsizeError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "usize is too big to become an u32: {}", self.invalid_size)
            }
        }

        impl error::Error for FromUsizeError {
            fn description(&self) -> &str {
                return "usize is too big to become an u32";
            }

            fn cause(&self) -> Option<&error::Error> {
                None
            }
        }

        fn usize_to_u32(a: usize) -> Result<u32, io::Error> {
            if a > std::u32::MAX as usize {
                let e = FromUsizeError { invalid_size: a };
                return Err(io::Error::new(io::ErrorKind::InvalidData, e));
            } else {
                Ok(a as u32)
            }
        }

        fn write_string<W: Write>(stream: &mut W, s: &str) -> io::Result<()> {
            stream.write_u32::<LittleEndian>(usize_to_u32(s.len())?)?;
            stream.write_all(s.as_bytes())?;
            return Ok(());
        }

        pub fn serialize<W: Write>(
            config: &proxyconf::FullConfig,
            stream: &mut W
        ) -> io::Result<()> {
            let mut buffered = BufWriter::new(stream);

            buffered.write_u32::<LittleEndian>(0x46u32)?;
            buffered.write_u32::<LittleEndian>(config.counter)?;
            buffered.write_u32::<LittleEndian>(mk_bit_field(&config))?;

            write_string(&mut buffered, &config.config.manual_proxy_address)?;
            write_string(&mut buffered, &config.config.manual_proxy_overrides)?;
            write_string(&mut buffered, &config.config.setup_script_address)?;

            for i in 0..32 {
                buffered.write_u8(0)?;
            }

            return Ok(());
        }

        #[derive(Debug)]
        pub enum DeserializeError {
            IoError(io::Error),
            Utf8Error(std::str::Utf8Error),
            InvalidVersionError(u32),
        }

        impl fmt::Display for DeserializeError {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("DeserializeError: ")?;
                match *self {
                    DeserializeError::IoError(ref err) => err.fmt(fmt)?,
                    DeserializeError::Utf8Error(ref err) => err.fmt(fmt)?,
                    DeserializeError::InvalidVersionError(v) => write!(fmt, "Invalid version {}", v)?,
                }
                Ok(())
            }
        }

        impl Error for DeserializeError {
            fn description(&self) -> &str {
                match *self {
                    DeserializeError::IoError(ref err) => err.description(),
                    DeserializeError::Utf8Error(ref err) => err.description(),
                    DeserializeError::InvalidVersionError(_) => "Invalid version",
                }
            }

            fn cause(&self) -> Option<&Error> {
                match *self {
                    DeserializeError::IoError(ref error) => Some(error),
                    DeserializeError::Utf8Error(ref error) => Some(error),
                    _ => None,
                }
            }
        }

        impl From<io::Error> for DeserializeError {
            fn from(err: io::Error) -> DeserializeError {
                DeserializeError::IoError(err)
            }
        }

        impl From<std::str::Utf8Error> for DeserializeError {
            fn from(err: std::str::Utf8Error) -> DeserializeError {
                DeserializeError::Utf8Error(err)
            }
        }

        fn read_string<R: Read>(stream: &mut R) -> Result<String, DeserializeError> {
            let len = stream.read_u32::<LittleEndian>()?;
            let mut bytes = vec![0; len as usize];
            stream.read_exact(&mut bytes)?;

            let s = std::str::from_utf8(&bytes)?;
            return Ok(String::from(s));
        }

        pub fn deserialize<R: Read>(
            stream: &mut R
        ) -> Result<proxyconf::FullConfig, DeserializeError> {
            let mut buffered = BufReader::new(stream);

            let version = buffered.read_u32::<LittleEndian>()?;
            if version != 0x46u32 {
                return Err(DeserializeError::InvalidVersionError(version))
            }

            let counter = buffered.read_u32::<LittleEndian>()?;
            let conf = buffered.read_u32::<LittleEndian>()?;

            let automatically_detect_settings = (conf & 0x08) != 0x00;
            let use_setup_script = (conf & 0x04) != 0x00;
            let use_manual_proxy = (conf & 0x02) != 0x00;

            let manual_proxy_address = read_string(&mut buffered)?;
            let manual_proxy_overrides = read_string(&mut buffered)?;
            let setup_script_address = read_string(&mut buffered)?;

            let config = proxyconf::FullConfig {
                counter,
                config: proxyconf::ProxyConfig {
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
    }
}

fn main() {
    println!("Reading some system info...");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")
        .unwrap();
    let pf: String = cur_ver.get_value("ProgramFilesDir").unwrap();
    let dp: String = cur_ver.get_value("DevicePath").unwrap();
    println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);
    let info = cur_ver.query_info().unwrap();
    println!("info = {:?}", info);

    println!("And now lets write something...");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software").join("WinregRsExample1");
    let key = hkcu.create_subkey(&path).unwrap();

    key.set_value("TestSZ", &"written by Rust").unwrap();
    let sz_val: String = key.get_value("TestSZ").unwrap();
    key.delete_value("TestSZ").unwrap();
    println!("TestSZ = {}", sz_val);

    key.set_value("TestDWORD", &1234567890u32).unwrap();
    let dword_val: u32 = key.get_value("TestDWORD").unwrap();
    println!("TestDWORD = {}", dword_val);

    key.set_value("TestQWORD", &1234567891011121314u64).unwrap();
    let qword_val: u64 = key.get_value("TestQWORD").unwrap();
    println!("TestQWORD = {}", qword_val);

    key.create_subkey("sub\\key").unwrap();
    hkcu.delete_subkey_all(&path).unwrap();

    println!("Trying to open nonexistent key...");
    let key2 = hkcu.open_subkey(&path).unwrap_or_else(|e| match e.kind() {
        io::ErrorKind::NotFound => panic!("Key doesn't exist"),
        io::ErrorKind::PermissionDenied => panic!("Access denied"),
        _ => panic!("{:?}", e),
    });
}
