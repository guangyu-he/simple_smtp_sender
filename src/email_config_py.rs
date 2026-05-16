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
    ///
    /// # Requirements
    /// `pydantic` must be installed in the active Python environment (and the
    /// caller must be running inside that environment / virtualenv). If
    /// importing `pydantic` fails, a `RuntimeError` is raised describing the
    /// underlying import error.
    ///
    /// # Arguments
    /// * `pydantic_obj` - Pydantic BaseModel instance containing configuration parameters
    ///
    /// # Returns
    /// An EmailConfig instance populated from the Pydantic BaseModel.
    ///
    /// # Raises
    /// * `RuntimeError` - if `pydantic` cannot be imported from the current
    ///   Python environment.
    /// * `TypeError` - if `pydantic_obj` is not an instance of
    ///   `pydantic.BaseModel`.
    /// * `ValueError` - if the model's fields cannot be deserialized into an
    ///   `EmailConfig` (e.g. missing or mistyped fields).
    fn load_from_pydantic<'p>(
        _cls: &Bound<'p, PyType>,
        py: Python<'p>,
        pydantic_obj: Bound<'p, PyAny>,
    ) -> PyResult<Self> {
        let module = PyModule::import(py, "pydantic").map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to import `pydantic`. Make sure `pydantic` is installed \
                 in the active Python environment (e.g. `pip install pydantic`) \
                 and that this code is being executed inside that environment. \
                 Original error: {e}"
            ))
        })?;
        let base_model = module.getattr("BaseModel")?.cast_into::<PyType>()?;

        if !pydantic_obj.is_instance(&base_model)? {
            let got = pydantic_obj
                .get_type()
                .name()
                .map(|n| n.to_string())
                .unwrap_or_else(|_| "<unknown>".to_string());
            return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "Expected an instance of `pydantic.BaseModel`, got `{got}`"
            )));
        }
        let model_dump_fn = pydantic_obj.getattr("model_dump")?;
        let dict_obj = model_dump_fn.call0()?;
        from_pyobject(dict_obj).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!(
                "Failed to deserialize Pydantic model into EmailConfig: {e}"
            ))
        })
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
