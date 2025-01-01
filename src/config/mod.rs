use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub workspace: Workspace,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub path: String,
    pub sources: Vec<Source>,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub href: String,
}

pub fn read_config_file(path: &str) -> Result<Config> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_config(reader)
}

pub fn parse_config<R: Read>(reader: R) -> Result<Config> {
    let config: Config = serde_yaml::from_reader(reader)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_config() {
        let yaml = r#"
workspace:
  path: ~/workspace
  sources:
   - href: github.com/grahambrooks
"#;
        let config = super::parse_config(yaml.as_bytes()).unwrap();
        assert_eq!(config.workspace.path, "~/workspace");
        assert_eq!(config.workspace.sources.len(), 1);
        assert_eq!(config.workspace.sources[0].href, "github.com/grahambrooks");
    }

    #[test]
    fn test_read_malformed_config() {
        let yaml = r#""#;
        let result = super::parse_config(yaml.as_bytes());
        assert!(result.is_err());
    }
}
