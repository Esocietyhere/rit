use ansi_term::Colour;
use clap::Parser;
use rbxcloud::rbx::{RbxCloud, UniverseId};

#[derive(Debug, Parser)]
pub struct Message {
    pub api_key: String,
    pub universe_id: u64,
}

impl Message {
    pub fn new(api_key: &str, universe_id: u64) -> Message {
        Message {
            api_key: api_key.to_string(),
            universe_id,
        }
    }

    pub async fn publish(&self, topic: &str, data: &str) {
        let cloud = RbxCloud::new(&self.api_key, UniverseId(self.universe_id));
        let messaging = cloud.messaging(topic);

        messaging.publish(data).await.ok();
        println!(
            "{} message \"{}\" with topic: {}",
            Colour::Green.bold().paint("Published"),
            data,
            topic
        );
    }
}
