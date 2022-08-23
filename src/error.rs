use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    string::FromUtf8Error,
};

use csv::Error as CsvError;

/// Error type used when handling transactions.
///
/// New type now to allow for easy usage.
/// Improved error handling can be added later.
#[derive(Debug)]
pub struct TransactionError(String);

impl Display for TransactionError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl From<CsvError> for TransactionError {
    fn from(err: CsvError) -> Self {
        Self(err.to_string())
    }
}

impl From<IoError> for TransactionError {
    fn from(err: IoError) -> Self {
        Self(err.to_string())
    }
}

impl From<FromUtf8Error> for TransactionError {
    fn from(err: FromUtf8Error) -> Self {
        Self(err.to_string())
    }
}

impl From<&str> for TransactionError {
    fn from(err: &str) -> Self {
        Self(err.to_owned())
    }
}

impl From<String> for TransactionError {
    fn from(err: String) -> Self {
        Self(err)
    }
}
