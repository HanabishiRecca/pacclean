use std::{
    collections::HashMap,
    fs,
    io::{self, ErrorKind, Read, Result, Write},
    path,
};

fn is_pkg_file(name: &str) -> bool {
    name.contains(".pkg.tar") && name.split('.').next_back() != Some("sig")
}

pub fn get_cached_pkgs(cachedir: &str) -> Result<HashMap<String, u64>> {
    Ok(fs::read_dir(cachedir)?
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            let meta = entry.metadata().ok()?;
            (meta.is_file() && is_pkg_file(&name)).then_some((name, meta.len()))
        })
        .collect())
}

pub fn read_answer() -> Result<bool> {
    io::stdout().flush()?;
    let mut buf = [0];
    io::stdin().read_exact(buf.as_mut_slice())?;
    let [code] = buf;
    Ok(code == b'\n' || code == b'y' || code == b'Y')
}

fn remove_file(path: &str) -> bool {
    if let Err(e) = fs::remove_file(path) {
        if e.kind() != ErrorKind::NotFound {
            println!("Failed to remove '{path}': {e}");
            return false;
        }
    }
    true
}

pub fn remove_package(cachedir: &str, name: &str) {
    let mut path = String::from_iter([cachedir, path::MAIN_SEPARATOR_STR, &name]);

    if remove_file(&path) {
        path.push_str(".sig");
        remove_file(&path);
    }
}
