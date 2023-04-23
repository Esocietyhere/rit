use crate::color::Color;
use clap::Parser;
use rbxcloud::rbx::{PlaceId, PublishVersionType, RbxCloud, UniverseId};
use serde_json::Value;

#[derive(Debug, Parser)]
pub struct Universe {
    pub api_key: String,
    pub universe_id: u64,
}

impl Universe {
    pub fn new(api_key: &str, universe_id: u64) -> Universe {
        Universe {
            api_key: api_key.to_string(),
            universe_id,
        }
    }

    pub async fn publish(&self, path: &str, place_to_publish: (&String, &Value)) {
        let place_name = place_to_publish.0;
        let place_id = place_to_publish.1.as_u64().unwrap();

        let publish_version_type = PublishVersionType::Published;
        let cloud = RbxCloud::new(&self.api_key, UniverseId(self.universe_id));
        let experience = cloud.experience(PlaceId(place_id));

        // Publish place:
        let publish_result = experience.publish(path, publish_version_type).await;
        match publish_result {
            Ok(result) => {
                println!(
                    "{} {} ({}) with version number: {}",
                    Color::green().pad("Published"),
                    place_name,
                    place_id,
                    result.version_number
                );
            }
            Err(e) => {
                eprintln!("{e:?}");
            }
        }
    }
}
