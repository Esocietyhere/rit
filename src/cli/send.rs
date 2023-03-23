use clap::Parser;

use super::getenv;
use crate::config::Config;
use crate::rbx::Message;

/// Send a message to MessageService
#[derive(Debug, Parser)]
pub struct SendCommand {
    /// The branch to send the message to
    #[clap(short, long, value_parser)]
    branch_name: Option<String>,
    /// Determines where the message is sent.
    #[clap(short, long, value_parser)]
    topic: Option<String>,
    /// The data to include in the message.
    #[clap(short, long, value_parser)]
    message: Option<String>,
    /// The Roblox API key
    #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
    api_key: Option<String>,
}

impl SendCommand {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        let api_key = getenv(self.api_key.clone(), "OPENCLOUD_KEY".to_string());
        let branch = match self.branch_name.clone() {
            Some(v) => v,
            None => "main".to_string(),
        };
        let config = Config::new(branch);
        let universe_id = config.get_universe_id().unwrap();

        Message::new(&api_key, universe_id)
            .publish(&self.topic.clone().unwrap(), &self.message.clone().unwrap())
            .await;
        Ok(None)
    }
}
