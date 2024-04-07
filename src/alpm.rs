use alpm::{Alpm, Db, Error, Event, Result, SigLevel};

pub fn init_alpm(dbpath: &str) -> Result<Alpm> {
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

pub fn load_dbs(alpm: &Alpm, repos: impl Iterator<Item = String>) -> Result<Vec<&Db>> {
    repos
        .filter_map(|repo| match alpm.register_syncdb(repo, SigLevel::NONE) {
            Err(Error::DbNotNull) => None,
            r => Some(r),
        })
        .collect::<Result<Vec<_>>>()
}
