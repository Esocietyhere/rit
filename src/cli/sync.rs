use super::getenv;
use anyhow::Ok;
use clap::Parser;
use std::process::Command;

/// Sync images to the Roblox CDN with Tarmac
#[derive(Debug, Parser)]
pub struct SyncCommand {
    /// The authentication token to use
    #[clap(short, long, value_parser)]
    auth: Option<String>,
}

impl SyncCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        img_sync(self.auth.clone())
    }
}

pub fn img_sync(auth: Option<String>) -> anyhow::Result<Option<String>> {
    let auth = getenv(auth.clone(), "ROBLOSECURITY".to_string());
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"tarmac sync --target roblox --auth "{}" --retry 3 --retry-delay 5"#,
            auth,
        ))
        .output()
        .expect("failed to execute process");
    Ok(Some("Synced images to Roblox CDN.".to_string()))
}
