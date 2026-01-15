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

```toml
[dependencies]
# Rust-only version (no Python dependencies)
simple_smtp_sender = { version = "0.3.1" }
```

### Build Python package from Source (requires Rust toolchain and maturin)

Please note that the default feature *does not* include the Python PyO3 binding.
To build Python binding using maturin, please declare the dependency with `--features python`:

```bash
git clone https://github.com/guangyu-he/simple_smtp_sender.git
cd simple_smtp_sender
## prepare venv and maturin if needed
# uv venv
# uv sync
uv run maturin develop --features python
```

Or build a wheel:

```bash
uv run maturin build --features python
pip install target/wheels/simple_smtp_sender-*.whl
```

### Requirements

- Python >= 3.10 (for Python package)
- Rust toolchain (for building from source)

## Usage

### Rust

check tests.rs in `tests/` for more examples.

### Python

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

`__new__` parameters:

- `server`: SMTP server URL
- `sender_email`: Sender's email address
- `username`: SMTP username
- `password`: SMTP password

APIs:

- `load_from_env()`: Load configuration from environment variables:
    - `EMAIL_SERVER`
    - `EMAIL_SENDER_EMAIL`
    - `EMAIL_USERNAME`
    - `EMAIL_PASSWORD`
- `load_from_map(config_map: dict)`: Load configuration from a dictionary.

### Sends an email synchronously (blocking) using the provided configuration.

`send_email(config, recipient, subject, body, cc=None, bcc=None, attachment=None)`

- `config`: `EmailConfig` instance
- `recipient`: List of recipient email(s)
- `subject`: Email subject
- `body`: Email body
- `cc`: List of CC recipients (optional)
- `bcc`: List of BCC recipients (optional)
- `attachment`: Path to file to attach (optional)

### Sends an email asynchronously (non-blocking, returns an awaitable).

`async_send_email(config, recipient, subject, body, cc=None, bcc=None, attachment=None)`

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
