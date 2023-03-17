use clap::{Parser, Subcommand};
use cli::studio::{build, BuildParams};
use rbxcloud::rbx::{error::Error, PlaceId, RbxCloud, UniverseId};
use std::process::Command;

#[derive(Debug, Parser)]
#[clap(name = "experience", about = "Manage experiences")]
pub struct Experience {
    pub api_key: String,
    pub universe_id: UniverseId,
}

impl Experience {
    pub fn new(api_key: &str, universe_id: UniverseId) -> Experience {
        Experience {
            api_key: api_key.to_string(),
            universe_id,
        }
    }

    pub fn deploy(&self, place_id: PlaceId, name: &str) -> Result<(), Error> {
        let deployDir = "build/deploy";
        let path = format!("{}/{}.rbxlx", deployDir, name);

        build(&BuildParams {
            project_name: name.to_string(),
            output_name: path.clone(),
        })?;

        let cloud = RbxCloud::new(&self.api_key, self.universe_id);
        let experience = cloud.experience(place_id);
        experience.publish(&path)?;
    }

    pub fn print(&self, place_id: PlaceId) -> () {
        println!("Place ID: {}", place_id);
    }
}
