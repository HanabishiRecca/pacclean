use alpm::{Alpm, Db, Error, Event, Result, SigLevel};

pub fn init(dbpath: &str) -> Result<Alpm> {
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

pub fn load_dbs<'a>(alpm: &'a Alpm, repos: &[impl AsRef<str>]) -> Result<Vec<&'a Db>> {
    repos
        .iter()
        .filter_map(
            |repo| match alpm.register_syncdb(repo.as_ref(), SigLevel::NONE) {
                Err(Error::DbNotNull) => None,
                r => Some(r),
            },
        )
        .collect::<Result<Vec<_>>>()
}
