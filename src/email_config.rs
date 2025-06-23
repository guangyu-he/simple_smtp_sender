#[cfg(feature = "python")]
use pyo3::{pyclass, pymethods};
use serde::{Deserialize, Serialize};
use std::fmt;

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
