use crate::EmailConfig;
use crate::email::{async_send_email, send_email};

#[derive(Clone, Debug)]
/// Builder pattern for sending emails
pub struct EmailClient {
    pub config: EmailConfig,
    pub recipient: Vec<String>,
    pub subject: String,
    pub body: Option<String>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub attachment: Option<String>,
}

impl Default for EmailClient {
    fn default() -> Self {
        EmailClient {
            config: EmailConfig::from_env(),
            recipient: vec![],
            subject: "No subject".to_string(),
            body: None,
            cc: None,
            bcc: None,
            attachment: None,
        }
    }
}

impl EmailClient {
    /// Creates a new EmailClient with the given configuration
    /// # Arguments
    /// * `config` - Email configuration
    pub fn new(config: impl Into<EmailConfig>) -> Self {
        EmailClient {
            config: config.into(),
            ..Self::default()
        }
    }

    /// Sets the email configuration
    /// # Arguments
    /// * `config` - Email configuration
    pub fn config(mut self, config: impl Into<EmailConfig>) -> Self {
        self.config = config.into();
        self
    }

    /// Sets the recipient(s) of the email
    /// # Arguments
    /// * `recipient` - A vector of recipient email addresses
    pub fn recipient(mut self, recipient: Vec<impl Into<String>>) -> Self {
        self.recipient = recipient.into_iter().map(|r| r.into()).collect();
        self
    }

    /// Sets the subject of the email
    /// # Arguments
    /// * `subject` - Subject of the email
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = subject.into();
        self
    }

    /// Sets the body of the email
    /// # Arguments
    /// * `body` - Body content of the email
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Sets the CC recipients of the email
    /// # Arguments
    /// * `cc` - A vector of CC email addresses
    pub fn cc(mut self, cc: Vec<String>) -> Self {
        self.cc = Some(cc);
        self
    }

    /// Sets the BCC recipients of the email
    /// # Arguments
    /// * `bcc` - A vector of BCC email addresses
    pub fn bcc(mut self, bcc: Vec<String>) -> Self {
        self.bcc = Some(bcc);
        self
    }

    /// Sets the attachment of the email
    /// # Arguments
    /// * `attachment` - File path of the attachment
    pub fn attachment(mut self, attachment: impl Into<String>) -> Self {
        self.attachment = Some(attachment.into());
        self
    }

    /// Sends the email synchronously
    /// # Returns
    /// * `anyhow::Result<()>` - Result of the email sending operation
    pub fn send(self) -> anyhow::Result<()> {
        send_email(
            self.config,
            self.recipient,
            self.subject,
            self.body.unwrap_or_default(),
            self.cc,
            self.bcc,
            self.attachment,
        )
    }

    /// Sends the email asynchronously
    /// # Returns
    /// * `anyhow::Result<()>` - Result of the email sending operation
    pub async fn send_async(self) -> anyhow::Result<()> {
        async_send_email(
            self.config,
            self.recipient,
            self.subject,
            self.body.unwrap_or_default(),
            self.cc,
            self.bcc,
            self.attachment,
        )
        .await
    }
}
