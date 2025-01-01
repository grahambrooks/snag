use anyhow::{Result, Context};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub workspace: Workspace,
    pub auth: HashMap<String, Auth>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub path: String,
    pub sources: Vec<Source>,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub host: String,
    pub owner: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub token: String,
}

pub fn read_config_file(path: &str) -> Result<Config> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_config(reader)
}

pub fn parse_config<R: Read>(reader: R) -> Result<Config> {
    let mut config: Config = serde_yaml::from_reader(reader)?;
    replace_env_vars(&mut config);
    Ok(config)
}

fn replace_env_vars(config: &mut Config) {
    config.workspace.path = replace_env_var(&config.workspace.path);
    for source in &mut config.workspace.sources {
        source.host = replace_env_var(&source.host);
        if let Some(owner) = &source.owner {
            source.owner = Some(replace_env_var(owner));
        }
    }
    for auth in config.auth.values_mut() {
        auth.token = replace_env_var(&auth.token);
    }
}

fn replace_env_var(value: &str) -> String {
    let re = regex::Regex::new(r"\$\{(\w+)\}").unwrap();
    re.replace_all(value, |caps: &regex::Captures| {
        env::var(&caps[1]).unwrap_or_else(|_| caps[0].to_string())
    }).to_string()
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn test_parse_config() {
        let yaml = r#"
workspace:
  path: ~/workspace
  sources:
    - host: github
      owner: grahambrooks
auth:
  github:
    token: ${GITHUB_TOKEN}
"#;
        let config = super::parse_config(yaml.as_bytes()).unwrap();
        assert_eq!(config.workspace.path, "~/workspace");
        assert_eq!(config.workspace.sources.len(), 1);
        assert_eq!(config.workspace.sources[0].host, "github");
        assert_eq!(config.workspace.sources[0].owner.as_deref(), Some("grahambrooks"));
        assert_eq!(config.auth.len(), 1);
        assert_eq!(config.auth["github"].token, env::var("GITHUB_TOKEN").unwrap());
    }

    #[test]
    fn test_read_malformed_config() {
        let yaml = r#""#;
        let result = super::parse_config(yaml.as_bytes());
        assert!(result.is_err());
    }
}