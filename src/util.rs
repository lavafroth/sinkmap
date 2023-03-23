use color_eyre::eyre::{eyre, ContextCompat, Result};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};

use std::str::FromStr;

pub fn parse_raw_headers(raw: &[String]) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for header in raw.iter() {
        add_header(&mut headers, header).unwrap_or_else(|e| {
            eprintln!("warning: ignoring malformed header `{header}`: {e}");
        });
    }
    headers
}

fn add_header(headers: &mut HeaderMap, raw: &str) -> Result<()> {
    let (k, v) = raw
        .split_once(':')
        .wrap_err(eyre!("failed to split header string with delimiter ':'"))?;
    headers
        .insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?)
        .wrap_err(eyre!(
            "failed to insert key `{k}` and value `{v}` into headers"
        ))?;
    Ok(())
}

pub fn url_is_ok(uri: &str) -> bool {
    Url::parse(uri).map(|url| matches!(url.scheme(), "https" | "http")) == Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unix_paths_are_not_urls() {
        assert!(!url_is_ok("foo/bar/baz.js.map"));
        assert!(!url_is_ok("./foo/bar/baz.js.map"));
        assert!(!url_is_ok("../foo/bar/baz.js.map"));
        assert!(!url_is_ok("/usr/share/baz.js.map"));
    }
    #[test]
    fn windows_paths_are_not_urls() {
        assert!(!url_is_ok("foo\\baz.js.map"));
        assert!(!url_is_ok(".\\foo\\baz.js.map"));
        assert!(!url_is_ok("..\\foo\\baz.js.map"));
        assert!(!url_is_ok("C:\\Windows\\Tasks\\baz.js.map"));
    }
    #[test]
    fn unusable_urls() {
        assert!(!url_is_ok("http//example.com/foo"));
        assert!(!url_is_ok("rust@http://example.com/foo"));
        assert!(!url_is_ok("rust@https://example.com/foo"));
        assert!(!url_is_ok("https::example.com/foo"));
        assert!(!url_is_ok("ftp:/example.com/foo"));
        assert!(!url_is_ok("ftp://example.com/foo"));
    }
    #[test]
    fn valid_urls() {
        assert!(url_is_ok("http://example.com/foo"));
        assert!(url_is_ok("http://localhost/foo.js.map"));
        assert!(url_is_ok("http://example.com/foo.js.map"));
        assert!(url_is_ok("https://example.com/foo"));
        assert!(url_is_ok("https://example.com/foo.js.map"));
    }
}
