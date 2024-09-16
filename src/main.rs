mod alpm;
mod byte_format;
mod cli;
mod io;
mod package;
mod print;
mod types;

use std::{env, error::Error, process::ExitCode};

const DEFAULT_CACHEDIR: &str = "/var/cache/pacman/pkg";
const DEFAULT_DBPATH: &str = "/var/lib/pacman";

macro_rules! default {
    ($option: expr, $default: expr) => {
        match $option {
            Some(value) => value,
            _ => $default,
        }
    };
}

fn print_help() {
    let bin = env::current_exe().ok();
    println!(
        include_str!("help.in"),
        PKG = env!("CARGO_PKG_NAME"),
        VER = env!("CARGO_PKG_VERSION"),
        BIN_NAME = default!(
            (|| bin.as_ref()?.file_name()?.to_str())(),
            env!("CARGO_BIN_NAME")
        ),
    );
}

fn run() -> Result<(), Box<dyn Error>> {
    let Some(config) = cli::read_args(env::args().skip(1))? else {
        print_help();
        return Ok(());
    };

    print::message("checking for outdated packages...");

    let cachedir = default!(config.cachedir(), DEFAULT_CACHEDIR);
    let dbpath = default!(config.dbpath(), DEFAULT_DBPATH);
    let repos = default!(config.repos(), &io::find_repos(dbpath)?);

    let pkgs = alpm::filter_pkgs(io::get_cached_pkgs(cachedir)?, dbpath, repos)?;

    if pkgs.is_empty() {
        print::message("no outdated packages");
        return Ok(());
    }

    println!();
    let mut total = 0;

    for pkg in &pkgs {
        total += pkg.size();
        print::pkg(pkg);
    }

    println!();
    print::size(
        format_args!("Total packages to remove: {}", pkgs.len()),
        total,
    );
    println!();

    if !print::request("Proceed with removing?")? {
        return Ok(());
    }

    print::message("removing outdated packages...");

    for pkg in &pkgs {
        io::remove_pkg(cachedir, pkg);
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
