use std::{path::Path, process::Command};

use clap::Parser;

/// Open a place file in Roblox Studio
#[derive(Debug, Parser)]
pub struct OpenCommand {
    /// The path to the place file
    #[clap(short, long, value_parser)]
    file_path: Option<String>,
}

impl OpenCommand {
    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let output = open_place(self.file_path.clone());
        Ok(output)
    }
}

pub fn open_place(file_path: Option<String>) -> Option<String> {
    let input = file_path.unwrap_or(format!("build/{}.rbxl", "default"));
    let path = Path::new(&input);

    if !path.exists() {
        return Some(format!("File {:?} does not exist!", path));
    }

    let (command, arg) = match std::env::consts::OS {
        "windows" => ("powershell.exe", format!("start {:?}", path)),
        "linux" => ("xdg-open", format!("{:?}", path)),
        "macos" => ("open", format!("{:?}", path)),
        _ => return Some("Unsupported operating system!".to_string()),
    };

    let output = Some(
        Command::new(command)
            .arg(arg)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e)),
    )?
    .unwrap();

    if !output.clone().status.success() {
        return Some(format!("Command failed with code {}", output.status));
    }

    None
}
