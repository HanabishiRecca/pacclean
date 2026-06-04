use crate::package::Pkg;
use crate::print;
use crate::types::{Arr, Str};
use alpm::{Alpm, AnyEvent, Db, Event, Package, Result, SigLevel};
use std::collections::HashMap;

fn event(e: AnyEvent, _: &mut ()) {
    if let Event::DatabaseMissing(event) = e.event() {
        print::warning(format_args!("database file for '{}' does not exist", event.dbname()));
    }
}

fn init(dbpath: &str) -> Result<Alpm> {
    let alpm = Alpm::new("/", dbpath)?;
    alpm.set_event_cb((), event);
    Ok(alpm)
}

fn load_dbs<'a>(alpm: &'a Alpm, repos: &[impl AsRef<str>]) -> Result<Arr<&'a Db>> {
    repos.iter().map(|repo| alpm.register_syncdb(repo.as_ref(), SigLevel::NONE)).collect()
}

fn filter_simple(map: &mut HashMap<Str, u64>, dbs: Arr<&Db>) {
    for db in dbs {
        for pkg in db.pkgs() {
            if let Some(name) = pkg.filename() {
                map.remove(name);
            }
        }
    }
}

fn filter_unique(map: &mut HashMap<Str, u64>, dbs: Arr<&Db>) {
    let mut pkgs = HashMap::<&str, &Package>::new();

    for db in dbs {
        for pkg in db.pkgs() {
            let name = pkg.name();

            if let Some(other) = pkgs.get(name) {
                if pkg.version() < other.version() {
                    continue;
                }
            }

            pkgs.insert(name, pkg);
        }
    }

    for pkg in pkgs.into_values() {
        if let Some(filename) = pkg.filename() {
            map.remove(filename);
        }
    }
}

pub fn filter_pkgs(
    pkgs: Arr<Pkg>, dbpath: &str, repos: &[impl AsRef<str>], unique: bool,
) -> Result<Arr<Pkg>> {
    let alpm = init(dbpath)?;
    let dbs = load_dbs(&alpm, repos)?;

    let mut map: HashMap<_, _> = pkgs.into_iter().map(Pkg::into_kv).collect();
    match unique {
        true => filter_unique(&mut map, dbs),
        _ => filter_simple(&mut map, dbs),
    }

    let mut result: Arr<_> = map.into_iter().map(Pkg::from_kv).collect();
    result.sort_unstable();
    Ok(result)
}
