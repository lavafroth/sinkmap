use color_eyre::eyre::{eyre, Result};
use regex::Regex;
use serde::Deserialize;
use std::vec::IntoIter;
use std::{
    fs,
    iter::Zip,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceMap {
    version: u8,
    sources: Vec<String>,
    sources_content: Vec<String>,
}

impl SourceMap {
    pub fn new(json: &str) -> Result<SourceMap> {
        let sourcemap: SourceMap = serde_json::from_str(json)
            .map_err(|e| eyre!("failed to parse JSON body into sourcemap structure: {e}"))?;
        if sourcemap.version > 3 {
            eprintln!("warning: detected untested version for sourcemap");
        }
        if sourcemap.sources.is_empty() {
            return Err(eyre!("sourcemap contains no source files"));
        }
        if sourcemap.sources.len() != sourcemap.sources_content.len() {
            return Err(eyre!(
                "number of source files does not equal number of content entries",
            ));
        }
        Ok(sourcemap)
    }

    pub fn sources(&self) -> &Vec<String> {
        &self.sources
    }

    pub fn sources_content(&self) -> &Vec<String> {
        &self.sources_content
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn output(self, out_path: &str) -> Result<()> {
        let windows_re = Regex::new(r#"[?%*|:"<>]"#).unwrap();
        for (mut source, content) in self.into_iter() {
            if cfg!(windows) {
                source = windows_re.replace_all(&source, "").to_string()
            }
            let full_path: PathBuf = [
                out_path,
                source
                    .trim_start_matches("webpack:///")
                    .trim_start_matches(['.', '/']),
            ]
            .iter()
            .collect();
            fs::create_dir_all(full_path.parent().unwrap_or(Path::new(".")))?;
            fs::write(&full_path, &content)?;
            println!("wrote {} bytes to {}", content.len(), full_path.display());
        }
        Ok(())
    }
}

impl IntoIterator for SourceMap {
    type IntoIter = Zip<IntoIter<String>, IntoIter<String>>;
    type Item = (String, String);

    fn into_iter(self) -> Self::IntoIter {
        self.sources
            .into_iter()
            .zip(self.sources_content.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_sourcemap() {
        SourceMap::new("{\"foo\":\"bar\"}".into()).unwrap();
    }

    #[test]
    #[should_panic(expected = "missing field `sources`")]
    fn sourcemap_without_sources() {
        SourceMap::new("{\"version\":3,\"mappings\":\"kIAAAA,EAAOC,QAAU,iE,iBCAjBD\"}".into())
            .unwrap();
    }

    #[test]
    fn valid_sourcemap() {
        let sourcemap = SourceMap::new("{\"version\":3,\"mappings\":\"kIAAAA,EAAOC,QAAU,iE,iBCAjBD\",\"sources\":[\"index.js\",\"boo.js\"],\"sourcesContent\":[\"alert('xss');\",\"console.log(1)\"]}".into()).unwrap();
        assert_eq!(sourcemap.version(), 3);
        assert_eq!(sourcemap.into_iter().count(), 2);
    }
}
