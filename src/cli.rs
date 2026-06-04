#[cfg(test)]
mod tests;

use crate::types::{Arr, Str};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Default)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Config {
    unique: Option<bool>,
    dbpath: Option<Str>,
    cachedir: Option<Str>,
    repos: Option<Arr<Str>>,
}

impl Config {
    pub fn unique(&self) -> Option<bool> {
        self.unique
    }

    pub fn cachedir(&self) -> Option<&str> {
        self.cachedir.as_deref()
    }

    pub fn dbpath(&self) -> Option<&str> {
        self.dbpath.as_deref()
    }

    pub fn repos(&self) -> Option<&[Str]> {
        self.repos.as_deref()
    }
}

#[derive(Debug)]
pub enum CliError {
    NoValue(Str),
    Unknown(Str),
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use CliError::*;
        match self {
            NoValue(arg) => write!(f, "option '{arg}' requires value"),
            Unknown(arg) => write!(f, "unknown option '{arg}'"),
        }
    }
}

macro_rules! E {
    ($e: expr) => {{
        use CliError::*;
        return Err($e);
    }};
}

macro_rules! F {
    ($s: expr) => {
        From::from($s.as_ref())
    };
}

fn parse_list<'a, T: FromIterator<impl From<&'a str>>>(str: &'a str) -> T {
    str.split(',').filter(|s| !s.is_empty()).map(From::from).collect()
}

pub fn read_args(
    mut args: impl Iterator<Item = impl AsRef<str>>,
) -> Result<Option<Config>, CliError> {
    let mut config = Config::default();

    while let Some(arg) = args.next() {
        macro_rules! next {
            () => {
                match args.next() {
                    Some(value) => value,
                    _ => E!(NoValue(F!(arg))),
                }
            };
        }
        macro_rules! list {
            () => {
                parse_list(next!().as_ref())
            };
        }
        match arg.as_ref() {
            "" => {}
            "-u" | "--unique" => {
                config.unique = Some(true);
            }
            "--cachedir" => {
                config.cachedir = Some(F!(next!()));
            }
            "--dbpath" => {
                config.dbpath = Some(F!(next!()));
            }
            "--repos" => {
                config.repos = Some(list!());
            }
            "-h" | "--help" => return Ok(None),
            _ => E!(Unknown(F!(arg))),
        }
    }

    Ok(Some(config))
}
