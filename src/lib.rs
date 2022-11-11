use clap::{Args, Parser};
use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};
use serde::Deserialize;
use std::error::Error;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceMap {
    version: u8,
    sources: Vec<String>,
    sources_content: Vec<String>,
}

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

impl SourceMap {
    fn new(json: String) -> Result<SourceMap, Box<dyn Error>> {
        let sourcemap: SourceMap = serde_json::from_str(&json)?;
        if sourcemap.version > 3 {
            eprintln!("warning: detected untested version for sourcemap");
        }
        Ok(sourcemap)
    }

    pub fn output(&self, out_path: &str) -> Result<(), Box<dyn Error>> {
        let windows_re = Regex::new(r#"[?%*|:"<>]"#).unwrap();
        for (source, content) in self.sources.iter().zip(self.sources_content.iter()) {
            let _dst = if cfg!(windows) {
                windows_re.replace_all(source, "")
            } else {
                std::borrow::Cow::Borrowed(&source[..])
            };
            let mut full_path = std::path::PathBuf::from(out_path);
            full_path.push(
                _dst.strip_prefix("webpack:///")
                    .unwrap_or(&_dst)
                    .trim_start_matches(['.', '/']),
            );
            fs::create_dir_all(full_path.parent().unwrap_or_else(|| Path::new(".")))?;
            fs::write(&full_path, content)?;
            println!("wrote {} bytes to {:#?}", content.len(), &full_path);
        }
        Ok(())
    }
}

fn add_header(headers: &mut HeaderMap, raw: &str) -> Result<(), Box<dyn Error>> {
    let (k, v) = raw
        .split_once(':')
        .ok_or("failed to split header string with delimiter ':'")?;
    headers
        .insert(
            HeaderName::from_bytes(k.as_bytes())?,
            HeaderValue::from_str(v)?,
        )
        .ok_or(format!(
            "failed to insert key `{k}` and value `{v}` into headers"
        ))?;
    Ok(())
}

pub fn fetch(config: &Config) -> Result<String, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    for raw in config.headers.iter() {
        if let Err(e) = add_header(&mut headers, raw) {
            eprintln!("ignoring malformed header `{}`: {}", raw, e);
        }
    }
    let mut client = Client::builder().default_headers(headers);
    if let Some(proxy) = &config.proxy {
        client = client.proxy(reqwest::Proxy::all(proxy)?);
    }
    Ok(client.build()?.get(&config.uri).send()?.text()?)
}

pub fn is_url(uri: &str) -> bool {
    Url::parse(uri).map(|url| matches!(url.scheme(), "https" | "http")) == Ok(true)
}

pub fn read_resource(config: &Config) -> Result<SourceMap, Box<dyn Error>> {
    SourceMap::new(if is_url(&config.uri) {
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
    fn unix_paths_are_not_urls() {
        assert!(!is_url("foo/bar/baz.js.map"));
        assert!(!is_url("./foo/bar/baz.js.map"));
        assert!(!is_url("../foo/bar/baz.js.map"));
        assert!(!is_url("/usr/share/baz.js.map"));
    }
    #[test]
    fn windows_paths_are_not_urls() {
        assert!(!is_url("foo\\baz.js.map"));
        assert!(!is_url(".\\foo\\baz.js.map"));
        assert!(!is_url("..\\foo\\baz.js.map"));
        assert!(!is_url("C:\\Windows\\Tasks\\baz.js.map"));
    }
    #[test]
    fn unusable_urls() {
        assert!(!is_url("http//example.com/foo"));
        assert!(!is_url("rust@http://example.com/foo"));
        assert!(!is_url("rust@https://example.com/foo"));
        assert!(!is_url("https::example.com/foo"));
        assert!(!is_url("ftp:/example.com/foo"));
        assert!(!is_url("ftp://example.com/foo"));
    }
    #[test]
    fn valid_urls() {
        assert!(is_url("http://example.com/foo"));
        assert!(is_url("http://localhost/foo.js.map"));
        assert!(is_url("http://example.com/foo.js.map"));
        assert!(is_url("https://example.com/foo"));
        assert!(is_url("https://example.com/foo.js.map"));
    }
    #[test]
    fn unmap_docsearch_js() -> Result<(), Box<dyn Error>> {
        let sourcemap = read_resource(&Config {
            uri: "https://unpkg.com/docsearch.js@2.4.1/dist/cdn/docsearch.min.js.map".to_string(),
            headers: vec![],
            proxy: None,
        })?;
        assert_eq!(sourcemap.sources.len(), 65);
        assert_eq!(sourcemap.sources.len(), sourcemap.sources_content.len());
        assert_eq!(sourcemap.version, 3);
        Ok(())
    }
}
