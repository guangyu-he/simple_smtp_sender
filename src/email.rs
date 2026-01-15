use std::fs;
use std::path::PathBuf;

use anyhow::{Result, anyhow};

use lettre::message::header::ContentType;
use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use lettre::{Message, SmtpTransport, Transport};

use crate::email_config::EmailConfig;

/// Check the required arguments for sending an email
/// # Arguments
/// * `server` - SMTP server address
/// * `recipient` - A vector of recipient email addresses
/// # Returns
/// * `anyhow::Result<()>` - Result of the argument check
fn arg_check<S: AsRef<str>, R: AsRef<str>>(server: S, recipient: &[R]) -> Result<()> {
    if server.as_ref().is_empty() {
        anyhow::bail!("No SMTP server provided");
    }
    if recipient.is_empty() {
        anyhow::bail!("No recipient provided");
    }

    Ok(())
}

/// Build the email message
/// # Arguments
/// * `from` - Sender email address
/// * `recipient` - A vector of recipient email addresses
/// * `subject` - Email subject
/// * `body` - Email body
/// * `cc` - Optional vector of CC email addresses
/// * `bcc` - Optional vector of BCC email addresses
/// * `attachment` - Optional attachment file path
/// # Returns
/// * `anyhow::Result<Message>` - Resulting email message
fn msg_builder<
    S: Into<String>,
    E: AsRef<str>,
    R: AsRef<str>,
    B: Into<String>,
    A: Into<PathBuf>,
    I: IntoIterator<Item = R>,
>(
    from: E,
    recipient: I,
    subject: S,
    body: B,
    cc: Option<I>,
    bcc: Option<I>,
    attachment: Option<A>,
) -> Result<Message> {
    let from_email = from.as_ref().parse::<Mailbox>()?;
    let mut email_builder = Message::builder().from(from_email).subject(subject);

    for each_recipient in recipient {
        let recipient_email = each_recipient.as_ref().parse::<Mailbox>()?;
        email_builder = email_builder.to(recipient_email);
    }

    match cc {
        Some(cc) => {
            for each_cc in cc {
                let cc_email = each_cc.as_ref().parse::<Mailbox>()?;
                email_builder = email_builder.cc(cc_email);
            }
        }
        None => {}
    };

    match bcc {
        Some(bcc) => {
            for each_bcc in bcc {
                let bcc_email = each_bcc.as_ref().parse::<Mailbox>()?;
                email_builder = email_builder.bcc(bcc_email);
            }
        }
        None => {}
    }

    let mut multipart_builder = MultiPart::mixed()
        .multipart(MultiPart::alternative().singlepart(SinglePart::html(body.into())));

    match attachment {
        Some(attachment) => {
            let attachment_path = attachment.into();
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
            let filename = attachment_path
                .file_name()
                .ok_or_else(|| anyhow!("Invalid attachment path"))?
                .to_string_lossy()
                .to_string();
            let attachment_part = Attachment::new(filename).body(attachment_body, content_type);
            multipart_builder = multipart_builder.singlepart(attachment_part);
        }
        None => {}
    }

    Ok(email_builder.multipart(multipart_builder)?)
}

/// Synchronous function to send an email
/// # Arguments
/// * `config` - Email configuration
/// * `recipient` - A vector of recipient email addresses
/// * `subject` - Email subject
/// * `body` - Email body
/// * `cc` - Optional vector of CC email addresses
/// * `bcc` - Optional vector of BCC email addresses
/// * `attachment` - Optional attachment file path
/// # Returns
/// * `anyhow::Result<()>` - Result of the email sending operation
pub fn send_email<S: Into<String>, R: AsRef<str>, B: Into<String>, A: Into<PathBuf>>(
    config: EmailConfig,
    recipient: Vec<R>,
    subject: S,
    body: B,
    cc: Option<Vec<R>>,
    bcc: Option<Vec<R>>,
    attachment: Option<A>,
) -> Result<()> {
    arg_check(&config.username, &recipient)?;

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
    let mailer = SmtpTransport::starttls_relay(config.server.as_ref())?
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

/// Asynchronous function to send an email
/// # Arguments
/// * `config` - Email configuration
/// * `recipient` - A vector of recipient email addresses
/// * `subject` - Email subject
/// * `body` - Email body
/// * `cc` - Optional vector of CC email addresses
/// * `bcc` - Optional vector of BCC email addresses
/// * `attachment` - Optional attachment file path
/// # Returns
/// * `anyhow::Result<()>` - Result of the email sending operation
pub async fn async_send_email<S: Into<String>, R: AsRef<str>, B: Into<String>, A: Into<PathBuf>>(
    config: EmailConfig,
    recipient: Vec<R>,
    subject: S,
    body: B,
    cc: Option<Vec<R>>,
    bcc: Option<Vec<R>>,
    attachment: Option<A>,
) -> Result<()> {
    arg_check(&config.server, &recipient)?;

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
        Err(e) => return Err(anyhow!(e)),
    };

    let mailer = match AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(config.server.as_ref())
    {
        Ok(mailer) => mailer
            .credentials(Credentials::new(config.username, config.password))
            .build(),
        Err(e) => return Err(anyhow!(e)),
    };

    match mailer.send(email).await {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e)),
    }
}
