# simple_smtp_sender

A Simple SMTP Email Sender with the support of sync or async sending.
Powered by powered by Rust, [lettre](https://lettre.rs/) and [PyO3](https://github.com/PyO3/pyo3).

## Overview

This project provides a Python extension module implemented in Rust for sending emails via SMTP, including support for
attachments, CC, and BCC. There are methods for both synchronous and asynchronous sending.
It leverages the performance and safety of Rust, exposes a convenient Python API, and is built
using [PyO3](https://github.com/PyO3/pyo3) and [lettre](https://lettre.rs/).
The module is compatible with Python 3.10 and above.

## Features

- Send emails via SMTP synchronously or asynchronously from Python
- Support HTML email contents
- Attach files to emails
- Support for CC and BCC
- Secure authentication
- Easy configuration via Python class

## Installation

### from PyPI

```bash
pip install simple_smtp_sender
```

### from source (requires Rust toolchain and maturin)

```bash
git clone https://github.com/guangyu-he/simple_smtp_sender.git
cd simple_smtp_sender
# prepare venv and maturin if needed
maturin build
```

### Requirements

- Python >= 3.10
- Rust toolchain (for building)

## Usage

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
