use crate::rbx::Experience;
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(short, long, value_parser)]
    pub json: Value,

    #[clap(short, long, value_parser)]
    pub branch: String,
}

pub struct DeployParams {
    pub branch_name: Option<String>,
    pub api_key: String,
}

impl Config {
    pub fn new(branch: String) -> Self {
        let mut file = File::open("config.json").expect("Unable to open config.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read config.json");
        let json: Value = serde_json::from_str(&contents).unwrap();
        Self { json, branch }
    }

    pub fn get_universe_id(&self) -> (Result<u64, anyhow::Error>) {
        let universe_id = &self
            .json
            .get("deployment")
            .unwrap()
            .get("universes")
            .unwrap()
            .get(&self.branch);
        return match universe_id {
            Some(v) => Ok(Some(v).unwrap().as_u64().unwrap()),
            None => Err(anyhow::anyhow!(
                "No universe id found for branch {}",
                &self.branch
            )),
        };
    }

    pub fn get_places(&self) -> Result<&Vec<Value>, anyhow::Error> {
        let places = &self
            .json
            .get("deployment")
            .unwrap()
            .get("places")
            .unwrap()
            .get(&self.branch);
        return match places {
            Some(v) => Ok(Some(v).unwrap().as_array().unwrap()),
            None => Err(anyhow::anyhow!(
                "No places found for branch {}",
                &self.branch
            )),
        };
    }
}

pub fn deploy(params: &DeployParams) -> anyhow::Result<Option<String>> {
    let branch = match params.branch_name.clone() {
        Some(v) => v,
        None => "master".to_string(),
    };

    let config = Config::new(params.branch_name.clone().unwrap());
    let universe_id = config.get_universe_id();
    let places = config.get_places();
    Ok(None)
}
