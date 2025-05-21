use std::fmt;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use lettre::message::header::ContentType;
use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use pyo3::{pyclass, pymethods};

#[derive(Clone)]
#[pyclass(dict, get_all, set_all, str, subclass)]
pub struct EmailConfig {
    pub server: String,
    pub from_email: String,
    pub username: String,
    pub password: String,
}

#[pymethods]
impl EmailConfig {
    #[new]
    #[pyo3(signature = (server, sender_email, username, password))]
    pub fn new(server: &str, sender_email: &str, username: &str, password: &str) -> Self {
        EmailConfig {
            server: server.to_string(),
            from_email: sender_email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl fmt::Display for EmailConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EmailConfig<server={}, from_email={}, username={}, password={}>",
            self.server, self.from_email, self.username, self.password
        )
    }
}

pub fn send_email(
    config: EmailConfig,
    recipient: &str,
    subject: &str,
    body: &str,
    cc: Option<&str>,
    bcc: Option<&str>,
    attachment: Option<&str>,
) -> Result<()> {
    let server = config.server.as_str();
    if server == "smtp.example.com" || server == "" {
        return Err(anyhow!("Config not set"));
    }
    let from_email_str = config.from_email.as_str();
    let from_email = from_email_str.parse::<Mailbox>()?;
    let creds = Credentials::new(config.username, config.password);
    let to_email = recipient.parse::<Mailbox>()?;

    let mut email_builder = Message::builder()
        .from(from_email)
        .to(to_email)
        .subject(subject);

    let cc_str = cc.unwrap_or("");
    if !cc_str.is_empty() {
        let cc_email = cc_str.parse::<Mailbox>()?;
        email_builder = email_builder.cc(cc_email);
    }

    let bcc_str = bcc.unwrap_or("");
    if !bcc_str.is_empty() {
        let bcc_email = bcc_str.parse::<Mailbox>()?;
        email_builder = email_builder.bcc(bcc_email);
    }

    let mut multipart_builder = MultiPart::mixed()
        .multipart(MultiPart::alternative().singlepart(SinglePart::html(String::from(body))));

    let attachment_str = attachment.unwrap_or("");
    if !attachment_str.is_empty() {
        let attachment_path = PathBuf::from(attachment.unwrap_or(""));
        let attachment_name = attachment_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let attachment_body = fs::read(attachment_path)?;
        let attachment_content_type = mime_guess::from_path(&attachment_name).first_or_text_plain();
        let content_type = ContentType::parse(&attachment_content_type.to_string())?;
        let attachment_part = Attachment::new(attachment_name).body(attachment_body, content_type);
        multipart_builder = multipart_builder.singlepart(attachment_part);
    }

    let email = email_builder.multipart(multipart_builder)?;

    // Open a remote connection to the SMTP server with STARTTLS
    let mailer = SmtpTransport::starttls_relay(server)?
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Error sending email, {}", e)),
    }
}
