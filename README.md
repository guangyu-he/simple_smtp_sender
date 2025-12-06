# simple_smtp_sender

A Simple SMTP Email sender crate with the support of sync or async sending.
Can be called from Python. Powered by Rust, [lettre](https://lettre.rs/)
and [PyO3](https://github.com/PyO3/pyo3).

## Overview

This project provides rust crate and a Python extension module implemented in Rust for sending emails via SMTP,
including support for
attachments, CC, and BCC. There are methods for both synchronous and asynchronous sending.
It leverages the performance and safety of Rust, exposes a convenient Python API, and is built
using [PyO3](https://github.com/PyO3/pyo3) and [lettre](https://lettre.rs/).
The python module is compatible with Python 3.10 and above.

## Features

- Send emails via SMTP synchronously or asynchronously
- Support HTML email contents
- Attach files to emails
- Support for CC and BCC
- Secure authentication
- Easy configuration via Python class
- Flexible feature flags for Rust-only or Python-enabled builds
- No Python dependencies required for Rust-only usage

## Installation

### Python Package from PyPI

```bash
uv pip install simple_smtp_sender
# or
pip install simple_smtp_sender
```

### Rust Crate

Please note that the default feature include the Python PyO3 binding, to use the crate as native
rust pacakge, please declare the dependency with `default-feature=false`:

```toml
[dependencies]
# Rust-only version (no Python dependencies)
simple_smtp_sender = { version = "0.2.6", default-features = false }
```

### Build Python package from Source (requires Rust toolchain and maturin)

```bash
git clone https://github.com/guangyu-he/simple_smtp_sender.git
cd simple_smtp_sender
## prepare venv and maturin if needed
# uv venv
# uv sync
uv run maturin develop
```

Or build a wheel:

```bash
uv run maturin build
pip install target/wheels/simple_smtp_sender-*.whl
```

### Requirements

- Python >= 3.10 (for Python package)
- Rust toolchain (for building from source)

## Usage

An example test from Rust crate:

**Note**: this test can only be run natively if you installed with
`default-features = false`

```rust
use simple_smtp_sender::{send_email_async, send_email_sync, EmailConfig};

#[test]
fn send_email_sync_test() {
    let config = EmailConfig::new(
        "smtp.example.com",
        "your@email.com",
        "your_username",
        "your_password",
    );
    let result = send_email_sync(config, vec!["recipient@email.com".to_string()], "Test Email", "Hello from Rust!", None, None, None);
    assert!(result.is_ok());
}

#[tokio::test]
fn send_email_async_test() {
    let config = EmailConfig::new(
        "smtp.example.com",
        "your@email.com",
        "your_username",
        "your_password",
    );
    let result = send_email_async(config, vec!["recipient@email.com".to_string()], "Test Email", "Hello from Rust!", None, None, None).await;
    assert!(result.is_ok());
}

//new builder method
#[test]
fn send_email_sync_builder_test() {
    let config = EmailConfig::new(
        "smtp.example.com",
        "your@email.com",
        "your_username",
        "your_password",
    );
    let result = config
        .send_to(vec!["recipient@email.com".to_string()])
        .subject("Test Email Builder")
        .body("Hello from Rust Email Builder!")
        .send();
    assert!(result.is_ok());
}

```

If you installed the Python feature (default), you can test the above codes using for example:

```shell
cargo test --no-default-features send_email_sync_test
```

An example from Python API:

```python
from simple_smtp_sender import EmailConfig, send_email, async_send_email

config = EmailConfig(
    server="smtp.example.com",
    sender_email="your@email.com",
    username="your_username",
    password="your_password",
)

# Synchronous send (blocking)
send_email(
    config,
    recipient=["recipient@email.com"],
    subject="Test Email",
    body="Hello from Rust!",
)

# With attachment, CC, and BCC:
send_email(
    config,
    recipient=["recipient@email.com"],
    subject="With Attachment",
    body="See attached file.",
    cc=["cc@email.com"],
    bcc=["bcc@email.com"],
    attachment="/path/to/file.pdf",
)

# Asynchronous send (non-blocking)
import asyncio


async def main():
    await async_send_email(
        config,
        recipient=["recipient@email.com"],
        subject="Async Email",
        body="Sent asynchronously!",
    )


asyncio.run(main())

```

## API

### `EmailConfig`

Configuration class for SMTP server and credentials.

- `server`: SMTP server URL
- `sender_email`: Sender's email address
- `username`: SMTP username
- `password`: SMTP password

### `send_email(config, recipient, subject, body, cc=None, bcc=None, attachment=None)`

Sends an email synchronously (blocking) using the provided configuration.

- `config`: `EmailConfig` instance
- `recipient`: List of recipient email(s)
- `subject`: Email subject
- `body`: Email body
- `cc`: List of CC recipients (optional)
- `bcc`: List of BCC recipients (optional)
- `attachment`: Path to file to attach (optional)

### `async_send_email(config, recipient, subject, body, cc=None, bcc=None, attachment=None)`

Sends an email asynchronously (non-blocking, returns an awaitable).

- `config`: `EmailConfig` instance
- `recipient`: List of recipient email(s)
- `subject`: Email subject
- `body`: Email body
- `cc`: List of CC recipients (optional)
- `bcc`: List of BCC recipients (optional)
- `attachment`: Path to file to attach (optional)

## Development

- Rust dependencies are managed in `Cargo.toml`.
- Python build configuration is in `pyproject.toml`.
- Main Rust logic in `src/`.

## License

MIT
