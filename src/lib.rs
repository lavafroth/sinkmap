mod sourcemap;
mod util;
pub use crate::sourcemap::SourceMap;
use clap::{Args, Parser};
use reqwest::blocking::Client;
use std::{error::Error, fs};

#[derive(Args)]
pub struct Config {
    #[arg(short, long)]
    pub uri: String,
    #[arg(short, long)]
    pub proxy: Option<String>,
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,
}

#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(about = "Rust tool to extract JavaScript source code from sourcemap files")]
pub struct Cli {
    #[command(flatten)]
    config: Config,
    #[arg(short, long, default_value_t = String::from(".") )]
    output: String,
}

pub fn fetch(config: &Config) -> Result<String, Box<dyn Error>> {
    let mut client = Client::builder().default_headers(util::parse_raw_headers(&config.headers));
    if let Some(proxy) = &config.proxy {
        client = client.proxy(reqwest::Proxy::all(proxy)?);
    }
    Ok(client.build()?.get(&config.uri).send()?.text()?)
}

pub fn read_resource(config: &Config) -> Result<SourceMap, Box<dyn Error>> {
    SourceMap::new(if util::url_is_ok(&config.uri) {
        fetch(config)?
    } else {
        fs::read_to_string(&config.uri)?
    })
}

pub fn run(cli: &Cli) -> Result<(), Box<dyn Error>> {
    read_resource(&cli.config)?.output(&cli.output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unmap_docsearch_js() -> Result<(), Box<dyn Error>> {
        let sourcemap = read_resource(&Config {
            uri: "https://unpkg.com/docsearch.js@2.4.1/dist/cdn/docsearch.min.js.map".to_string(),
            headers: vec![],
            proxy: None,
        })?;
        assert_eq!(sourcemap.sources().len(), 65);
        assert_eq!(sourcemap.version(), 3);
        Ok(())
    }
}
