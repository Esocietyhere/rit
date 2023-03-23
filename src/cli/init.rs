use clap::Parser;
use std::process::Command;

/// Initializes a new Rojo project.
#[derive(Debug, Parser)]
pub struct InitCommand;
impl InitCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        Command::new("sh")
            .arg("-c")
            .arg("rojo init && wally init && aftman init")
            .output()
            .expect("failed to execute process");
        Ok(Some("Initialized new Rojo project.".to_string()))
    }
}
