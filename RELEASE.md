# Release Notes

## v0.4.0

### Breaking Changes

- **Migrated from OpenSSL to Rustls**: TLS backend switched from `native-tls` (OpenSSL) to `rustls`. This removes the
  need for OpenSSL system libraries and the vendored OpenSSL dependency on Linux, simplifying cross-compilation and
  deployment.

### Changes

- Bumped `pyo3` from `0.27.2` to `0.28`
- Bumped `pyo3-async-runtimes` from `0.27.0` to `0.28`
- Bumped `tokio` (dev) from `1.48` to `1.49`
- Switched `lettre` TLS feature from `tokio1-native-tls` to `tokio1-rustls-tls`
- Removed vendored OpenSSL dependency for Linux builds

## v0.3.1

- Fixed dependency version specifications

## v0.3.0

- Initial public release
