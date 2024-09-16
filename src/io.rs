use crate::{
    package::Pkg,
    print,
    types::{Arr, Str},
};
use std::{
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

macro_rules! C {
    ($e: expr) => {
        if !$e {
            return None;
        }
    };
}

fn map_repos(entry: Result<DirEntry>) -> Option<Result<Str>> {
    let entry = R!(entry);
    C!(R!(entry.metadata()).is_file());

    let mut name = entry.file_name().into_string().ok()?;
    C!(name.ends_with(DB_EXT));

    let len = name.len().checked_sub(DB_EXT.len())?;
    C!(len > 0);
    name.truncate(len);

    Some(Ok(Str::from(name)))
}

pub fn find_repos(dbpath: &str) -> Result<Arr<Str>> {
    fs::read_dir(PathBuf::from_iter([dbpath, DB_DIR]))?
        .filter_map(map_repos)
        .collect()
}

fn map_pkgs(entry: Result<DirEntry>) -> Option<Result<Pkg>> {
    let entry = R!(entry);
    let name = Str::from(entry.file_name().into_string().ok()?);

    let meta = R!(entry.metadata());
    C!(meta.is_file());

    C!(!name.ends_with(SIG_EXT));
    C!(name.contains(PKG_EXT));

    Some(Ok(Pkg::new(name, meta.len())))
}

pub fn get_cached_pkgs(cachedir: &str) -> Result<Arr<Pkg>> {
    fs::read_dir(cachedir)?.filter_map(map_pkgs).collect()
}

fn remove_file(path: &str) {
    if let Err(e) = fs::remove_file(path) {
        if e.kind() != ErrorKind::NotFound {
            print::warning(format_args!("failed to remove '{path}': {e}"));
        }
    }
}

pub fn remove_pkg(cachedir: &str, pkg: &Pkg) {
    let mut path = String::from_iter([cachedir, path::MAIN_SEPARATOR_STR, pkg.name(), SIG_EXT]);
    remove_file(&path);

    path.truncate(path.len() - SIG_EXT.len());
    remove_file(&path);
}
