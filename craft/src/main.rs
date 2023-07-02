mod cli;

use craft::{run_file, run_prompt, EmpResult, HAD_ERROR};
use home::home_dir;
use log::*;
use simplelog::*;
use std::{
    fs::{self, File},
    sync::atomic::Ordering,
};

/// Initialize logging to a file and stdout
fn init_logging() -> EmpResult {
    let log_file_path = home_dir()
        .ok_or("Failed to get home directory")?
        .join(".local")
        .join("state")
        .join("fun_interpreter.log");

    println!("Logs saved to {log_file_path:?}");

    fs::create_dir_all(log_file_path.parent().unwrap())?;

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create(log_file_path).unwrap(),
        ),
    ])?;

    info!("Logging initialized");

    Ok(())
}

fn main() -> EmpResult {
    init_logging()?;
    let args = cli::parse_args();

    match args.file {
        Some(path) => run_file(path),
        None => run_prompt(),
    }
    .expect("Failed to run interpreter");

    if HAD_ERROR.load(Ordering::SeqCst) {
        Err("Encountered error(s) while running")?
    }

    Ok(())
}
