const DEFAULT_DBPATH: &str = "/var/lib/pacman";
const DEFAULT_CACHEDIR: &str = "/var/cache/pacman/pkg";
const DEFAULT_REPOS: &[&str] = &["core", "extra"];

mod alpm;
mod byte_format;
mod io;

use std::{error::Error, process::ExitCode};

type R<T> = Result<T, Box<dyn Error>>;

pub fn filter_pkgs(
    dbpath: &str,
    cachedir: &str,
    repos: &[impl AsRef<str>],
) -> R<Vec<(String, u64)>> {
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

    let mut files: Vec<_> = pkgs.into_iter().collect();
    files.sort_unstable();
    Ok(files)
}

fn run() -> R<()> {
    io::print_message("checking for outdated packages...");

    let dbpath = DEFAULT_DBPATH;
    let cachedir = DEFAULT_CACHEDIR;
    let repos = DEFAULT_REPOS;
    let pkgs = &filter_pkgs(dbpath, cachedir, repos)?;

    if pkgs.is_empty() {
        io::print_message("no outdated packages");
        return Ok(());
    }

    println!();
    let mut total = 0;

    for (name, size) in pkgs {
        total += size;
        io::print_pkg(name, *size);
    }

    println!();
    io::print_pkg(
        format_args!("Total packages to remove: {}", pkgs.len()),
        total,
    );
    println!();

    if !io::make_request("Proceed with removing?")? {
        return Ok(());
    }

    io::print_message("removing outdated packages...");

    for (name, _) in pkgs {
        io::remove_pkg(cachedir, name);
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            io::print_error(e);
            ExitCode::FAILURE
        }
        _ => ExitCode::SUCCESS,
    }
}
