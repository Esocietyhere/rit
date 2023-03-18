use crate::commands::*;
use clap::{Parser, Subcommand};

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
        project_name: Option<String>,
        /// The name of the output file
        #[clap(short, long, value_parser)]
        output_name: Option<String>,
    },
    /// Open a place file
    Open {
        /// The path to the place file
        #[clap(short, long, value_parser)]
        file_path: Option<String>,
    },
    /// Builds the project and opens the place file
    Run {
        /// The name of the project to build
        #[clap(short, long, value_parser)]
        project_name: Option<String>,
        /// The name of the output file
        #[clap(short, long, value_parser)]
        output_name: Option<String>,
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
        #[clap(short, long, value_parser)]
        auth: Option<String>,
    },
    /// Manage the datastore
    Datastore(DataStore),
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            Command::Init => init(),
            Command::Devtools => devtools(),
            Command::Build {
                project_name,
                output_name,
            } => Ok(build(&BuildParams {
                project_name,
                output_name,
            })),
            Command::Open { file_path } => Ok(open_place(&OpenPlaceParams { file_path })),
            Command::Run {
                project_name,
                output_name,
            } => {
                open_place(&OpenPlaceParams {
                    file_path: build(&BuildParams {
                        project_name: project_name.clone(),
                        output_name: output_name.clone(),
                    }),
                });
                Ok(None)
            }
            Command::Deploy {
                branch_name,
                api_key,
            } => {
                deploy(&DeployParams {
                    branch_name,
                    api_key,
                })
                .await
            }
            Command::Sync { auth } => img_sync(&SyncParams { auth }),
            Command::Datastore(command) => command.run().await,
        }
    }
}
