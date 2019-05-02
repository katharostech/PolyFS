use std::error::Error;
use std::fmt;
use clap::ArgMatches;
use clap::arg_enum;

///
/// This is a convenient way to pass the arguments that a subcommand are going
/// to need.
pub struct ArgSet<'a> {
    /// The global CLI argument matches.
    pub global: &'a ArgMatches<'a>,
    /// The argument matches for the current subcommand.
    pub sub: &'a ArgMatches<'a>,
}

/// Return type for most CLI components.
pub type CliResult<T> = Result<T, CliError>;

/// A CLI Error.
#[derive(Debug)]
pub struct CliError {
    /// Should describe what the program was trying to do and could not.
    pub message: String,
    /// The actual Error that occurred when attempting to perform the operation
    /// described by the `message`.
    pub cause: Option<Box<dyn Error>>
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.source() {
            Some(cause) => write!(f, "{}\nCaused by: {}", self.message, cause),
            None => write!(f, "{}", self.message)
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.cause {
            Some(e) => Some(e.as_ref()),
            None => None
        }
    }
}

/// Utility for propagating CLI errors.
/// 
/// Takes a result and returns the Ok value if it is Ok and creates a CliError
/// with the given message if the result is Err.
///
/// # Example
/// ```
/// let result: Result<&str, CliError> = Ok("hello world");
/// let hello = try_to!(result, "Couldn't get hello message");
/// assert_eq!(hello, "hello world");
/// let result: Result<&str, CliError> = Err(CliError {
///     message: "There was a problem",
///     cause: None
/// })
/// let hello = try_to!(result, "Error message");
/// ```
/// the last line will fail returning a CliError with the message set to "Error
/// message" and the cause set to the error in `result`.
#[macro_export]
macro_rules! try_to {
    ( $result:expr, $error_message:expr ) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                let error = Err(crate::cli::CliError {
                    message: String::from($error_message),
                    cause: Some(Box::new(e))
                });
                log::debug!("Error details: {:#?}", error);
                return error
            }
        }
    };
}

/// The format used for the PolyFS configuration file.
arg_enum! {
    /// A file format supported for the PolyFS config file
    #[allow(non_camel_case_types, missing_docs)]
    #[derive(PartialEq, Debug)]
    pub enum ConfigFormat {
        yaml,
        json,
    }
}
