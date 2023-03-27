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
        open_place(self.file_path.clone());
        Ok(None)
    }
}

pub fn open_place(file_path: Option<String>) -> Option<String> {
    let input = file_path.unwrap_or(format!("build/{}.rbxl", "default"));
    let path = Path::new(&input);
    if !path.exists() {
        println!("File {} does not exist!", input);
    };

    if cfg!(target_os = "windows") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(r#"start "{}""#, input))
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "wsl") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(r#"start "{}""#, input))
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(r#"xdg-open "{}""#, input))
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg(format!(r#"open "{}""#, input))
            .output()
            .expect("failed to execute process");
    } else {
        println!("Unsupported operating system!");
    }
    Some(input)
}
