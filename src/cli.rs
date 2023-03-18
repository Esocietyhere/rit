use crate::commands::*;
use clap::{Parser, Subcommand};
use std::env;

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
    Deploy {
        /// The branch to deploy to
        #[clap(short, long, value_parser)]
        branch_name: Option<String>,
        /// The Roblox API key
        #[clap(short, long, value_parser, env = "OPENCLOUD_KEY")]
        api_key: Option<String>,
    },
    /// Syncs images to the Roblox CDN
    Sync {
        #[clap(short, long, value_parser, env = "ROBLOSECURITY")]
        auth: Option<String>,
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
            Command::Deploy {
                branch_name,
                api_key,
            } => deploy(&DeployParams {
                branch_name: branch_name,
                api_key: api_key.unwrap_or(env::var("OPENCLOUD_KEY").unwrap()),
            }),
            Command::Sync { auth } => img_sync(&SyncParams {
                auth: auth.unwrap_or(env::var("ROBLOSECURITY").unwrap()),
            }),
        }
    }
}
