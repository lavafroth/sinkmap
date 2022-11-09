use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = sinkmap::Cli::parse();
    sinkmap::run(&args)
}
