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

pub fn find_packages_to_purge(
    dbpath: &str,
    cachedir: &str,
    repos: Vec<String>,
) -> R<Vec<(String, u64)>> {
    let mut files = io::get_cached_pkgs(cachedir)?;
    let alpm = alpm::init_alpm(dbpath)?;
    let dbs = alpm::load_dbs(&alpm, repos.into_iter())?;

    for db in dbs {
        for pkg in db.pkgs() {
            if let Some(name) = pkg.filename() {
                files.remove(name);
            }
        }
    }

    let mut files: Vec<_> = files.into_iter().collect();
    files.sort_unstable();
    Ok(files)
}

fn run() -> R<()> {
    let conf = pacman_conf::get_configuration(PACMAN_CONF)?;
    let dbpath = conf.dbpath.as_deref().unwrap_or(DEFAULT_DBPATH);
    let cachedir = conf.cachedir.as_deref().unwrap_or(DEFAULT_CACHEDIR);
    let files = find_packages_to_purge(dbpath, cachedir, conf.repos)?;

    println!("Cache directory: {cachedir}");

    if files.is_empty() {
        println!("No packages to remove.");
        return Ok(());
    }

    println!("Out of sync packages:");
    println!();

    let mut total = 0;

    for (name, size) in &files {
        total += size;
        println!("{name} ({})", ByteFormat(*size));
    }

    println!();
    println!(
        "Total packages to remove: {} ({})",
        files.len(),
        ByteFormat(total)
    );
    print!(":: Do you want to proceed? [Y/n] ");

    if !io::read_answer()? {
        return Ok(());
    }

    println!("Removing out of sync packages from the cache...");

    for (name, _) in &files {
        io::remove_package(cachedir, name);
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
