use super::build::build;
use super::getenv;
use crate::color::Color;
use crate::config::Config;
use crate::rbx::{Message, Universe};
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
    #[clap(short, long, value_parser)]
    api_key: Option<String>,
}

impl DeployCommand {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        let api_key = getenv(self.api_key.clone(), "OPENCLOUD_KEY".to_string());
        let branch = match self.branch_name.clone() {
            Some(v) => v,
            None => "main".to_string(),
        };

        println!(
            "{} to {} universe",
            Color::green().paint("Publishing"),
            branch.clone()
        );

        let config = Config::new(branch.clone());
        let universe_id = config.get_universe_id().expect("Universe ID not found");
        let places = config.get_places();

        let universe = Universe::new(&api_key, universe_id);

        for (_, place_to_publish) in places.unwrap().iter().enumerate() {
            let deploy_dir = format!("deploy/{}", place_to_publish.0);
            let path = build(Some(place_to_publish.0.to_string()), Some(deploy_dir)).unwrap();

            universe.publish(&path, place_to_publish).await;
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
