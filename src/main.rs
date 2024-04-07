const PACMAN_CONF: &str = "/etc/pacman.conf";
const DEFAULT_DBPATH: &str = "/var/lib/pacman/";
const DEFAULT_CACHEDIR: &str = "/var/cache/pacman/pkg/";

mod alpm;
mod byte_format;
mod io;
mod pacman_conf;

use byte_format::ByteFormat;
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
    let conf = pacman_conf::get_configuration(PACMAN_CONF)?;
    let dbpath = conf.dbpath.as_deref().unwrap_or(DEFAULT_DBPATH);
    let cachedir = conf.cachedir.as_deref().unwrap_or(DEFAULT_CACHEDIR);
    let pkgs = &filter_pkgs(dbpath, cachedir, &conf.repos)?;

    println!("Cache directory: {cachedir}");

    if pkgs.is_empty() {
        println!("No packages to remove.");
        return Ok(());
    }

    println!("Out of sync packages:");
    println!();

    let mut total = 0;

    for (name, size) in pkgs {
        total += size;
        println!("{name} ({})", ByteFormat(*size));
    }

    println!();
    println!(
        "Total packages to remove: {} ({})",
        pkgs.len(),
        ByteFormat(total)
    );
    print!(":: Do you want to proceed? [Y/n] ");

    if !io::read_answer()? {
        return Ok(());
    }

    println!("Removing out of sync packages from the cache...");

    for (name, _) in pkgs {
        io::remove_pkg(cachedir, name);
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            println!("{e}");
            ExitCode::FAILURE
        }
        _ => ExitCode::SUCCESS,
    }
}
