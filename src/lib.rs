pub mod token;
pub mod token_type;

use log::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
};
pub type EmpResult = Result<(), Box<dyn std::error::Error>>;

pub static had_error: AtomicBool = AtomicBool::new(false);

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, loc: &str, message: &str) {
    info!("[line {}] Error {} : {}", line, loc, message);
}

pub fn run_file(path: PathBuf) -> EmpResult {
    let contents = fs::read_to_string(path)?;
    run(&contents);
    Ok(())
}

pub fn run_prompt() -> EmpResult {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            return Ok(());
        }

        run(input);
    }
}

pub fn run(source: &str) -> EmpResult {
    // let scanner = Scanner::new(source);
    // tokens = scanner.scan_tokens();

    // for token in tokens {
    //     println!("{token:?}");
    // }

    Ok(())
}
