use std::{error, fmt};

#[derive(Debug)]
pub struct CmdParseError {
    pub err_msg: String
}

impl fmt::Display for CmdParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse the command: {}", self.err_msg)
    }
}

impl error::Error for CmdParseError {}
