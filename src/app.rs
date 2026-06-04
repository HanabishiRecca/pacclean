use crate::{alpm, cli, io, print};
use std::env;
use std::error::Error;

const DEFAULT_UNIQUE: bool = false;
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

pub fn run() -> Result<bool, Box<dyn Error>> {
    let Some(config) = cli::read_args(env::args().skip(1))? else {
        return Ok(true);
    };

    print::message("checking for outdated packages...");

    let unique = default!(config.unique(), DEFAULT_UNIQUE);
    let cachedir = default!(config.cachedir(), DEFAULT_CACHEDIR);
    let dbpath = default!(config.dbpath(), DEFAULT_DBPATH);
    let repos = default!(config.repos(), &io::find_repos(dbpath)?);

    let pkgs = alpm::filter_pkgs(io::get_cached_pkgs(cachedir)?, dbpath, repos, unique)?;

    if pkgs.is_empty() {
        print::message("no outdated packages");
        return Ok(false);
    }

    println!();

    let mut total = 0;

    for pkg in &pkgs {
        total += pkg.size();
        print::pkg(pkg);
    }

    println!();
    print::size(format_args!("Total packages to remove: {}", pkgs.len()), total);
    println!();

    if !print::request("Proceed with removing?")? {
        return Ok(false);
    }

    print::message("removing outdated packages...");

    for pkg in &pkgs {
        io::remove_pkg(cachedir, pkg);
    }

    Ok(false)
}
