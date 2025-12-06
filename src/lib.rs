mod email;
mod email_client;
mod email_config;

pub use email::async_send_email as send_email_async;
pub use email::send_email as send_email_sync;
pub use email_client::EmailClient;
pub use email_config::EmailConfig;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_async_runtimes::tokio::future_into_py;

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (config, recipient, subject, body, cc = None, bcc = None, attachment = None))]
fn send_email(
    config: EmailConfig,
    recipient: Vec<String>,
    subject: String,
    body: String,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    attachment: Option<String>,
) -> PyResult<()> {
    match send_email_sync(config, recipient, subject, body, cc, bcc, attachment) {
        Ok(_) => Ok(()),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
    }
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (config, recipient, subject, body, cc = None, bcc = None, attachment = None))]
pub fn async_send_email<'p>(
    py: Python<'p>,
    config: EmailConfig,
    recipient: Vec<String>,
    subject: String,
    body: String,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    attachment: Option<String>,
) -> PyResult<Bound<'p, PyAny>> {
    future_into_py(py, async move {
        match send_email_async(config, recipient, subject, body, cc, bcc, attachment).await {
            Ok(_) => Ok(()),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
        }
    })
}

#[cfg(feature = "python")]
#[pymodule]
mod simple_smtp_sender {
    #[pymodule_export]
    use crate::async_send_email;
    #[pymodule_export]
    use crate::email_config::EmailConfig;
    #[pymodule_export]
    use crate::send_email;
}
