use crate::{
    print,
    types::{Arr, Str},
};
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    io::{ErrorKind, Result},
    path::{self, PathBuf},
};

const DB_DIR: &str = "sync";
const DB_EXT: &str = ".db";
const PKG_EXT: &str = ".pkg.tar";
const SIG_EXT: &str = ".sig";

macro_rules! R {
    ($e: expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        }
    };
}

macro_rules! N {
    ($e: expr) => {
        if $e {
            return None;
        }
    };
}

macro_rules! Y {
    ($e: expr) => {
        N!(!$e)
    };
}

fn map_repos(entry: Result<DirEntry>) -> Option<Result<Str>> {
    let entry = R!(entry);
    Y!(R!(entry.metadata()).is_file());

    let mut name = entry.file_name().into_string().ok()?;
    Y!(name.ends_with(DB_EXT));

    let len = name.len().checked_sub(DB_EXT.len())?;
    Y!(len > 0);
    name.truncate(len);

    Some(Ok(Str::from(name)))
}

pub fn find_repos(dbpath: &str) -> Result<Arr<Str>> {
    fs::read_dir(PathBuf::from_iter([dbpath, DB_DIR]))?
        .filter_map(map_repos)
        .collect()
}

fn map_pkgs(entry: Result<DirEntry>) -> Option<Result<(Str, u64)>> {
    let entry = R!(entry);
    let name = Str::from(entry.file_name().into_string().ok()?);

    let meta = R!(entry.metadata());
    Y!(meta.is_file());

    N!(name.ends_with(SIG_EXT));
    Y!(name.contains(PKG_EXT));

    Some(Ok((name, meta.len())))
}

pub fn get_cached_pkgs(cachedir: &str) -> Result<HashMap<Str, u64>> {
    fs::read_dir(cachedir)?.filter_map(map_pkgs).collect()
}

fn remove_file(path: &str) {
    if let Err(e) = fs::remove_file(path) {
        if e.kind() != ErrorKind::NotFound {
            print::warning(format_args!("failed to remove '{path}': {e}"));
        }
    }
}

pub fn remove_pkg(cachedir: &str, name: &str) {
    let mut path = String::from_iter([cachedir, path::MAIN_SEPARATOR_STR, name, SIG_EXT]);
    remove_file(&path);

    path.truncate(path.len() - SIG_EXT.len());
    remove_file(&path);
}
