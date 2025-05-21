mod email;
mod email_config;

use email_config::EmailConfig;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (config, recipient, subject, body, cc = None, bcc = None, attachment = None))]
fn send_email(
    config: EmailConfig,
    recipient: Vec<String>,
    subject: &str,
    body: &str,
    cc: Option<Vec<String>>,
    bcc: Option<Vec<String>>,
    attachment: Option<&str>,
) -> PyResult<()> {
    match email::send_email(config, recipient, subject, body, cc, bcc, attachment) {
        Ok(_) => Ok(()),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn simple_smtp_sender(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<EmailConfig>()?;
    m.add_function(wrap_pyfunction!(send_email, m)?)?;
    Ok(())
}
