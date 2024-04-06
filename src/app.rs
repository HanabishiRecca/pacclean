use crate::consts::*;
use crate::error::R;

use alpm::{Alpm, Db, Error, Event, SigLevel};
use std::{
    collections::HashMap,
    fs,
    io::{self, ErrorKind, Read, Write},
    path,
};

fn is_pkg_file(name: &str) -> bool {
    name.contains(".pkg.tar") && name.split('.').next_back() != Some("sig")
}

fn get_cached_pkgs(cachedir: &str) -> R<HashMap<String, u64>> {
    Ok(fs::read_dir(cachedir)?
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            let meta = entry.metadata().ok()?;
            (meta.is_file() && is_pkg_file(&name)).then_some((name, meta.len()))
        })
        .collect())
}

fn load_dbs(alpm: &Alpm, repos: impl Iterator<Item = String>) -> R<Vec<&Db>> {
    Ok(repos
        .filter_map(|repo| match alpm.register_syncdb(repo, SigLevel::NONE) {
            Err(Error::DbNotNull) => None,
            r => Some(r),
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn init_alpm(dbpath: &str) -> R<Alpm> {
    let alpm = Alpm::new("/", dbpath)?;

    alpm.set_event_cb((), |e, _| {
        if let Event::DatabaseMissing(event) = e.event() {
            println!(
                "database file for '{}' does not exist (use 'pacman -Sy' to download)",
                event.dbname()
            )
        }
    });

    Ok(alpm)
}

fn find_files_to_purge(dbpath: &str, cachedir: &str, repos: Vec<String>) -> R<Vec<(String, u64)>> {
    let mut files = get_cached_pkgs(cachedir)?;
    let alpm = init_alpm(dbpath)?;
    let dbs = load_dbs(&alpm, repos.into_iter())?;

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

fn read_answer() -> R<bool> {
    print!("[Y/n] ");
    io::stdout().flush()?;
    let mut buf = [0];
    io::stdin().read_exact(buf.as_mut_slice())?;
    let [code] = buf;
    Ok(code == b'\n' || code == b'y' || code == b'Y')
}

fn human_readable_size(size: u64) -> String {
    const UNITS: &[&str] = &["KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    const FACTOR: f64 = 1024.0;

    if size < FACTOR as u64 {
        return format!("{size} B");
    }

    let mut result = (size as f64) / FACTOR;
    let mut n = 0;

    while result >= FACTOR {
        result /= FACTOR;
        n += 1;
    }

    format!(
        "{} {}",
        (result * 100.0).round() / 100.0,
        UNITS.get(n).unwrap_or(&"")
    )
}

fn remove_package(cachedir: &str, name: &str) -> bool {
    let mut path = String::from_iter([cachedir, path::MAIN_SEPARATOR_STR, &name]);

    if let Err(e) = fs::remove_file(&path) {
        println!("Failed to remove '{path}': {e}");
        return false;
    }

    path.push_str(".sig");

    if let Err(e) = fs::remove_file(&path) {
        if e.kind() != ErrorKind::NotFound {
            println!("Failed to remove '{path}': {e}");
        }
    }

    true
}

pub fn run() -> R<()> {
    let conf = crate::pacman_conf::get_configuration(PACMAN_CONF)?;
    let dbpath = conf.dbpath.as_deref().unwrap_or(DEFAULT_DBPATH);
    let cachedir = conf.cachedir.as_deref().unwrap_or(DEFAULT_CACHEDIR);
    let files = find_files_to_purge(dbpath, cachedir, conf.repos)?;

    println!("Cache directory: {cachedir}");

    if files.is_empty() {
        println!("No packages to remove.");
        return Ok(());
    }

    println!("The following packages will be removed:");
    println!();

    for (name, size) in &files {
        println!("{name} ({})", human_readable_size(*size));
    }

    println!();
    print!(":: Do you want to proceed? ");

    if !read_answer()? {
        return Ok(());
    }

    println!("removing old packages from cache...");

    let mut count = 0;
    let mut total = 0;

    for (name, size) in &files {
        if remove_package(cachedir, name) {
            count += 1;
            total += size;
        }
    }

    println!("Removed packages: {count} ({})", human_readable_size(total));
    Ok(())
}
