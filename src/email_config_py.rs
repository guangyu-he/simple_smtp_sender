#[cfg(feature = "python")]
use crate::EmailConfig;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyType;
#[cfg(feature = "python")]
use pyo3::{pymethods, Bound, PyResult, Python};
#[cfg(feature = "python")]
use serde_pyobject::{from_pyobject, to_pyobject};
#[cfg(feature = "python")]
use std::collections::HashMap;

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
    /// # Returns
    /// A new EmailConfig instance populated with the provided parameters
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

    #[classmethod]
    /// Loads EmailConfig from a Pydantic BaseModel
    /// # Arguments
    /// * `pydantic_obj` - Pydantic BaseModel instance containing configuration parameters
    /// # Returns
    /// An EmailConfig instance populated from the Pydantic BaseModel, or None if the object is not a Pydantic BaseModel
    fn load_from_pydantic<'p>(
        _cls: &Bound<'p, PyType>,
        py: Python<'p>,
        pydantic_obj: Bound<'p, PyAny>,
    ) -> PyResult<Option<Self>> {
        // Try to import pydantic module
        let module = match PyModule::import(py, "pydantic") {
            Ok(m) => m,
            Err(_) => {
                return Ok(None);
            }
        };
        let base_model = module.getattr("BaseModel")?.cast_into::<PyType>()?;

        if pydantic_obj.is_instance(&base_model)? {
            let model_dump_fn = pydantic_obj.getattr("model_dump")?;
            let dict_obj = model_dump_fn.call0()?;
            Ok(Some(from_pyobject(dict_obj)?))
        } else {
            Ok(None)
        }
    }

    /// Converts EmailConfig to a Python dictionary
    fn to_dict<'p>(&self, py: Python<'p>) -> PyResult<Bound<'p, PyAny>> {
        Ok(to_pyobject(py, self).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to convert object to dict: {}",
                e
            ))
        })?)
    }
}
