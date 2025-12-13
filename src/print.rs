use crate::byte_format::ByteFormat;
use crate::package::Pkg;
use std::env;
use std::fmt::Display;
use std::io::{self, Read, Result, Write};

pub fn help() {
    let bin = env::current_exe().ok();
    println!(
        include_str!("help.in"),
        PKG = env!("CARGO_PKG_NAME"),
        VER = env!("CARGO_PKG_VERSION"),
        BIN_NAME = (|| bin.as_ref()?.file_name()?.to_str())().unwrap_or(env!("CARGO_BIN_NAME")),
    );
}

pub fn request(message: impl Display) -> Result<bool> {
    print!("\x1b[34;1m::\x1b[0;1m {message} [Y/n] \x1b[0m");
    io::stdout().flush()?;
    let mut buf = [0];
    io::stdin().read_exact(buf.as_mut_slice())?;
    let [code] = buf;
    Ok(code == b'\n' || code == b'y' || code == b'Y')
}

pub fn message(message: impl Display) {
    println!("\x1b[0m{message}\x1b[0m");
}

pub fn size(name: impl Display, size: u64) {
    println!("\x1b[0;1m{name} \x1b[0m(\x1b[32;1m{}\x1b[0m)\x1b[0m", ByteFormat(size));
}

pub fn pkg(pkg: &Pkg) {
    size(pkg.name(), pkg.size());
}

pub fn error(e: impl Display) {
    eprintln!("\x1b[31;1merror:\x1b[0m {e}\x1b[0m");
}

pub fn warning(w: impl Display) {
    eprintln!("\x1b[33;1mwarning:\x1b[0m {w}\x1b[0m");
}
