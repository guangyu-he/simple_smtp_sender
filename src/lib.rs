mod email;
mod email_config;

pub use email::async_send_email as send_email_async;
pub use email::send_email as send_email_sync;
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
fn async_send_email<'p>(
    py: Python<'p>,
    config: Bound<'p, PyAny>,
    recipient: Bound<'p, PyAny>,
    subject: Bound<'p, PyAny>,
    body: Bound<'p, PyAny>,
    cc: Option<Bound<'p, PyAny>>,
    bcc: Option<Bound<'p, PyAny>>,
    attachment: Option<Bound<'p, PyAny>>,
) -> PyResult<Bound<'p, PyAny>> {
    let config = config.extract::<EmailConfig>()?;
    let recipient = recipient.extract::<Vec<String>>()?;
    let subject = subject.extract::<String>()?;
    let body = body.extract::<String>()?;
    let cc = match cc {
        Some(cc) => Some(cc.extract::<Vec<String>>()?),
        None => None,
    };
    let bcc = match bcc {
        Some(bcc) => Some(bcc.extract::<Vec<String>>()?),
        None => None,
    };
    let attachment = match attachment {
        Some(attachment) => Some(attachment.extract::<String>()?),
        None => None,
    };

    future_into_py(py, async move {
        match send_email_async(config, recipient, subject, body, cc, bcc, attachment).await {
            Ok(_) => Ok(()),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
        }
    })
}

#[cfg(feature = "python")]
#[pymodule]
fn simple_smtp_sender(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<EmailConfig>()?;
    m.add_function(wrap_pyfunction!(send_email, m)?)?;
    m.add_function(wrap_pyfunction!(async_send_email, m)?)?;
    Ok(())
}
