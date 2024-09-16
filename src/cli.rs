#[cfg(test)]
mod tests;

use crate::types::{Arr, Str};

#[derive(Default)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Config {
    dbpath: Option<Str>,
    cachedir: Option<Str>,
    repos: Option<Arr<Str>>,
}

impl Config {
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
pub enum Error {
    NoValue(Str),
    Unknown(Str),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match self {
            NoValue(arg) => write!(f, "option '{arg}' requires value"),
            Unknown(arg) => write!(f, "unknown option '{arg}'"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! E {
    ($e: expr) => {{
        use Error::*;
        return Err($e);
    }};
}

macro_rules! F {
    ($s: expr) => {
        From::from($s.as_ref())
    };
}

fn parse_list<'a, T: FromIterator<impl From<&'a str>>>(str: &'a str) -> T {
    str.split(',')
        .filter(|s| !s.is_empty())
        .map(From::from)
        .collect()
}

pub fn read_args(mut args: impl Iterator<Item = impl AsRef<str>>) -> Result<Option<Config>> {
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
