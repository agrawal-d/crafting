use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "My fun programming language interpreter"
)]
pub struct Cli {
    #[arg(short, long, help = "Input file to use")]
    pub file: PathBuf,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
