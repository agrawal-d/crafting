#[macro_use]
extern crate lazy_static;

pub mod ast;
pub mod ast_printer;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod token_type;

use log::*;
use scanner::Scanner;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    fs::{self},
    path::PathBuf,
};

pub type EmpResult = Result<(), Box<dyn std::error::Error>>;
pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub fn error(line: usize, message: &str) {
    HAD_ERROR.store(true, Ordering::SeqCst);
    report(line, "", message);
}

fn report(line: usize, loc: &str, message: &str) {
    info!("[line {}] Error {} : {}", line, loc, message);
}

pub fn run_file(path: PathBuf) -> EmpResult {
    println!("Running {:?}", path);
    let contents = fs::read_to_string(path)?;
    run(&contents)
}

pub fn run_prompt() -> EmpResult {
    println!("REPL mode: Type code to run");
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            return Ok(());
        }

        if let Err(err) = run(input) {
            error!("Error while running code: {:?}", err);
        }
    }
}

pub fn run(source: &str) -> EmpResult {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{token}");
    }

    Ok(())
}
