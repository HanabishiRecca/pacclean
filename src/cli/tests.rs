use super::*;

macro_rules! read_args {
    ($a: expr) => {
        read_args($a.into_iter())
    };
}

fn cmp(a: &[impl AsRef<str>], b: &[impl AsRef<str>]) {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        assert_eq!(a[i].as_ref(), b[i].as_ref());
    }
}

#[test]
fn args() {
    const DBPATH: &str = "/path/to/db";
    const CACHEDIR: &str = "/path/to/cache";

    let repos = [
        "core-testing",
        "core",
        "extra-testing",
        "extra",
        "multilib-testing",
        "multilib",
    ];

    let args = [
        "--dbpath",
        DBPATH,
        "--cachedir",
        CACHEDIR,
        "--repos",
        &repos.join(","),
        "",
    ];

    let config = read_args!(args).unwrap().unwrap();
    assert_eq!(config.dbpath(), Some(DBPATH));
    assert_eq!(config.cachedir(), Some(CACHEDIR));
    cmp(config.repos().unwrap(), &repos);
}

macro_rules! test_args {
    ($a: expr, $r: expr $(,)?) => {
        assert_eq!(read_args!($a).unwrap(), $r)
    };
}

#[test]
fn no_args() {
    test_args!([""; 0], Some(Config::new()));
}

#[test]
fn help() {
    test_args!(["--dbpath", "foo", "--help", "--foo"], None);
}

macro_rules! test_error {
    ($a:expr, $r:pat $(,)?) => {
        assert!(matches!(read_args!($a), Err($r)))
    };
}

#[test]
fn error_no_value() {
    test_error!(["--dbpath"], Error::NoValue(_));
}

#[test]
fn error_unknown() {
    test_error!(["--foo"], Error::Unknown(_));
}
