mod error;
#[cfg(test)]
mod tests;

use crate::types::{Arr, Str};
use error::Error;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Config {
    dbpath: Option<Str>,
    cachedir: Option<Str>,
    repos: Option<Arr<Str>>,
}

impl Config {
    fn new() -> Self {
        Config {
            cachedir: None,
            dbpath: None,
            repos: None,
        }
    }

    pub fn cachedir(&self) -> Option<&str> {
        self.cachedir.as_deref()
    }

    pub fn dbpath(&self) -> Option<&str> {
        self.dbpath.as_deref()
    }

    pub fn repos(&self) -> Option<&[impl AsRef<str>]> {
        self.repos.as_deref()
    }
}

macro_rules! E {
    ($e: expr) => {{
        use Error::*;
        return Err($e);
    }};
}

fn parse_list<'a, T: FromIterator<impl From<&'a str>>>(str: &'a str) -> T {
    str.split(',')
        .filter(|s| !s.is_empty())
        .map(From::from)
        .collect()
}

macro_rules! F {
    ($s: expr) => {
        From::from($s.as_ref())
    };
}

pub fn read_args(mut args: impl Iterator<Item = impl AsRef<str>>) -> Result<Option<Config>, Error> {
    let mut config = Config::new();

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
            "-c" | "--cachedir" => {
                config.cachedir = Some(F!(next!()));
            }
            "-d" | "--dbpath" => {
                config.dbpath = Some(F!(next!()));
            }
            "-r" | "--repos" => {
                config.repos = Some(list!());
            }
            "-h" | "--help" => {
                return Ok(None);
            }
            _ => E!(Unknown(F!(arg))),
        }
    }

    Ok(Some(config))
}
