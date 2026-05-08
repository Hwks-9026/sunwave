use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author, 
    version, 
    about = "A repl and interpreter for expressions and closures.", 
    long_about = None, 
    )]
pub struct Args {
    #[arg(value_name = "FILE")]
    pub input: Option<PathBuf>,
}

pub fn parse() -> Args {
    Args::parse()
}

