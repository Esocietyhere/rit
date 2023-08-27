use super::getenv;
use crate::color::Color;
use crate::rbx::Remodel;
use clap::Parser;

/// Import assets and maps
#[derive(Debug, Parser)]
pub struct ImportCommand {
    /// Whether to import assets
    #[clap(short = 'A', long, takes_value = false)]
    game_assets: bool,
    /// Whether to import all maps
    #[clap(short = 'M', long, takes_value = false)]
    game_maps: bool,
    /// The path to the place file
    #[clap(short, long, value_parser)]
    file_path: Option<String>,
    /// The name of the map to import
    #[clap(short, long, value_parser)]
    map_name: Option<String>,
    /// The authentication token to use
    #[clap(short, long, value_parser)]
    auth: Option<String>,
}

impl ImportCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let auth = getenv(self.auth.clone(), "ROBLOSECURITY".to_string());
        let remodel = Remodel::new(auth);

        println!(
            "{} {}",
            Color::green().pad("Importing"),
            (if self.map_name.is_some() {
                self.map_name.clone().unwrap()
            } else if self.game_assets && self.game_maps {
                "all assets and maps".to_string()
            } else if self.game_assets {
                "all assets".to_string()
            } else if self.game_maps {
                "all maps".to_string()
            } else {
                return Ok(Some("No import options specified!".to_string()));
            })
        );

        if self.map_name.is_some() {
            if self.file_path.is_some() {
                remodel.run(
                    "importLocalMap",
                    &[
                        self.file_path.clone().unwrap(),
                        self.map_name.clone().unwrap(),
                    ],
                );
            } else {
                remodel.run("importMap", &[self.map_name.clone().unwrap()]);
            }
        } else {
            if self.game_assets {
                remodel.run("importAssets", &[]);
            }

            if self.game_maps {
                remodel.run("importAllMaps", &[]);
            }
        }

        Ok(None)
    }
}
