use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[cfg(feature = "python")]
use pyo3::{Bound, PyResult, pyclass, pymethods, types::PyType};

#[derive(Clone)]
#[cfg_attr(feature = "python", pyclass(dict, get_all, set_all, str, eq, subclass))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
/// Configuration for email server connection
pub struct EmailConfig {
    pub server: String,
    pub sender_email: String,
    pub username: String,
    pub password: String,
}

impl EmailConfig {
    /// Creates a new EmailConfig instance
    /// # Arguments
    /// * `server` - SMTP server address
    /// * `sender_email` - Sender email address
    /// * `username` - Username for SMTP authentication
    /// * `password` - Password for SMTP authentication
    pub fn new(server: &str, sender_email: &str, username: &str, password: &str) -> Self {
        EmailConfig {
            server: server.to_string(),
            sender_email: sender_email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    /// Loads EmailConfig from environment variables
    /// - EMAIL_SERVER
    /// - EMAIL_SENDER_EMAIL
    /// - EMAIL_USERNAME
    /// - EMAIL_PASSWORD
    pub fn from_env() -> Self {
        let server = std::env::var("EMAIL_SERVER").unwrap_or_default();
        let sender_email = std::env::var("EMAIL_SENDER_EMAIL").unwrap_or_default();
        let username = std::env::var("EMAIL_USERNAME").unwrap_or_default();
        let password = std::env::var("EMAIL_PASSWORD").unwrap_or_default();

        EmailConfig {
            server,
            sender_email,
            username,
            password,
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

impl From<HashMap<String, String>> for EmailConfig {
    fn from(map: HashMap<String, String>) -> Self {
        EmailConfig {
            server: map.get("server").cloned().unwrap_or_default(),
            sender_email: map.get("sender_email").cloned().unwrap_or_default(),
            username: map.get("username").cloned().unwrap_or_default(),
            password: map.get("password").cloned().unwrap_or_default(),
        }
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl EmailConfig {
    #[new]
    #[pyo3(signature = (server, sender_email, username, password))]
    /// Creates a new EmailConfig instance
    /// # Arguments
    /// * `server` - SMTP server address
    /// * `sender_email` - Sender email address
    /// * `username` - Username for SMTP authentication
    /// * `password` - Password for SMTP authentication
    pub fn py_new(
        server: &str,
        sender_email: &str,
        username: &str,
        password: &str,
    ) -> PyResult<Self> {
        Ok(Self::new(server, sender_email, username, password))
    }

    #[classmethod]
    /// Loads EmailConfig from environment variables
    /// # Returns
    /// An EmailConfig instance populated from environment variables
    pub fn load_from_env(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
        Ok(Self::from_env())
    }

    #[classmethod]
    /// Loads EmailConfig from a dictionary
    /// # Arguments
    /// * `map` - A dictionary containing configuration parameters
    /// # Returns
    /// An EmailConfig instance populated from the dictionary
    pub fn load_from_map(_cls: &Bound<'_, PyType>, map: HashMap<String, String>) -> PyResult<Self> {
        Ok(Self::from(map))
    }
}
