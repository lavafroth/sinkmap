use anyhow::Result;
use clap::Parser;
use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceMap {
    pub version: u8,
    pub sources: Vec<String>,
    pub sources_content: Vec<String>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub uri: String,
    #[arg(short, long)]
    pub proxy: Option<String>,
    #[arg(short, long, default_value_t = String::from("./") )]
    pub output: String,
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,
}

impl SourceMap {
    pub fn new(json: &str) -> Result<SourceMap> {
        let sourcemap = serde_json::from_str(json)?;
        Ok(sourcemap)
    }

    pub fn output(&self, cli: &Cli) -> Result<()> {
        let windows_re = Regex::new(r#"[?%*|:"<>]"#).unwrap();
        for (source, content) in self.sources.iter().zip(self.sources_content.iter()) {
            let _dst;
            if cfg!(windows) {
                _dst = windows_re.replace_all(source, "");
            } else {
                _dst = std::borrow::Cow::Borrowed(&source);
            }
            let mut dst = _dst.strip_prefix("webpack:///").unwrap_or(source);
            dst = dst
                .trim_start_matches("..")
                .trim_start_matches("./")
                .trim_start_matches("/");
            let dst = Path::new(dst);

            let output = Path::new(&cli.output);
            let full_path = output.join(dst);

            let dir = full_path.parent().unwrap_or(Path::new("."));
            fs::create_dir_all(dir)?;
            fs::write(&full_path, content)?;
            println!("wrote {} bytes to {:#?}", content.len(), &full_path);
        }
        Ok(())
    }
}

fn fetch(cli: &Cli) -> Result<String> {
    let mut headers = HeaderMap::new();
    for header in cli.headers.iter() {
        match header.split_once(':') {
            Some((k, v)) => match HeaderValue::from_str(&v.to_string()) {
                Ok(value) => match HeaderName::from_bytes((&k).as_bytes()) {
                    Ok(key) => {
                        headers.insert(key, value);
                    }
                    _ => {
                        eprintln!("ignoring malformed header `{}`", header);
                    }
                },
                _ => {
                    eprintln!("ignoring malformed header `{}`", header);
                }
            },
            _ => {
                eprintln!("ignoring malformed header `{}`", header);
            }
        }
    }
    let mut client = Client::builder().default_headers(headers);
    if let Some(proxy) = &cli.proxy {
        client = client.proxy(reqwest::Proxy::all(proxy)?);
    }
    let client = client.build()?;
    let text = client.get(&cli.uri).send()?.text()?;
    Ok(text)
}

pub fn read(cli: &Cli) -> Result<SourceMap> {
    let source = match Url::parse(&cli.uri) {
        Ok(uri) => match uri.scheme() {
            "https" | "http" => fetch(cli)?,
            _ => fs::read_to_string(&cli.uri)?,
        },
        _ => fs::read_to_string(&cli.uri)?,
    };
    SourceMap::new(&source)
}

pub fn run(cli: &Cli) -> Result<()> {
    read(cli)?.output(cli)?;
    Ok(())
}
