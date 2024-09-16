use crate::{package::Pkg, print, types::Arr};
use alpm::{Alpm, Db, Event, Result, SigLevel};
use std::collections::HashMap;

fn init(dbpath: &str) -> Result<Alpm> {
    let alpm = Alpm::new("/", dbpath)?;

    alpm.set_event_cb((), |e, _| {
        if let Event::DatabaseMissing(event) = e.event() {
            print::warning(format_args!(
                "database file for '{}' does not exist",
                event.dbname()
            ))
        }
    });

    Ok(alpm)
}

fn load_dbs<'a>(alpm: &'a Alpm, repos: &[impl AsRef<str>]) -> Result<Arr<&'a Db>> {
    repos
        .iter()
        .map(|repo| alpm.register_syncdb(repo.as_ref(), SigLevel::NONE))
        .collect()
}

pub fn filter_pkgs(pkgs: Arr<Pkg>, dbpath: &str, repos: &[impl AsRef<str>]) -> Result<Arr<Pkg>> {
    let alpm = init(dbpath)?;
    let dbs = load_dbs(&alpm, repos)?;

    let mut map: HashMap<_, _> = pkgs
        .into_vec() // rust-lang/rust#59878
        .into_iter()
        .map(Pkg::into_hash)
        .collect();

    for db in dbs {
        for pkg in db.pkgs() {
            if let Some(name) = pkg.filename() {
                map.remove(name);
            }
        }
    }

    let mut result: Arr<_> = map.into_iter().map(Pkg::from_hash).collect();
    result.sort_unstable();
    Ok(result)
}
