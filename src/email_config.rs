use pyo3::{pyclass, pymethods};
use std::fmt;

#[derive(Clone)]
#[pyclass(dict, get_all, set_all, str, subclass)]
pub struct EmailConfig {
    pub server: String,
    pub sender_email: String,
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
            sender_email: sender_email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
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
