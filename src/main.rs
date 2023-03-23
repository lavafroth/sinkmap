use clap::Parser;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let args = sinkmap::Cli::parse();
    sinkmap::run(&args)
}
