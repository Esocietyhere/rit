use crate::commands::*;
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Parser)]
#[clap(name = "rit", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize a new project
    Init,
    /// Installs the configured devtools
    Devtools,
    /// Publish an experience
    Build {
        /// The name of the project to build
        #[clap(short, long, value_parser)]
        project_name: String,
        /// The name of the output file
        #[clap(short, long, value_parser)]
        output_name: String,
    },
    /// Open a place file
    Open {
        /// The name of the place file to open
        #[clap(short, long, value_parser)]
        file_name: String,
    },
    /// Builds the project and opens the place file
    Run {
        /// The name of the project to build
        #[clap(short, long, value_parser)]
        project_name: String,
        /// The name of the output file
        #[clap(short, long, value_parser)]
        output_name: String,
    },
    /// Builds the project and deploys it to the Roblox CDN
    Deploy,
    /// Syncs images to the Roblox CDN
    Sync {
        #[clap(short, long, value_parser, env = "ROBLOSECURITY")]
        auth: String,
    },
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            Command::Init => init(),
            Command::Devtools => devtools(),
            Command::Build {
                project_name,
                output_name,
            } => build(&BuildParams {
                project_name,
                output_name,
            }),
            Command::Open { file_name } => open_place(&OpenPlaceParams { file_name }),
            Command::Run {
                project_name,
                output_name,
            } => {
                build(&BuildParams {
                    project_name: project_name.clone(),
                    output_name: output_name.clone(),
                })?;

                open_place(&OpenPlaceParams {
                    file_name: format!(r#"build/{}.rbxl"#, output_name.clone(),),
                })?;
                Ok(None)
            }
            Command::Deploy => {
                let mut file = File::open("config.json")?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let json: Value = serde_json::from_str(&contents)?;

                println!("{:?}", json);

                Ok(None)
            }
            Command::Sync { auth } => img_sync(&SyncParams { auth }),
        }
    }
}
