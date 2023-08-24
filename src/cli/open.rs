use crate::color::Color;
use clap::Parser;
use roblox_install::RobloxStudio;
use std::path::Path;
use std::process::{Command, Stdio};

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

fn open_output(file_path: String) -> String {
    format!("{} `{}`", Color::green().pad("Running"), file_path)
}

pub fn open_place(file_path: Option<String>) -> Option<String> {
    let input = file_path.unwrap_or(format!("build/{}.rbxl", "default"));
    let path = Path::new(&input);

    if path.exists() {
        let studio_install =
            RobloxStudio::locate().expect("Could not locate a Roblox Studio installation.");

        let _studio_process = Command::new(studio_install.application_path())
            .arg(format!("{}", path.display()))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        println!("{}", open_output(input.clone()));
    } else {
        return Some(format!("File {:?} does not exist!", path));
    }

    None
}
