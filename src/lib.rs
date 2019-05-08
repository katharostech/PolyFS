#[warn(missing_docs)]
#[warn(future_incompatible)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod cli;
pub mod log;
pub mod app;

use std::error::Error;
use std::fmt;

/// Return type for most PolyFS functions that return results
pub type PolyfsResult<T> = Result<T, PolyfsError>;

/// The PolyFS Error class
///
/// Every PolyFS error has a message and an optional cause.
#[derive(Debug)]
pub struct PolyfsError {
    /// Should describe what the program was trying to do and could not ( i.e.
    /// `Could not create config file` ).
    pub message: String,
    /// The actual Error that occurred when attempting to perform the operation
    /// described by the `message`.
    pub cause: Option<Box<dyn Error>>
}

impl fmt::Display for PolyfsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.source() {
            Some(cause) => write!(f, "{}. Caused by: {}", self.message, cause),
            None => write!(f, "{}", self.message)
        }
    }
}

impl Error for PolyfsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.cause {
            Some(e) => Some(e.as_ref()),
            None => None
        }
    }
}

/// Utility for propagating errors.
///
/// Takes a result and returns the Ok value if it is Ok and creates a
/// PolyfsError with the given message wrapping the causing error if the result
/// is Err.
#[macro_export]
macro_rules! try_to {
    ( $result:expr, $error_message:expr ) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                let error = Err(crate::PolyfsError {
                    message: String::from($error_message),
                    cause: Some(Box::new(e))
                });
                log::debug!("Error details: {:#?}", error);
                return error
            }
        }
    };
}

