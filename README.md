# ProxyConf

A small command line tool to set proxy configuration on Windows.

## Why does it exists

Using command-line only windows version (Core or Nano) behind a proxy can be problematic: some windows applications use IE settings but there is no good way to set them, other use WinHTTP settings that `netsh winhttp` can configure but it's not available on latest nano server images.

`proxyconf` aim to be a simple command line tool that can do this configuration and is suitable to use in containers (Docker).

## Sample usage

```dockerfile
FROM microsoft/nanoserver:1803

COPY proxyconf.exe .
RUN proxyconf set proxy 10.0.0.1:8080 "*.my-company;<local>"
```

## Command line

* `proxyconf show` Show the current proxy configuration
* `proxyconf set` Set the proxy configuration. One of theses must be specified:
  * `no-proxy` No proxy should be used.
  * `auto-detect` Automatically detect settings. Windows will use the [Web Proxy Autodiscovery Protocol][wpad] or fallback to direct connection
  * `proxy <ADDRESS:PORT> [BYPASS_LIST]`
    * `ADDRESS:PORT` The proxy server and optional port to use.
    * `BYPASS_LIST` Optional list of addresses that bypass the proxy separated by semicolons (`;`).<br/>
      Use `<local>` to bypass all short name hosts.<br/>
      Default to `<local>` if not specified.
  * `setup-script <URL>` Use [Proxy auto-config (PAC)][pac] from the specified URL.

[wpad]: https://en.wikipedia.org/wiki/Web_Proxy_Auto-Discovery_Protocol
[pac]: https://en.wikipedia.org/wiki/Proxy_auto-config

## Technical information

This tool set some specific keys in the registry, with the following format:

### Internet explorer settings

They are stored in the registry key `HKCU:SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings\Connections` in the REG_BINARY `DefaultConnectionSettings` value.

The (undocumented) format is (With all values in little endian):
* `version: u32` Seem to be a version number *unconfirmed*.
* `counter: u32` An incremential counter used to detect changes.
* `configuration_bits: u32` A bit field with the following bits:
  1. Always set
  2. Use manual proxy
  3. Use setup script
  4. Automatically detect settings
* `proxy_address_len: u32` Length of the proxy address string.
* `proxy_addres: Vec<u8>` Proxy address (ASCII).
* `bypass_list_len: u32` Length of the bypass list string.
* `bypass_list: Vec<u8>` Bypass list (ASCII).
* `setup_script_len: u32` Length of the setup script url string.
* `setup_script: Vec<u8>` Setup script url (ASCII).
