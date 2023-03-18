use crate::commands::{build, BuildParams};
use crate::rbx::Place;
use anyhow::Ok;
use clap::Parser;
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
    pub api_key: Option<String>,
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

    pub fn get_universe_id(&self) -> Result<u64, anyhow::Error> {
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
        return match places {
            Some(v) => Ok(Some(v).unwrap().as_object().unwrap()),
            None => Err(anyhow::anyhow!(
                "No places found for branch {}",
                &self.branch
            )),
        };
    }
}

pub async fn deploy(params: &DeployParams) -> anyhow::Result<Option<String>> {
    let branch = match params.branch_name.clone() {
        Some(v) => v,
        None => "main".to_string(),
    };

    let api_key = match params.api_key.clone() {
        Some(v) => v,
        None => env::var("OPENCLOUD_KEY").expect("OPENCLOUD_KEY not set"),
    };

    let config = Config::new(branch);
    let universe_id = config.get_universe_id();
    let places = config.get_places();

    let place = Place::new(&api_key, universe_id.unwrap());

    for (place_name, place_id) in places.unwrap().iter() {
        let deploy_dir = format!("deploy/{}", place_name);
        let path = build(&BuildParams {
            project_name: Some(place_name.to_string()),
            output_name: Some(deploy_dir),
        })
        .unwrap();

        place.publish(&path, place_id.as_u64().unwrap()).await;
    }
    Ok(None)
}
