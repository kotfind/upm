use core::fmt::Write;
use ekv::flash::Flash;
use embassy_time::TimeoutError;
use heapless::String;
use log::error;
use thiserror::Error;
use upm_common::{Resp, resp::ErrorResp};

use crate::io;

pub type QueryResult = Result<(), QueryError>;

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("internal error occured")]
    Internal,

    #[error("{msg}")]
    Custom { msg: String<128> },
}

impl From<QueryError> for Resp {
    fn from(e: QueryError) -> Self {
        let mut text = String::new();

        if let Err(er) = write!(text, "{e}") {
            error!("failed to write query error: {er}");
            text = "unknown".try_into().unwrap();
        }

        Resp::Error(ErrorResp { text })
    }
}

impl<F: Flash> From<rekv::Error<F>> for QueryError {
    fn from(e: rekv::Error<F>) -> Self {
        error!("query error: rekv: {e:?}");
        QueryError::Internal
    }
}

impl From<TimeoutError> for QueryError {
    fn from(e: TimeoutError) -> Self {
        error!("query error: timeout: {e:?}");
        QueryError::Internal
    }
}

impl From<io::Error> for QueryError {
    fn from(e: io::Error) -> Self {
        error!("query error: io: {e:?}");
        QueryError::Internal
    }
}
