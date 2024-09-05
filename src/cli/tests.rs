use super::*;

macro_rules! S {
    ($s: expr) => {
        String::from($s)
    };
}

macro_rules! read_args {
    ($a:expr) => {
        read_args($a.into_iter())
    };
}

#[test]
fn args() {
    const DBPATH: &str = "/path/to/db";
    const CACHEDIR: &str = "/path/to/cache";

    let repos: &[String] = &[
        S!("core-testing"),
        S!("core"),
        S!("extra-testing"),
        S!("extra"),
        S!("multilib-testing"),
        S!("multilib"),
    ];

    let args = [
        S!("--dbpath"),
        S!(DBPATH),
        S!("--cachedir"),
        S!(CACHEDIR),
        S!("--repos"),
        repos.join(","),
    ];

    let config = read_args!(args).unwrap().unwrap();
    assert_eq!(config.dbpath(), Some(DBPATH));
    assert_eq!(config.cachedir(), Some(CACHEDIR));
    assert_eq!(config.repos(), Some(repos));
}

macro_rules! test_args {
    ($a:expr, $r:expr $(,)?) => {
        assert_eq!(read_args!($a).unwrap(), $r)
    };
}

#[test]
fn no_args() {
    test_args!([], Some(Config::new()));
}

#[test]
fn help() {
    test_args!([S!("--dbpath"), S!("foo"), S!("--help"), S!("--foo")], None);
}

macro_rules! test_error {
    ($a:expr, $r:pat $(,)?) => {
        assert!(matches!(read_args!($a), Err($r)))
    };
}

#[test]
fn error_no_value() {
    test_error!([S!("--dbpath")], Error::NoValue(_));
}

#[test]
fn error_unknown() {
    test_error!([S!("--foo")], Error::Unknown(_));
}
