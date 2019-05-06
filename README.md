# ProxyConf

[![License][license-badge]](LICENSE)
[![crates.io][crate-badge]][crate]
[![Docs][docs-badge]][docs]
[![Azure Pipelines Status][pipelines-badge]][pipelines]

[license-badge]: https://img.shields.io/badge/License-MIT-green.svg?longCache=true
[crate-badge]: https://img.shields.io/badge/crates.io-v0.2.1-orange.svg?longCache=true 
[crate]: https://crates.io/crates/proxyconf
[docs-badge]: https://docs.rs/proxyconf/badge.svg?version=0.2.1
[docs]: https://docs.rs/proxyconf
[pipelines-badge]: https://vbfox.visualstudio.com/ProxyConf/_apis/build/status/ProxyConf%20YAML?branchName=master
[pipelines]: https://vbfox.visualstudio.com/ProxyConf/_build/latest?definitionId=2&branchName=master

A small command line tool to set proxy configuration on Windows.

## Why does it exists

Using command-line only windows version (Core or Nano) behind a proxy can be problematic: some windows applications use IE settings but there is no good way to set them, other use WinHTTP settings that `netsh winhttp` can configure but it's not available on latest nano server images.

`proxyconf` aim to be a simple command line tool that can do this configuration and is suitable to use in containers (Docker).

## Sample usage

```dockerfile
FROM microsoft/nanoserver:1803

COPY proxyconf.exe .
RUN proxyconf set proxy 10.0.0.1:8080 "*.my-company;<local>" \
    && proxyconf winhttp proxy 10.0.0.1:8080 "*.my-company;<local>"
```

## Command line

* `proxyconf show` Show the current proxy configuration
* `proxyconf set` Set the current user proxy configuration. One of theses must be specified:
  * `no-proxy` No proxy should be used.
  * `auto-detect` Automatically detect settings. Windows will use the [Web Proxy Autodiscovery Protocol][wpad] or fallback to direct connection
  * `proxy <ADDRESS:PORT> [BYPASS_LIST]` Use an hardcoded proxy (or proxies, see below) with an optional bypass list.
  * `setup-script <URL>` Use [Proxy auto-config (PAC)][pac] from the specified URL.
* `proxyconf winhttp` Set the system-wide winhttp proxy configuration. One of theses must be specified:
  * `no-proxy` No proxy should be used.
  * `proxy <ADDRESS:PORT> [BYPASS_LIST]` Use an hardcoded proxy (or proxies, see below) with an optional bypass list.

Common values:
  * `ADDRESS:PORT` The proxy server and optional port to use.<br/>
    Can also be a list of `protocol=address:port` separated by semicolons (`;`). The protocols are `http`, `https`, `ftp` and `socks`. <br/>
  * `BYPASS_LIST` Optional list of addresses that bypass the proxy separated by semicolons (`;`).<br/>
  Use `<local>` to bypass all short name hosts.<br/>
  Default to `<local>` if not specified.

[wpad]: https://en.wikipedia.org/wiki/Web_Proxy_Auto-Discovery_Protocol
[pac]: https://en.wikipedia.org/wiki/Proxy_auto-config

## What can be changed

### Internet options

Per-user settings (by default, there is a registry flag to have system ones) used by Internet Explorer initially and now by most applications.

They can be edited in the GUI from Internet Explorer settings or on recent versions of Windows directly from the control panel.

### WinHTTP settings

A binary key storing settings for applications using the [WinHTTP][winhttp] programming interface. It doesn't support much more than a static proxy but it's supported by services like windows update.

[winhttp]: https://docs.microsoft.com/en-us/windows/desktop/WinHttp/about-winhttp

## Technical information

This tool set some specific keys in the registry, with the following format:

### Binary/Modern Internet Options

Stored as a REG_BINARY value.

* One key per connection exists under `HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings\Connections` with the default one for LAN under `DefaultConnectionSettings`.
* If `HKEY_LOCAL_MACHINE\Software\Policies\Microsoft\Windows\CurrentVersion\Internet Settings` key `ProxySettingsPerUser` is `1` (DWORD) the same location is used in `HKEY_LOCAL_MACHINE` instead and the settings are global.
* The WinHTTP settings use an old version in a value named `WinHttpSettings` under the key `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings\Connections`

I know of 3 versions:
* **0x28** Used by `netsh winhttp`
* **0x3C** Used in IE 6
* **0x46** Used since IE 7 and still used in IE 11 / Windows 10

The (undocumented) format is (With all values in little endian):
* `version: u32` Version number
* `counter: u32` An incremential counter used to detect changes.
* `configuration_bits: u32` A bit field with the following bits:
  * `1` Always set
  * `2` Use manual proxy
  * `3` Use setup script *(version >= 0x3C)*
  * `4` Automatically detect settings *(version >= 0x3C)*
* `proxy_address_len: u32` Length of the proxy address string.
* `proxy_addres: Vec<u8>` Proxy address (ASCII).
* `bypass_list_len: u32` Length of the bypass list string.
* `bypass_list: Vec<u8>` Bypass list (ASCII).
* `setup_script_len: u32` Length of the setup script url string. *(version >= 0x3C)*
* `setup_script: Vec<u8>` Setup script url (ASCII). *(version >= 0x3C)*
* 28 unknown bytes for version 0x3C, 32 for version 0x46

### Values/Legacy Internet Options

They aren't used anymore but are kept for applications that might access them directly, the GUI set them when a change is done but doesn't read it.
They are stored in the registry key `HKCU:SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings\Connections`.
