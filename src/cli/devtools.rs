use ansi_term::Colour;
use clap::Parser;
use std::process::Command;

/// Install tarmac, remodel, rojo, wally, selene, and stylua
#[derive(Debug, Parser)]
pub struct DevtoolsCommand;

impl DevtoolsCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        devtools()
    }
}

struct Tool {
    pub name: String,
    pub version: String,
}

impl Tool {
    fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }
}

fn devtools() -> anyhow::Result<Option<String>> {
    let tools = vec![
        Tool::new("roblox/tarmac", "0.7.0"),
        Tool::new("rojo-rbx/remodel", "0.11.0"),
        Tool::new("rojo-rbx/rojo", "7.2.1"),
        Tool::new("upliftgames/wally", "0.3.1"),
        Tool::new("kampfkarren/selene", "0.25.0"),
        Tool::new("johnnymorganz/stylua", "0.17.1"),
    ];

    for tool in tools {
        // Trust tool
        Command::new("aftman")
            .arg("trust")
            .arg(tool.name.clone())
            .output()
            .expect("Failed to execute command");

        // Add tool
        Command::new("aftman")
            .arg("add")
            .arg(format!("{}@{}", tool.name, tool.version))
            .output()
            .expect("Failed to execute command");

        println!(
            "{} {} {}",
            Colour::Green.paint("Installed"),
            Colour::Yellow.paint(tool.name),
            Colour::Green.paint(tool.version)
        )
    }

    Ok(Some("Finished installing devtools.".to_string()))
}
