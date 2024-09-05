use crate::byte_format::ByteFormat;
use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    io::{self, ErrorKind, Read, Result, Write},
    path,
};

fn is_pkg(name: &str) -> bool {
    name.contains(".pkg.tar") && name.split('.').next_back() != Some("sig")
}

pub fn get_cached_pkgs(cachedir: &str) -> Result<HashMap<String, u64>> {
    let mut pkgs = HashMap::new();

    for entry in fs::read_dir(cachedir)? {
        let entry = entry?;

        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };

        if !is_pkg(&name) {
            continue;
        }

        let meta = entry.metadata()?;
        if !meta.is_file() {
            continue;
        }

        pkgs.insert(name, meta.len());
    }

    Ok(pkgs)
}

fn remove_file(path: &str) -> bool {
    if let Err(e) = fs::remove_file(path) {
        if e.kind() != ErrorKind::NotFound {
            print_warning(format_args!("failed to remove '{path}': {e}"));
            return false;
        }
    }
    true
}

pub fn remove_pkg(cachedir: &str, name: &str) {
    let mut path = String::from_iter([cachedir, path::MAIN_SEPARATOR_STR, name]);

    if remove_file(&path) {
        path.push_str(".sig");
        remove_file(&path);
    }
}

pub fn make_request(message: impl Display) -> Result<bool> {
    print!("\x1b[34;1m::\x1b[0;1m {message} [Y/n] \x1b[0m");
    io::stdout().flush()?;
    let mut buf = [0];
    io::stdin().read_exact(buf.as_mut_slice())?;
    let [code] = buf;
    Ok(code == b'\n' || code == b'y' || code == b'Y')
}

pub fn print_message(message: impl Display) {
    println!("\x1b[0m{message}\x1b[0m");
}

pub fn print_pkg(name: impl Display, size: u64) {
    println!(
        "\x1b[0;1m{name} \x1b[0m(\x1b[32;1m{}\x1b[0m)\x1b[0m",
        ByteFormat(size),
    );
}

pub fn print_error(e: impl Display) {
    eprintln!("\x1b[31;1merror:\x1b[0m {e}\x1b[0m");
}

pub fn print_warning(w: impl Display) {
    eprintln!("\x1b[33;1mwarning:\x1b[0m {w}\x1b[0m");
}
