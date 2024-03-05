use std::{error, fmt};

#[derive(Debug)]
pub struct CmdParseError {
    err_msg: String
}

impl From<&str> for CmdParseError {
    fn from(cmd: &str) -> Self {
        CmdParseError { err_msg: format!("{} doesn't exist", cmd) }
    }
}

impl fmt::Display for CmdParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse the command: {}", self.err_msg)
    }
}

impl error::Error for CmdParseError {}
