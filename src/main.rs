use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = sinkmap::Cli::parse();
    sinkmap::run(&args)
}
