mod sourcemap;
mod util;
pub use crate::sourcemap::SourceMap;
use clap::{Args, Parser};
use color_eyre::eyre::Result;
use reqwest::{blocking::Client, Proxy};
use std::fs::read_to_string;

#[derive(Args)]
pub struct Config {
    /// URI that points to the sourcemap (".js.map") file to be extracted
    #[arg(short, long)]
    pub uri: String,
    /// Proxy URL to use when fetching sourcemap
    #[arg(short, long)]
    pub proxy: Option<String>,
    /// Headers to use when send request for the sourcemap file
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,
}

#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(about = "Rust tool to extract JavaScript source code from sourcemap files")]
pub struct Cli {
    #[command(flatten)]
    config: Config,
    /// Directory to output files from the sourcemap into
    #[arg(short, long, default_value_t = String::from(".") )]
    output: String,
}

pub fn fetch(config: &Config) -> Result<String> {
    let mut client = Client::builder().default_headers(util::parse_raw_headers(&config.headers));
    if let Some(proxy) = &config.proxy {
        client = client.proxy(Proxy::all(proxy)?);
    }
    Ok(client.build()?.get(&config.uri).send()?.text()?)
}

pub fn read_resource(config: &Config) -> Result<SourceMap> {
    let json = if util::url_is_ok(&config.uri) {
        fetch(config)?
    } else {
        read_to_string(&config.uri)?
    };
    SourceMap::new(&json)
}

pub fn run(cli: &Cli) -> Result<()> {
    read_resource(&cli.config)?.output(&cli.output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unmap_docsearch_js() -> Result<()> {
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
