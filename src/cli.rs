macro_rules! E {
    ($e: expr) => {
        return Err($e.into())
    };
}

#[derive(Debug)]
pub enum Error {
    NoValue(String),
    Unknown(String),
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

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Config {
    dbpath: Option<String>,
    cachedir: Option<String>,
    repos: Option<Vec<String>>,
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
    
    pub fn repos(&self) -> Option<&[String]> {
        self.repos.as_deref()
    }
}

pub fn read_args(mut args: impl Iterator<Item = String>) -> Result<Option<Config>, Error> {
    let mut config = Config::new();

    while let Some(arg) = args.next() {
        macro_rules! next {
            () => {
                match args.next() {
                    Some(value) => value,
                    _ => E!(Error::NoValue(arg)),
                }
            };
        }
        macro_rules! collect {
            () => {
                next!()
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect()
            };
        }
        match arg.as_str().trim() {
            "" => {}
            "-c" | "--cachedir" => {
                config.cachedir = Some(next!());
            }
            "-d" | "--dbpath" => {
                config.dbpath = Some(next!());
            }
            "-r" | "--repos" => {
                config.repos = Some(collect!());
            }
            "-h" | "--help" => {
                return Ok(None);
            }
            _ => E!(Error::Unknown(arg)),
        }
    }

    Ok(Some(config))
}

#[cfg(test)]
mod tests;
