use anyhow::Result;
#[cfg(feature = "python")]
use pyo3::{pyclass, pymethods};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::email::send_email;

#[derive(Clone, Debug)]
pub struct EmailBuilder {
    config: EmailConfig,
    recipient: Vec<String>,
    subject: Option<String>,
    body: Option<String>,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    attachment: Option<String>,
}

impl EmailBuilder {
    pub fn new(config: EmailConfig, recipient: Vec<String>) -> Self {
        EmailBuilder {
            config,
            recipient,
            subject: None,
            body: None,
            cc: None,
            bcc: None,
            attachment: None,
        }
    }

    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn cc(mut self, cc: Vec<String>) -> Self {
        self.cc = Some(cc);
        self
    }

    pub fn bcc(mut self, bcc: Vec<String>) -> Self {
        self.bcc = Some(bcc);
        self
    }

    pub fn attachment(mut self, attachment: impl Into<String>) -> Self {
        self.attachment = Some(attachment.into());
        self
    }

    pub fn send(self) -> Result<()> {
        let subject = self.subject.unwrap_or_else(|| "No Subject".to_string());
        let body = self.body.unwrap_or_else(|| "No Body".to_string());

        send_email(
            self.config,
            self.recipient,
            subject,
            body,
            self.cc,
            self.bcc,
            self.attachment,
        )
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "python", pyclass(dict, get_all, set_all, str, subclass))]
#[derive(Serialize, Deserialize, Debug)]
pub struct EmailConfig {
    pub server: String,
    pub sender_email: String,
    pub username: String,
    pub password: String,
}

impl EmailConfig {
    pub fn new(server: &str, sender_email: &str, username: &str, password: &str) -> Self {
        EmailConfig {
            server: server.to_string(),
            sender_email: sender_email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn send_to(self, recipient: Vec<String>) -> EmailBuilder {
        EmailBuilder::new(self, recipient)
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl EmailConfig {
    #[new]
    #[pyo3(signature = (server, sender_email, username, password))]
    pub fn py_new(server: &str, sender_email: &str, username: &str, password: &str) -> Self {
        Self::new(server, sender_email, username, password)
    }
}

impl fmt::Display for EmailConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EmailConfig<server={}, sender_email={}, username={}, password={}>",
            self.server, self.sender_email, self.username, self.password
        )
    }
}
