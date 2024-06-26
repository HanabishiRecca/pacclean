use super::read_config;
use std::io::{BufReader, Cursor, Result};

#[test]
fn config() -> Result<()> {
    let reader = BufReader::new(Cursor::new(include_bytes!("test_config.in")));
    let conf = read_config(reader)?;
    assert_eq!(conf.dbpath.as_deref(), Some("/var/lib/pacman/"));
    assert_eq!(conf.repos, ["core", "extra", "multilib"]);
    Ok(())
}

#[test]
fn config_no_dbpath() -> Result<()> {
    let reader = BufReader::new(Cursor::new(include_bytes!("test_config_no_dbpath.in")));
    let conf = read_config(reader)?;
    assert!(conf.dbpath.is_none());
    assert_eq!(conf.repos, ["core"]);
    Ok(())
}

#[test]
fn config_empty() -> Result<()> {
    let reader = BufReader::new(Cursor::new([]));
    let conf = read_config(reader)?;
    assert!(conf.dbpath.is_none());
    assert!(conf.repos.is_empty());
    Ok(())
}
