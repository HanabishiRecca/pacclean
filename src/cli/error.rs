use crate::types::Str;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    NoValue(Str),
    Unknown(Str),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            NoValue(arg) => write!(f, "option '{arg}' requires value"),
            Unknown(arg) => write!(f, "unknown option '{arg}'"),
        }
    }
}
