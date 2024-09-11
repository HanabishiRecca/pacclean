mod alpm;
mod byte_format;
mod cli;
mod io;
mod print;
mod types;

use crate::types::{Arr, Str};
use std::{env, error::Error, process::ExitCode};

type R<T> = Result<T, Box<dyn Error>>;

const DEFAULT_CACHEDIR: &str = "/var/cache/pacman/pkg";
const DEFAULT_DBPATH: &str = "/var/lib/pacman";

fn filter_pkgs(cachedir: &str, dbpath: &str, repos: &[impl AsRef<str>]) -> R<Arr<(Str, u64)>> {
    let mut pkgs = io::get_cached_pkgs(cachedir)?;
    let alpm = alpm::init(dbpath)?;
    let dbs = alpm::load_dbs(&alpm, repos)?;

    for db in dbs {
        for pkg in db.pkgs() {
            if let Some(name) = pkg.filename() {
                pkgs.remove(name);
            }
        }
    }

    let mut files: Arr<_> = pkgs.into_iter().collect();
    files.sort_unstable();
    Ok(files)
}

fn print_help() {
    let bin = env::current_exe().ok();
    println!(
        include_str!("help.in"),
        PKG = env!("CARGO_PKG_NAME"),
        VER = env!("CARGO_PKG_VERSION"),
        BIN_NAME = (|| bin.as_ref()?.file_name()?.to_str())().unwrap_or(env!("CARGO_BIN_NAME")),
    );
}

fn run() -> R<()> {
    let Some(config) = cli::read_args(env::args().skip(1))? else {
        print_help();
        return Ok(());
    };

    print::message("checking for outdated packages...");

    let cachedir = config.cachedir().unwrap_or(DEFAULT_CACHEDIR);
    let dbpath = config.dbpath().unwrap_or(DEFAULT_DBPATH);

    let pkgs = match config.repos() {
        Some(repos) => &filter_pkgs(cachedir, dbpath, repos)?,
        _ => &filter_pkgs(cachedir, dbpath, &io::find_repos(dbpath)?)?,
    };

    if pkgs.is_empty() {
        print::message("no outdated packages");
        return Ok(());
    }

    println!();
    let mut total = 0;

    for (name, size) in pkgs {
        total += size;
        print::pkg(name, *size);
    }

    println!();
    print::pkg(
        format_args!("Total packages to remove: {}", pkgs.len()),
        total,
    );
    println!();

    if !print::request("Proceed with removing?")? {
        return Ok(());
    }

    print::message("removing outdated packages...");

    for (name, _) in pkgs {
        io::remove_pkg(cachedir, name);
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            print::error(e);
            ExitCode::FAILURE
        }
        _ => ExitCode::SUCCESS,
    }
}
