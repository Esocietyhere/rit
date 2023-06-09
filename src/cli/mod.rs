mod build;
mod datastore;
mod deploy;
mod import;
mod init;
mod open;
mod refresh;
mod run;
mod send;
mod sync;

use clap::{Parser, Subcommand};

pub use self::build::BuildCommand;
pub use self::datastore::DataStore;
pub use self::deploy::DeployCommand;
pub use self::import::ImportCommand;
pub use self::init::InitCommand;
pub use self::open::OpenCommand;
pub use self::refresh::RefreshCommand;
pub use self::run::RunCommand;
pub use self::send::SendCommand;
pub use self::sync::SyncCommand;

#[derive(Debug, Parser)]
#[clap(name = "Rit", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            Command::Init(command) => command.run(),
            Command::Build(command) => command.run(),
            Command::Open(command) => command.run(),
            Command::Run(command) => command.run(),
            Command::Sync(command) => command.run(),
            Command::Send(command) => command.run().await,
            Command::Deploy(command) => command.run().await,
            Command::Import(command) => command.run(),
            Command::Refresh(command) => command.run(),
            Command::Datastore(command) => command.run().await,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Init(InitCommand),
    Build(BuildCommand),
    Open(OpenCommand),
    Run(RunCommand),
    Sync(SyncCommand),
    Send(SendCommand),
    Deploy(DeployCommand),
    Import(ImportCommand),
    Refresh(RefreshCommand),
    Datastore(DataStore),
}

pub fn getenv(api_key: Option<String>, name: String) -> String {
    match api_key {
        Some(v) => v,
        None => std::env::var(name.clone())
            .unwrap_or_else(|_| panic!("environment variable \"{}\" is not set", name)),
    }
}
