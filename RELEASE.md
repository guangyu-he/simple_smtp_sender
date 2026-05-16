# Release Notes

## v0.4.1

### Breaking Changes (Python API)

- **`EmailConfig.load_from_pydantic()` signature change**: Return type changed
  from `Self | None` to `Self`. Errors are now surfaced as proper exceptions
  instead of being silently swallowed into `None`:
    - `RuntimeError` if `pydantic` cannot be imported from the active Python
      environment (the underlying import error is included in the message).
    - `TypeError` if the argument is not a `pydantic.BaseModel` instance (the
      actual type name is included in the message).
    - `ValueError` if the model's fields cannot be deserialized into an
      `EmailConfig` (e.g. missing or mistyped fields).

### Features (Python API)

- Added `EmailConfig.to_dict() -> dict[str, str]` for converting an
  `EmailConfig` instance into a Python dictionary.

### Internal / Build

- Gated the entire `email_config_py` module behind `#[cfg(feature = "python")]`
  at the `lib.rs` level, removing the repetitive per-item `cfg` attributes
  inside the module.
- Moved Rust test cases from `tests/` to `examples/examples.rs` for better
  organization, and removed the obsolete `[[test]]` entry from `Cargo.toml`.

### Dependencies

- Bumped `pyo3` to `0.28.3` (patch update).

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
