use clap::Parser;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(short, long, value_parser)]
    pub json: Value,

    #[clap(short, long, value_parser)]
    pub branch: String,
}

fn get_config() -> String {
    if Path::new("config.json").exists() {
        "config.json".to_string()
    } else {
        "remodel/config.json".to_string()
    }
}

impl Config {
    pub fn new(branch: String) -> Self {
        let mut file = File::open(get_config()).expect("Unable to open config.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read config.json");
        let json: Value = serde_json::from_str(&contents).unwrap();
        Self { json, branch }
    }

    pub fn get_universe_id(&self) -> Result<u64, anyhow::Error> {
        let universe_id = &self
            .json
            .get("deployment")
            .unwrap()
            .get("universes")
            .unwrap()
            .get(&self.branch);
        match universe_id {
            Some(v) => Ok(Some(v).unwrap().as_u64().unwrap()),
            None => Err(anyhow::anyhow!(
                "No universe id found for branch {}",
                &self.branch
            )),
        }
    }

    pub fn get_places(
        &self,
    ) -> Result<&serde_json::Map<std::string::String, Value>, anyhow::Error> {
        let places = &self
            .json
            .get("deployment")
            .unwrap()
            .get("places")
            .unwrap()
            .get(&self.branch);
        match places {
            Some(v) => Ok(Some(v).unwrap().as_object().unwrap()),
            None => Err(anyhow::anyhow!(
                "No places found for branch {}",
                &self.branch
            )),
        }
    }
}
