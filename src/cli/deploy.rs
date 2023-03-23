use super::build::build;
use super::getenv;
use crate::config::Config;
use crate::rbx::{Message, Place};
use anyhow::Ok;
use clap::Parser;

/// Build all projects and deploy them to Roblox
#[derive(Debug, Parser)]
pub struct DeployCommand {
    /// The branch to deploy to
    #[clap(short, long, value_parser)]
    branch_name: Option<String>,
    /// The deploy message
    #[clap(short, long, value_parser)]
    message: Option<String>,
    /// The Roblox API key
    #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
    api_key: Option<String>,
}

impl DeployCommand {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        let api_key = getenv(self.api_key.clone(), "OPENCLOUD_KEY".to_string());
        let branch = match self.branch_name.clone() {
            Some(v) => v,
            None => "main".to_string(),
        };

        println!("Publishing to {} universe", branch.clone());

        let config = Config::new(branch.clone());
        let universe_id = config.get_universe_id().unwrap();
        let places = config.get_places();

        let place = Place::new(&api_key, universe_id);

        for (place_name, place_id) in places.unwrap().iter() {
            let deploy_dir = format!("deploy/{}", place_name);
            let path = build(Some(place_name.to_string()), Some(deploy_dir)).unwrap();

            place.publish(&path, place_id.as_u64().unwrap()).await;
        }

        if self.message.is_some() {
            let topic = format!("updates-{}", branch);
            Message::new(&api_key, universe_id)
                .publish(&topic, &self.message.clone().unwrap())
                .await;
        }
        Ok(None)
    }
}
