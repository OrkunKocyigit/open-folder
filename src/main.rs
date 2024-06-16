use std::env;
use std::path::PathBuf;
use log::{debug};
#[cfg(feature = "debug")]
use std::fs::OpenOptions;
#[cfg(feature = "debug")]
use log::{LevelFilter};
#[cfg(feature = "debug")]
use simplelog::{Config, WriteLogger};

fn main() {
    setup_logger();
    let path = get_file_path();
    open_path(path);
}

fn open_path(path: PathBuf) {
    debug!("Opening {} path", path.display());
    opener::reveal(path).expect("Dict open failed");
    debug!("Path opened successfully");
}

fn get_file_path() -> PathBuf {
    debug!("Getting file path");
    if let Some(first_arg) = env::args().nth(1) {
        debug!("First console parameter is {}", first_arg);
        let file_path = PathBuf::from(first_arg).to_path_buf();
        debug!("File path parsed as {}", file_path.display());
        file_path
    } else {
        debug!("No first parameter given");
        panic!("First parameter is missing!")
    }
}

#[cfg(feature = "debug")]
fn setup_logger() {
    let file_path = env::current_exe().expect("Executable path not found").parent().expect("Executable parent not found").join("open-folder.log");
    let log_file = OpenOptions::new().create(true).append(true).open(file_path).expect("Failed to create log file");
    WriteLogger::init(LevelFilter::Debug, Config::default(), log_file).expect("Logger init failed");
    debug!("Logger setup");
}

#[cfg(not(feature = "debug"))]
fn setup_logger() {
    // Do nothing
}