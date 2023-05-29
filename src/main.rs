mod cli;

use home::home_dir;
use log::*;
use simplelog::*;
use std::{
    error::Error,
    fs::{self, File},
};

/// Initialize logging to a file and stdout
fn init_logging() -> Result<(), Box<dyn Error>> {
    let log_file_path = home_dir()
        .ok_or("Failed to get home directory")?
        .join(".local")
        .join("state")
        .join("fun_interpreter.log");

    println!("Logs saved to {log_file_path:?}");

    fs::create_dir_all(log_file_path.parent().unwrap())?;

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging()?;
    let args = cli::parse_args();
    Ok(())
}
