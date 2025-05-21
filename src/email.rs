use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use lettre::message::header::ContentType;
use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use lettre::{Message, SmtpTransport, Transport};
use pyo3::PyErr;

use crate::email_config::EmailConfig;

fn arg_check(config: &EmailConfig, recipient: &Vec<String>) -> Result<bool> {
    let server = config.server.as_str();

    if server == "smtp.example.com" || server == "" {
        return Err(anyhow!("Config not set"));
    }
    if recipient.is_empty() {
        return Err(anyhow!("No recipient"));
    }

    Ok(true)
}

fn msg_builder(
    from: String,
    recipient: Vec<String>,
    subject: String,
    body: String,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    attachment: Option<String>,
) -> Result<Message> {
    let from_email = from.parse::<Mailbox>()?;
    let mut email_builder = Message::builder().from(from_email).subject(subject);

    for each_recipient in recipient {
        let recipient_email = each_recipient.parse::<Mailbox>()?;
        email_builder = email_builder.to(recipient_email);
    }

    match cc {
        Some(cc) => {
            for each_cc in cc {
                let cc_email = each_cc.parse::<Mailbox>()?;
                email_builder = email_builder.cc(cc_email);
            }
        }
        None => {}
    };

    match bcc {
        Some(bcc) => {
            for each_bcc in bcc {
                let bcc_email = each_bcc.parse::<Mailbox>()?;
                email_builder = email_builder.bcc(bcc_email);
            }
        }
        None => {}
    }

    let mut multipart_builder = MultiPart::mixed()
        .multipart(MultiPart::alternative().singlepart(SinglePart::html(String::from(body))));

    match attachment {
        Some(attachment) => {
            let attachment_path = PathBuf::from(attachment);
            if !attachment_path.exists() {
                return Err(anyhow!("Attachment not found"));
            }
            if attachment_path.is_dir() {
                return Err(anyhow!("Attachment is a directory"));
            }
            let attachment_body = fs::read(&attachment_path)?;
            let attachment_content_type =
                mime_guess::from_path(&attachment_path).first_or_text_plain();
            let content_type = ContentType::parse(&attachment_content_type.to_string())?;
            let attachment_part = Attachment::new(attachment_path.to_string_lossy().to_string())
                .body(attachment_body, content_type);
            multipart_builder = multipart_builder.singlepart(attachment_part);
        }
        None => {}
    }

    Ok(email_builder.multipart(multipart_builder)?)
}

pub fn send_email(
    config: EmailConfig,
    recipient: Vec<String>,
    subject: String,
    body: String,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    attachment: Option<String>,
) -> Result<()> {
    arg_check(&config, &recipient)?;

    let email = msg_builder(
        config.sender_email,
        recipient,
        subject,
        body,
        cc,
        bcc,
        attachment,
    )?;

    // Open a remote connection to the SMTP server with STARTTLS
    let mailer = SmtpTransport::starttls_relay(config.server.as_str())?
        .credentials(Credentials::new(
            config.username.to_string(),
            config.password.to_string(),
        ))
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Error sending email, {}", e)),
    }
}

pub async fn async_send_email(
    config: EmailConfig,
    recipient: Vec<String>,
    subject: String,
    body: String,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    attachment: Option<String>,
) -> Result<(), PyErr> {
    let _: bool = match arg_check(&config, &recipient) {
        Ok(b) => b,
        Err(e) => return Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
    };

    let email = match msg_builder(
        config.sender_email,
        recipient,
        subject,
        body,
        cc,
        bcc,
        attachment,
    ) {
        Ok(email) => email,
        Err(e) => return Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
    };

    let mailer = match AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(config.server.as_str())
    {
        Ok(mailer) => mailer
            .credentials(Credentials::new(
                config.username.to_string(),
                config.password.to_string(),
            ))
            .build(),
        Err(e) => return Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
    };

    match mailer.send(email).await {
        Ok(_) => Ok(()),
        Err(e) => Err(pyo3::exceptions::PyException::new_err(format!(
            "Error sending email, {}",
            e
        ))),
    }
}
