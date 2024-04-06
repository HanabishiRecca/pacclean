mod app;
mod consts;
mod error;
mod pacman_conf;

use std::process::ExitCode;

fn main() -> ExitCode {
    match app::run() {
        Err(e) => {
            println!("{e}");
            ExitCode::FAILURE
        }
        _ => ExitCode::SUCCESS,
    }
}
