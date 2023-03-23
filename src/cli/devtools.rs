use std::process::Command;

use clap::Parser;

/// Install tarmac, remodel, rojo, wally, selene, and stylua
#[derive(Debug, Parser)]
pub struct DevtoolsCommand;
impl DevtoolsCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        devtools()?;
        Ok(None)
    }
}

fn devtools() -> anyhow::Result<Option<String>> {
    // Trust roblox/tarmac
    Command::new("aftman")
        .arg("trust")
        .arg("roblox/tarmac")
        .output()
        .expect("Failed to execute command");

    // Add roblox/tarmac@0.7.0
    Command::new("aftman")
        .arg("add")
        .arg("roblox/tarmac@0.7.0")
        .output()
        .expect("Failed to execute command");

    // Trust rojo-rbx/remodel
    Command::new("aftman")
        .arg("trust")
        .arg("rojo-rbx/remodel")
        .output()
        .expect("Failed to execute command");

    // Add rojo-rbx/remodel@0.11.0
    Command::new("aftman")
        .arg("add")
        .arg("rojo-rbx/remodel@0.11.0")
        .output()
        .expect("Failed to execute command");

    // Trust rojo-rbx/rojo
    Command::new("aftman")
        .arg("trust")
        .arg("rojo-rbx/rojo")
        .output()
        .expect("Failed to execute command");

    // Add rojo-rbx/rojo@7.2.1
    Command::new("aftman")
        .arg("add")
        .arg("rojo-rbx/rojo@7.2.1")
        .output()
        .expect("Failed to execute command");

    // Trust upliftgames/wally
    Command::new("aftman")
        .arg("trust")
        .arg("upliftgames/wally")
        .output()
        .expect("Failed to execute command");

    // Add upliftgames/wally@0.3.1
    Command::new("aftman")
        .arg("add")
        .arg("upliftgames/wally@0.3.1")
        .output()
        .expect("Failed to execute command");

    // Trust kampfkarren/selene
    Command::new("aftman")
        .arg("trust")
        .arg("kampfkarren/selene")
        .output()
        .expect("Failed to execute command");

    // Add kampfkarren/selene@0.25.0
    Command::new("aftman")
        .arg("add")
        .arg("kampfkarren/selene@0.25.0")
        .output()
        .expect("Failed to execute command");

    // Trust johnnymorganz/stylua
    Command::new("aftman")
        .arg("trust")
        .arg("johnnymorganz/stylua")
        .output()
        .expect("Failed to execute command");

    // Add johnnymorganz/stylua@0.25.0
    Command::new("aftman")
        .arg("add")
        .arg("johnnymorganz/stylua@0.17.0")
        .output()
        .expect("Failed to execute command");

    Ok(Some("Installed devtools.".to_string()))
}
