use thiserror::Error;
use upm_common::resp::ErrorResp;

use crate::io;

pub type CmdResult = Result<(), CmdError>;

#[derive(Debug, Error)]
pub enum CmdError {
    #[error("device returned an error: {msg}")]
    Device { msg: String },

    #[error("failed to get user input")]
    Interract(#[from] dialoguer::Error),

    #[error("failed to communicate with device")]
    IO(#[from] io::Error),

    #[error("system io failed")]
    StdIO(#[from] std::io::Error),

    #[error("confirmation failed for `{field}`")]
    Confirm { field: String },

    #[error("got a response of an unexpected type from a device")]
    UnexpectedResponse,
}

impl From<ErrorResp> for CmdError {
    fn from(e: ErrorResp) -> Self {
        Self::Device {
            msg: e.text.as_str().to_owned(),
        }
    }
}
