use pyo3::PyErr;
use thiserror::Error;

/// Represent possible errors returned by this library.
#[derive(Error, Debug)]
pub enum StrSimError {
    /// Represents errors that occur when the input data passing to the library is invalid.
    #[error("Invalid input data: {0}")]
    InvalidInputData(String),
    /// Represents errors that occur when the configuration data passing to the library is invalid.
    #[error("Invalid configuration: {0}")]
    InvalidConfigData(String),

    #[error("Integrity error - asking for an entity that isn't in the database: {0}")]
    DBIntegrityError(String),

    #[error("Generic integrity error: {0}")]
    IntegrityError(String),

    #[error("Logic error: {0}")]
    LogicError(String),

    #[error("Invalid arguments: {0}")]
    InvalidArgument(String),

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// PyO3 error
    #[error(transparent)]
    PyErr(#[from] pyo3::PyErr),

    #[error(transparent)]
    LSAPErr(#[from] lsap::LSAPError),
}

pub fn into_pyerr<E: Into<StrSimError>>(err: E) -> PyErr {
    let hderr = err.into();
    if let StrSimError::PyErr(e) = hderr {
        e
    } else {
        let anyerror: anyhow::Error = hderr.into();
        anyerror.into()
    }
}
