# ProxyConf

A small command line tool to set proxy configuration on Windows.

## Why does it exists

Using command-line only windows version (Core or Nano) behind a proxy can be problematic: some windows applications use IE settings but there is no good way to set them, other use WinHTTP settings that `netsh winhttp` can configure but it's not available on latest nano server images.

`proxyconf` aim to be a simple command line tool that can do this configuration and is suitable to use in containers (Docker).

## Sample usage

```dockerfile
FROM microsoft/nanoserver:1803

COPY proxyconf.exe .
RUN proxyconf set --proxy-server 10.0.0.1:8080 --bypass-list "*.my-company;<local>"
```

## Command line

* `proxyconf show` Show the current proxy configuration
* `proxyconf set` Set the proxy configuration
  * `--proxy-server` / `-s` The proxy server and optional port to use in the `SERVER:PORT` format.
  * `--bypass-list` / `-b` The list of addresses that bypass the proxy separated by semicolons (`;`). Use `<local>` to bypass all short name hosts.