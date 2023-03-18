use crate::commands::{build, BuildParams};
use clap::Parser;
use rbxcloud::rbx::{PlaceId, PublishVersionType, RbxCloud, UniverseId};

#[derive(Debug, Parser)]
#[clap(name = "experience", about = "Manage experiences")]
pub struct Place {
    pub api_key: String,
    pub universe_id: u64,
}

impl Place {
    pub fn new(api_key: &str, universe_id: u64) -> Place {
        Place {
            api_key: api_key.to_string(),
            universe_id,
        }
    }

    pub async fn publish(&self, name: &str, place_id: u64) -> () {
        let path = format!("deploy/{}", name);
        let publish_version_type = PublishVersionType::Published;

        build(&BuildParams {
            project_name: name.to_string(),
            output_name: path.clone(),
        })
        .ok();

        // Define RbxCloud instance:
        let cloud = RbxCloud::new(&self.api_key, UniverseId(self.universe_id));
        let experience = cloud.experience(PlaceId(place_id));

        // Publish place:
        let publish_result = experience.publish(&path, publish_version_type).await;
        match publish_result {
            Ok(result) => {
                println!("Published place! New version: {}", result.version_number);
            }
            Err(e) => {
                eprintln!("{e:?}");
            }
        }
    }
}
