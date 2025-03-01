use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Route {
    pub name: String,
    pub context: String,
    pub target: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub listen_addr: String,
    pub routes: Vec<Route>,
}

impl Config {
    pub fn from_yaml(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn find_route(&self, path: &str) -> Option<&Route> {
        self.routes.iter().find(|route| path.starts_with(&route.context))
    }
} 