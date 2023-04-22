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

        let mut output = "none".to_string();

        if self.map_name.is_some() {
            let mut map_type = "map";
            let args = &[
                self.file_path.clone().unwrap(),
                self.map_name.clone().unwrap(),
            ];

            if self.file_path.is_some() {
                map_type = "local-map";
            };

            remodel.run(&format!("import-{}.lua", map_type), args);
        } else {
            if self.game_assets {
                let asset_type = "assets";
                remodel.run(&format!("import-{}.lua", asset_type), &[]);

                output.push_str(asset_type);
            }

            if self.game_maps {
                let asset_type = "all-maps";
                remodel.run(&format!("import-{}.lua", asset_type), &[]);

                output.push_str(
                    format!("{}{}", if self.game_assets { ", " } else { "" }, asset_type).as_str(),
                );
            }
        }

        Ok(Some(format!(
            "{} {}",
            Color::green().paint("Importing"),
            self.map_name.clone().unwrap_or(output)
        )))
    }
}
