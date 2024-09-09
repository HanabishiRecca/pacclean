mod alpm;
mod byte_format;
mod cli;
mod io;

use std::{error::Error, process::ExitCode};

type R<T> = Result<T, Box<dyn Error>>;

const DEFAULT_CACHEDIR: &str = "/var/cache/pacman/pkg";
const DEFAULT_DBPATH: &str = "/var/lib/pacman";

fn filter_pkgs(cachedir: &str, dbpath: &str, repos: &[impl AsRef<str>]) -> R<Vec<(String, u64)>> {
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

fn print_help() {
    let bin = std::env::current_exe().ok();
    println!(
        include_str!("help.in"),
        PKG = env!("CARGO_PKG_NAME"),
        VER = env!("CARGO_PKG_VERSION"),
        BIN_NAME = (|| bin.as_ref()?.file_name()?.to_str())().unwrap_or(env!("CARGO_BIN_NAME")),
    );
}

fn run() -> R<()> {
    let Some(config) = cli::read_args(std::env::args().skip(1))? else {
        print_help();
        return Ok(());
    };

    io::print_message("checking for outdated packages...");

    let cachedir = config.cachedir().unwrap_or(DEFAULT_CACHEDIR);
    let dbpath = config.dbpath().unwrap_or(DEFAULT_DBPATH);
    let repos = match config.repos() {
        Some(repos) => repos,
        _ => &io::find_repos(dbpath)?,
    };
    let pkgs = &filter_pkgs(cachedir, dbpath, repos)?;

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
