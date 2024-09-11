use crate::{print, types::Arr};
use alpm::{Alpm, Db, Event, Result, SigLevel};

pub fn init(dbpath: &str) -> Result<Alpm> {
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

pub fn load_dbs<'a>(alpm: &'a Alpm, repos: &[impl AsRef<str>]) -> Result<Arr<&'a Db>> {
    repos
        .iter()
        .map(|repo| alpm.register_syncdb(repo.as_ref(), SigLevel::NONE))
        .collect()
}
