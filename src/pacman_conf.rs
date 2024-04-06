use std::{
    fs::File,
    io::{BufRead, BufReader, Error as IOError},
};

use crate::error::R;

fn get_file(path: &str) -> Result<File, IOError> {
    File::open(path)
}

fn skip_line(s: &str) -> bool {
    s.is_empty() || s.starts_with('#')
}

fn parse_section(s: &str) -> Option<&str> {
    s.trim().strip_prefix('[')?.strip_suffix(']')
}

fn parse_option(s: &str) -> Option<(&str, &str)> {
    s.split_once('=')
        .map(|(name, value)| (name.trim(), value.trim()))
}

pub struct PacmanConf {
    pub dbpath: Option<String>,
    pub cachedir: Option<String>,
    pub repos: Vec<String>,
}

fn read_config(mut reader: impl BufRead) -> R<PacmanConf> {
    let mut line = String::new();
    let mut in_options = false;
    let mut dbpath = None;
    let mut cachedir = None;
    let mut repos = Vec::new();

    loop {
        line.clear();

        if reader.read_line(&mut line)? == 0 {
            break;
        }

        if skip_line(&line) {
            continue;
        }

        match parse_section(&line) {
            Some("options") => {
                in_options = true;
                continue;
            }
            Some(s) => {
                in_options = false;
                repos.push(s.to_string());
                continue;
            }
            _ => {}
        }

        if !in_options {
            continue;
        }

        if dbpath.is_none() {
            if let Some(("DBPath", value)) = parse_option(&line) {
                dbpath = Some(value.to_string());
            }
        }

        if cachedir.is_none() {
            if let Some(("CacheDir", value)) = parse_option(&line) {
                cachedir = Some(value.to_string());
            }
        }
    }

    Ok(PacmanConf {
        dbpath,
        cachedir,
        repos,
    })
}

pub fn get_configuration(path: &str) -> R<PacmanConf> {
    read_config(BufReader::new(get_file(path)?))
}

#[cfg(test)]
mod tests;
