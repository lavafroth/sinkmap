use regex::Regex;
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

impl SourceMap {
    pub fn new(json: String) -> Result<SourceMap, Box<dyn Error>> {
        let sourcemap: SourceMap = serde_json::from_str(&json)
            .map_err(|e| format!("failed to parse JSON body into sourcemap structure: {e}"))?;
        if sourcemap.version > 3 {
            eprintln!("warning: detected untested version for sourcemap");
        }
        if sourcemap.sources.is_empty() {
            return Err(Box::from("sourcemap contains no source files"));
        }
        if sourcemap.sources.len() != sourcemap.sources_content.len() {
            return Err(Box::from(
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

//  #[cfg(test)]
//  mod tests {
//      use super::*;
//  }
