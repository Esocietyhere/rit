use clap::Parser;
use rbxcloud::rbx::{error::Error, RbxCloud, UniverseId};

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

    pub async fn publish(&self, topic: &str, text: &str) -> Result<(), Error> {
        // Define RbxCloud Messaging instance:
        let cloud = RbxCloud::new(&self.api_key, UniverseId(self.universe_id));
        let messaging = cloud.messaging(topic);

        messaging.publish(text).await
    }
}
